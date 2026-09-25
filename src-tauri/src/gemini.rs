use std::time::{Duration, Instant};

use serde_json::json;

use crate::settings::{self, ActiveProfile, TrafficEntry};

const GEMINI_FALLBACK: &[&str] = &[
    "gemini-3.1-flash-lite",
    "gemini-3.5-flash",
    "gemini-3-flash-preview",
];

pub async fn translate(source: &str) -> Result<String, String> {
    let profile = settings::active_profile()?;
    let (system_prompt, user_message) = settings::prompts_for(source);
    let timeout = Duration::from_secs(settings::timeout_secs());
    let max_tokens = settings::max_output_tokens();
    let client = reqwest::Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|e| e.to_string())?;

    let mut last_error = "err:failed".to_string();
    for model in models_for(&profile) {
        let started = Instant::now();
        match request(&client, &profile, &model, &system_prompt, &user_message, max_tokens).await {
            Ok(text) => {
                log_attempt(
                    &profile,
                    &model,
                    source,
                    &system_prompt,
                    &user_message,
                    &text,
                    "",
                    started.elapsed().as_millis() as u64,
                );
                return Ok(text);
            }
            Err(AttemptError::Stop(message)) => {
                log_attempt(
                    &profile,
                    &model,
                    source,
                    &system_prompt,
                    &user_message,
                    "",
                    &message,
                    started.elapsed().as_millis() as u64,
                );
                return Err(message);
            }
            Err(AttemptError::Next(message)) => {
                log_attempt(
                    &profile,
                    &model,
                    source,
                    &system_prompt,
                    &user_message,
                    "",
                    &message,
                    started.elapsed().as_millis() as u64,
                );
                last_error = message;
            }
        }
    }
    Err(last_error)
}

fn models_for(profile: &ActiveProfile) -> Vec<String> {
    let mut models = vec![profile.model.clone()];
    if profile.provider == "gemini" && profile.fallback_models {
        for model in GEMINI_FALLBACK {
            if !models.iter().any(|item| item == model) {
                models.push((*model).to_string());
            }
        }
    }
    models
}

fn log_attempt(
    profile: &ActiveProfile,
    model: &str,
    source: &str,
    system_prompt: &str,
    user_message: &str,
    response: &str,
    error: &str,
    elapsed_ms: u64,
) {
    settings::record_traffic(TrafficEntry {
        id: format!("{elapsed_ms}-{}", model.len()),
        at_ms: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0),
        provider: profile.provider.clone(),
        model: model.to_string(),
        profile_name: profile.name.clone(),
        elapsed_ms,
        source: source.to_string(),
        system_prompt: system_prompt.to_string(),
        user_message: user_message.to_string(),
        response: response.to_string(),
        error: error.to_string(),
    });
}

enum AttemptError {
    Next(String),
    Stop(String),
}

async fn request(
    client: &reqwest::Client,
    profile: &ActiveProfile,
    model: &str,
    system_prompt: &str,
    user_message: &str,
    max_tokens: u32,
) -> Result<String, AttemptError> {
    if profile.provider == "gemini" {
        gemini_request(client, &profile.key, model, system_prompt, user_message, max_tokens).await
    } else {
        openai_request(
            client,
            &profile.base_url,
            &profile.key,
            model,
            system_prompt,
            user_message,
            max_tokens,
        )
        .await
    }
}

async fn gemini_request(
    client: &reqwest::Client,
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_message: &str,
    max_tokens: u32,
) -> Result<String, AttemptError> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"
    );
    let body = json!({
        "systemInstruction": { "parts": [{ "text": system_prompt }] },
        "contents": [{ "role": "user", "parts": [{ "text": user_message }] }],
        "generationConfig": {
            "maxOutputTokens": max_tokens,
            "thinkingConfig": { "thinkingLevel": "MINIMAL" }
        }
    });
    let response = match client.post(url).query(&[("key", api_key)]).json(&body).send().await {
        Ok(response) => response,
        Err(error) => return Err(AttemptError::Next(send_error(&error))),
    };
    let status = response.status();
    let value: serde_json::Value = response
        .json()
        .await
        .map_err(|_| AttemptError::Next("err:unreadable".into()))?;
    if !status.is_success() {
        return Err(status_error(status.as_u16(), &value));
    }
    extract_gemini(&value)
        .map(|text| clean_output(&text))
        .map_err(AttemptError::Next)
}

async fn openai_request(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_message: &str,
    max_tokens: u32,
) -> Result<String, AttemptError> {
    if base_url.is_empty() {
        return Err(AttemptError::Stop("err:base_url".into()));
    }
    let url = format!("{base_url}/chat/completions");
    let body = json!({
        "model": model,
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": user_message }
        ],
        "max_tokens": max_tokens
    });
    let response = match client
        .post(url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => return Err(AttemptError::Next(send_error(&error))),
    };
    let status = response.status();
    let value: serde_json::Value = response
        .json()
        .await
        .map_err(|_| AttemptError::Next("err:unreadable".into()))?;
    if !status.is_success() {
        return Err(status_error(status.as_u16(), &value));
    }
    let text = value
        .pointer("/choices/0/message/content")
        .and_then(|item| item.as_str())
        .or_else(|| {
            value
                .pointer("/choices/0/message/content/0/text")
                .and_then(|item| item.as_str())
        })
        .unwrap_or("")
        .trim()
        .to_string();
    if text.is_empty() {
        Err(AttemptError::Next("err:empty".into()))
    } else {
        Ok(clean_output(&text))
    }
}

fn send_error(error: &reqwest::Error) -> String {
    if error.is_timeout() {
        "err:timeout".into()
    } else {
        "err:network".into()
    }
}

fn status_error(status: u16, value: &serde_json::Value) -> AttemptError {
    if status == 401 || status == 403 {
        return AttemptError::Stop("err:invalid_key".into());
    }
    let raw = value
        .pointer("/error/message")
        .and_then(|item| item.as_str())
        .unwrap_or("err:failed");
    let lower = raw.to_ascii_lowercase();
    let message = if status == 429 {
        "err:quota".to_string()
    } else if lower.contains("high demand") || lower.contains("try again later") {
        "err:busy".to_string()
    } else if raw.starts_with("err:") {
        raw.to_string()
    } else {
        raw.to_string()
    };
    if status == 429 || status == 500 || status == 502 || status == 503 || status == 504 {
        AttemptError::Next(message)
    } else {
        AttemptError::Stop(message)
    }
}

fn extract_gemini(value: &serde_json::Value) -> Result<String, String> {
    let parts = value
        .pointer("/candidates/0/content/parts")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "err:empty".to_string())?;

    let mut texts = Vec::new();
    for part in parts {
        if part.get("thought").and_then(|v| v.as_bool()).unwrap_or(false) {
            continue;
        }
        if let Some(text) = part.get("text").and_then(|v| v.as_str()) {
            if !text.trim().is_empty() {
                texts.push(text);
            }
        }
    }

    let joined = texts.join("").trim().to_string();
    if joined.is_empty() {
        Err("err:empty".to_string())
    } else {
        Ok(joined)
    }
}

fn clean_output(raw: &str) -> String {
    let mut text = raw.trim().to_string();

    if text.starts_with("```") {
        if let Some(first_nl) = text.find('\n') {
            text = text[first_nl + 1..].to_string();
        }
        if let Some(stripped) = text.strip_suffix("```") {
            text = stripped.trim().to_string();
        }
    }

    let chars: Vec<char> = text.chars().collect();
    if chars.len() >= 2 {
        let first = chars[0];
        let last = chars[chars.len() - 1];
        let wrapped = (first == '"' && last == '"')
            || (first == '\u{201C}' && last == '\u{201D}');
        if wrapped {
            text = chars[1..chars.len() - 1].iter().collect::<String>();
            text = text.trim().to_string();
        }
    }

    text
}
