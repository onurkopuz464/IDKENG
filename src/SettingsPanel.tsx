import { useEffect, useRef, useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  LANGUAGES,
  PROVIDERS,
  displayError,
  type Copy,
} from "./i18n";
import {
  codeToVk,
  errorMessage,
  formatHotkeyBinding,
  hotkeyParts,
  isKeySnapshot,
  isSettingsState,
  isTrafficList,
  type ApiKeyInfo,
  type HotkeyBinding,
  type TrafficEntry,
} from "./types";

const DOUBLE_TAP_MS = 500;

function isRuleList(value: unknown): value is string[] {
  return Array.isArray(value) && value.every((rule) => typeof rule === "string");
}

type SettingsPanelProps = {
  open: boolean;
  copy: Copy;
  onClose: () => void;
  onKeysChanged: (keys: ApiKeyInfo[], activeKeyId: string | null) => void;
  onHotkeyChanged: (label: string) => void;
  onLanguageChanged: (language: string) => void;
};

type PendingHotkey = {
  ctrl: boolean;
  alt: boolean;
  shift: boolean;
  win: boolean;
  vk: number;
};

type Tab = "appearance" | "translation" | "api" | "advanced";

export function SettingsPanel({
  open,
  copy,
  onClose,
  onKeysChanged,
  onHotkeyChanged,
  onLanguageChanged,
}: SettingsPanelProps) {
  const [tab, setTab] = useState<Tab>("appearance");
  const [name, setName] = useState("");
  const [apiKey, setApiKey] = useState("");
  const [provider, setProvider] = useState("gemini");
  const [model, setModel] = useState("gemini-3.1-flash-lite");
  const [baseUrl, setBaseUrl] = useState("");
  const [apiKeys, setApiKeys] = useState<ApiKeyInfo[]>([]);
  const [activeKeyId, setActiveKeyId] = useState<string | null>(null);
  const [autostart, setAutostart] = useState(false);
  const [canAutostart, setCanAutostart] = useState(true);
  const [closeToTray, setCloseToTray] = useState(true);
  const [startMinimized, setStartMinimized] = useState(false);
  const [hotkeyLabel, setHotkeyLabel] = useState("Ctrl+C+C");
  const [capturing, setCapturing] = useState(false);
  const [previewLabel, setPreviewLabel] = useState<string | null>(null);
  const [uiLanguage, setUiLanguage] = useState("tr");
  const [nativeLanguage, setNativeLanguage] = useState("tr");
  const [foreignLanguage, setForeignLanguage] = useState("en");
  const [promptRules, setPromptRules] = useState<string[]>([]);
  const rulesCustomizedRef = useRef(false);
  const [timeoutSecs, setTimeoutSecs] = useState(20);
  const [maxOutputTokens, setMaxOutputTokens] = useState(2048);
  const [fallbackModels, setFallbackModels] = useState(true);
  const [traffic, setTraffic] = useState<TrafficEntry[]>([]);
  const [status, setStatus] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const captureButtonRef = useRef<HTMLButtonElement>(null);
  const pendingRef = useRef<PendingHotkey | null>(null);
  const pendingAtRef = useRef(0);
  const timerRef = useRef(0);

  useEffect(() => {
    if (!open) {
      return;
    }
    void refresh();
  }, [open]);

  useEffect(() => {
    if (!open) {
      return;
    }
    let cancelled = false;
    void invoke<unknown>("preview_prompt_rules", {
      uiLanguage,
      nativeLanguage,
      foreignLanguage,
    }).then((value) => {
      if (cancelled || !isRuleList(value) || value.length === 0) {
        return;
      }
      const head = value[0];
      if (rulesCustomizedRef.current) {
        setPromptRules((rules) => [head, ...rules.slice(1)]);
        return;
      }
      setPromptRules(value);
    });
    return () => {
      cancelled = true;
    };
  }, [open, uiLanguage, nativeLanguage, foreignLanguage]);

  useEffect(() => {
    if (!open || !capturing) {
      return;
    }

    function onKeyDown(event: KeyboardEvent) {
      if (event.repeat) {
        return;
      }
      if (event.key === "Escape") {
        event.preventDefault();
        event.stopPropagation();
        stopCapture();
        return;
      }
      const vk = codeToVk(event.code);
      if (vk === null) {
        return;
      }
      event.preventDefault();
      event.stopPropagation();

      const next: PendingHotkey = {
        ctrl: event.ctrlKey,
        alt: event.altKey,
        shift: event.shiftKey,
        win: event.metaKey,
        vk,
      };
      const pending = pendingRef.current;
      const now = Date.now();
      if (
        pending &&
        pending.vk === next.vk &&
        pending.ctrl === next.ctrl &&
        pending.alt === next.alt &&
        pending.shift === next.shift &&
        pending.win === next.win &&
        now - pendingAtRef.current <= DOUBLE_TAP_MS
      ) {
        void saveHotkey({ ...next, taps: 2 });
        return;
      }

      pendingRef.current = next;
      pendingAtRef.current = now;
      setPreviewLabel(formatHotkeyBinding({ ...next, taps: 1 }));
      window.clearTimeout(timerRef.current);
      timerRef.current = window.setTimeout(() => {
        const current = pendingRef.current;
        if (!current) {
          return;
        }
        void saveHotkey({ ...current, taps: 1 });
      }, DOUBLE_TAP_MS);
    }

    window.addEventListener("keydown", onKeyDown, true);
    captureButtonRef.current?.focus();
    return () => {
      window.removeEventListener("keydown", onKeyDown, true);
    };
  }, [open, capturing]);

  useEffect(() => {
    if (!open) {
      stopCapture();
    }
  }, [open]);

  function stopCapture() {
    window.clearTimeout(timerRef.current);
    timerRef.current = 0;
    pendingRef.current = null;
    pendingAtRef.current = 0;
    setPreviewLabel(null);
    setCapturing(false);
  }

  function editRule(index: number, text: string) {
    if (index === 0) {
      return;
    }
    rulesCustomizedRef.current = true;
    setPromptRules((rules) => rules.map((rule, ruleIndex) => (ruleIndex === index ? text : rule)));
  }

  function removeRule(index: number) {
    if (index === 0) {
      return;
    }
    rulesCustomizedRef.current = true;
    setPromptRules((rules) => rules.filter((_, ruleIndex) => ruleIndex !== index));
  }

  function addRule() {
    rulesCustomizedRef.current = true;
    setPromptRules((rules) => [...rules, ""]);
  }

  async function resetRules() {
    rulesCustomizedRef.current = false;
    const value = await invoke<unknown>("preview_prompt_rules", {
      uiLanguage,
      nativeLanguage,
      foreignLanguage,
    });
    if (isRuleList(value)) {
      setPromptRules(value);
    }
  }

  function fail(caught: unknown) {
    setError(displayError(errorMessage(caught), copy));
  }

  async function refresh() {
    const value = await invoke<unknown>("get_settings_state");
    if (!isSettingsState(value)) {
      return;
    }
    setApiKeys(value.apiKeys);
    setActiveKeyId(value.activeKeyId);
    setAutostart(value.autostart);
    setCanAutostart(value.canAutostart);
    setCloseToTray(value.closeToTray);
    setStartMinimized(value.startMinimized);
    setHotkeyLabel(value.hotkeyLabel);
    setUiLanguage(value.uiLanguage);
    setNativeLanguage(value.nativeLanguage);
    setForeignLanguage(value.foreignLanguage);
    rulesCustomizedRef.current = value.rulesCustomized;
    setPromptRules(value.promptRules);
    setTimeoutSecs(value.timeoutSecs);
    setMaxOutputTokens(value.maxOutputTokens);
    setFallbackModels(value.fallbackModels);
    onKeysChanged(value.apiKeys, value.activeKeyId);
    onHotkeyChanged(value.hotkeyLabel);
    onLanguageChanged(value.uiLanguage);
    const log = await invoke<unknown>("get_traffic");
    if (isTrafficList(log)) {
      setTraffic(log);
    }
  }

  function applySnapshot(value: unknown) {
    if (!isKeySnapshot(value)) {
      return;
    }
    setApiKeys(value.apiKeys);
    setActiveKeyId(value.activeKeyId);
    onKeysChanged(value.apiKeys, value.activeKeyId);
  }

  async function savePrefs() {
    setError(null);
    setStatus(null);
    setSaving(true);
    try {
      const head = promptRules[0] ?? "";
      const tail = promptRules
        .slice(1)
        .map((rule) => rule.trim())
        .filter((rule) => rule.length > 0);
      const customized = rulesCustomizedRef.current && tail.length > 0;
      rulesCustomizedRef.current = customized;
      await invoke("set_app_prefs", {
        update: {
          uiLanguage,
          nativeLanguage,
          foreignLanguage,
          promptRules: tail,
          rulesCustomized: customized,
          timeoutSecs,
          maxOutputTokens,
          fallbackModels,
        },
      });
      if (customized) {
        setPromptRules([head, ...tail]);
      } else {
        const defaults = await invoke<unknown>("preview_prompt_rules", {
          uiLanguage,
          nativeLanguage,
          foreignLanguage,
        });
        if (isRuleList(defaults)) {
          setPromptRules(defaults);
        }
      }
      onLanguageChanged(uiLanguage);
      setStatus(copy.saved);
    } catch (caught) {
      fail(caught);
    } finally {
      setSaving(false);
    }
  }

  async function saveHotkey(hotkey: HotkeyBinding) {
    window.clearTimeout(timerRef.current);
    timerRef.current = 0;
    pendingRef.current = null;
    setCapturing(false);
    setPreviewLabel(null);
    setError(null);
    setStatus(null);
    try {
      const label = await invoke<unknown>("set_hotkey_binding", { hotkey });
      if (typeof label !== "string") {
        setError(copy.errFailed);
        return;
      }
      setHotkeyLabel(label);
      onHotkeyChanged(label);
      setStatus(copy.savedHotkey);
    } catch (caught) {
      fail(caught);
    }
  }

  async function addKey(event: FormEvent) {
    event.preventDefault();
    setError(null);
    setStatus(null);
    setSaving(true);
    try {
      const snapshot = await invoke<unknown>("add_api_key", {
        name,
        apiKey,
        provider,
        model,
        baseUrl,
      });
      applySnapshot(snapshot);
      setName("");
      setApiKey("");
      setStatus(copy.added);
    } catch (caught) {
      fail(caught);
    } finally {
      setSaving(false);
    }
  }

  async function selectKey(id: string) {
    setError(null);
    try {
      const snapshot = await invoke<unknown>("select_api_key", { id });
      applySnapshot(snapshot);
    } catch (caught) {
      fail(caught);
    }
  }

  async function removeKey(id: string) {
    setError(null);
    try {
      const snapshot = await invoke<unknown>("remove_api_key", { id });
      applySnapshot(snapshot);
    } catch (caught) {
      fail(caught);
    }
  }

  function toggleCapture() {
    setError(null);
    setStatus(null);
    if (capturing) {
      stopCapture();
      return;
    }
    pendingRef.current = null;
    pendingAtRef.current = 0;
    setPreviewLabel(null);
    setCapturing(true);
  }

  async function toggleAutostart(enabled: boolean) {
    setError(null);
    try {
      await invoke("set_autostart", { enabled });
      setAutostart(enabled);
    } catch (caught) {
      fail(caught);
    }
  }

  async function toggleCloseToTray(enabled: boolean) {
    setError(null);
    try {
      await invoke("set_close_to_tray", { enabled });
      setCloseToTray(enabled);
    } catch (caught) {
      fail(caught);
    }
  }

  async function toggleStartMinimized(enabled: boolean) {
    setError(null);
    try {
      await invoke("set_start_minimized", { enabled });
      setStartMinimized(enabled);
    } catch (caught) {
      fail(caught);
    }
  }

  async function clearLog() {
    setError(null);
    try {
      await invoke("clear_traffic");
      setTraffic([]);
    } catch (caught) {
      fail(caught);
    }
  }

  function onProvider(next: string) {
    setProvider(next);
    const found = PROVIDERS.find((item) => item.id === next);
    const first = found?.models[0];
    if (first) {
      setModel(first);
    } else {
      setModel("");
    }
  }

  if (!open) {
    return null;
  }

  const displayedHotkey = capturing ? previewLabel : hotkeyLabel;
  const modelChoices = PROVIDERS.find((item) => item.id === provider)?.models ?? [];

  return (
    <div className="modal-backdrop" onClick={onClose}>
      <section
        className="modal"
        role="dialog"
        aria-labelledby="settings-title"
        onClick={(event) => event.stopPropagation()}
      >
        <header className="modal-head">
          <h2 id="settings-title">{copy.settings}</h2>
          <button type="button" className="ghost" onClick={onClose}>
            {copy.close}
          </button>
        </header>

        <div className="tabs" role="tablist">
          {(["appearance", "translation", "api", "advanced"] as const).map((item) => (
            <button
              key={item}
              type="button"
              className={tab === item ? "tab active" : "tab"}
              onClick={() => setTab(item)}
            >
              {item === "appearance"
                ? copy.appearance
                : item === "translation"
                  ? copy.translationTab
                  : item === "api"
                    ? copy.api
                    : copy.advanced}
            </button>
          ))}
        </div>

        {tab === "appearance" ? (
          <>
            <label htmlFor="ui-language">{copy.uiLanguage}</label>
            <select
              id="ui-language"
              className="key-select"
              value={uiLanguage}
              onChange={(event) => {
                setUiLanguage(event.currentTarget.value);
                onLanguageChanged(event.currentTarget.value);
              }}
            >
              {LANGUAGES.map((language) => (
                <option key={language.code} value={language.code}>
                  {language.label}
                </option>
              ))}
            </select>

            <p className="lede">{copy.hotkey}</p>
            <div className="hotkey-row">
              <div className="hotkey-combo" aria-live="polite">
                {capturing && !previewLabel ? (
                  <p className="hotkey-listening">{copy.pressKeys}</p>
                ) : displayedHotkey ? (
                  hotkeyParts(displayedHotkey).map((part, index) => (
                    <span key={`${part}-${index}`}>
                      {index > 0 ? " + " : null}
                      <kbd>{part}</kbd>
                    </span>
                  ))
                ) : null}
              </div>
              <button
                ref={captureButtonRef}
                type="button"
                className={capturing ? "ghost" : "primary"}
                onClick={toggleCapture}
              >
                {capturing ? copy.cancel : copy.changeHotkey}
              </button>
            </div>
            <p className="hotkey-hint">{capturing && previewLabel ? copy.doubleAgain : copy.escCancel}</p>
          </>
        ) : null}

        {tab === "translation" ? (
          <>
            <label htmlFor="native-language">{copy.nativeLanguage}</label>
            <select
              id="native-language"
              className="key-select"
              value={nativeLanguage}
              onChange={(event) => {
                const next = event.currentTarget.value;
                setNativeLanguage(next);
                if (next === foreignLanguage) {
                  const fallback = LANGUAGES.find((language) => language.code !== next);
                  if (fallback) {
                    setForeignLanguage(fallback.code);
                  }
                }
              }}
            >
              {LANGUAGES.map((language) => (
                <option key={language.code} value={language.code}>
                  {language.label}
                </option>
              ))}
            </select>

            <p className="lede">{copy.prompt}</p>
            <ol className="rule-list">
              <li className="main-rule">
                <span className="rule-index">1.</span>
                <div className="main-rule-body">
                  <p>{promptRules[0] ?? ""}</p>
                  <select
                    className="key-select"
                    aria-label={copy.prompt}
                    value={foreignLanguage}
                    onChange={(event) => setForeignLanguage(event.currentTarget.value)}
                  >
                    {LANGUAGES.filter((language) => language.code !== nativeLanguage).map((language) => (
                      <option key={language.code} value={language.code}>
                        {language.label}
                      </option>
                    ))}
                  </select>
                </div>
              </li>
              {promptRules.slice(1).map((rule, index) => {
                const ruleIndex = index + 1;
                return (
                  <li key={ruleIndex}>
                    <span className="rule-index">{ruleIndex + 1}.</span>
                    <textarea
                      className="prompt-box"
                      value={rule}
                      aria-label={`${copy.prompt} ${ruleIndex + 1}`}
                      onChange={(event) => editRule(ruleIndex, event.currentTarget.value)}
                    />
                    <button type="button" className="ghost" onClick={() => removeRule(ruleIndex)}>
                      {copy.removeRule}
                    </button>
                  </li>
                );
              })}
            </ol>
            <div className="rule-actions">
              <button type="button" onClick={addRule}>
                {copy.addRule}
              </button>
              <button type="button" className="ghost" onClick={() => void resetRules()}>
                {copy.resetPrompt}
              </button>
            </div>
            <p className="hotkey-hint">{copy.promptHint}</p>

            <div className="section-head">
              <p className="lede">{copy.traffic}</p>
              <button type="button" className="ghost" onClick={() => void clearLog()}>
                {copy.clearTraffic}
              </button>
            </div>
            {traffic.length === 0 ? (
              <p className="fine">{copy.noTraffic}</p>
            ) : (
              <ul className="traffic-list">
                {traffic.map((entry) => (
                  <li key={entry.id}>
                    <details>
                      <summary>
                        {entry.profileName} · {entry.model} · {entry.elapsedMs}
                        {copy.elapsed}
                        {entry.error ? " · !" : ""}
                      </summary>
                      <p className="fine">{entry.source}</p>
                      <p className="lede">{copy.request}</p>
                      <pre>{entry.systemPrompt}</pre>
                      <pre>{entry.userMessage}</pre>
                      <p className="lede">{copy.response}</p>
                      <pre>{entry.error || entry.response}</pre>
                    </details>
                  </li>
                ))}
              </ul>
            )}
          </>
        ) : null}

        {tab === "api" ? (
          <>
            {apiKeys.length === 0 ? (
              <p className="fine">{copy.noKeys}</p>
            ) : (
              <ul className="key-list">
                {apiKeys.map((entry) => (
                  <li key={entry.id} className={entry.id === activeKeyId ? "active" : ""}>
                    <label>
                      <input
                        type="radio"
                        name="active-key"
                        checked={entry.id === activeKeyId}
                        onChange={() => void selectKey(entry.id)}
                      />
                      <span>
                        {entry.name}
                        <small>
                          {entry.provider} · {entry.model}
                        </small>
                      </span>
                    </label>
                    <button type="button" className="ghost" onClick={() => void removeKey(entry.id)}>
                      {copy.remove}
                    </button>
                  </li>
                ))}
              </ul>
            )}

            <form onSubmit={(event) => void addKey(event)}>
              <label htmlFor="provider">{copy.provider}</label>
              <select
                id="provider"
                className="key-select"
                value={provider}
                onChange={(event) => onProvider(event.currentTarget.value)}
              >
                {PROVIDERS.map((item) => (
                  <option key={item.id} value={item.id}>
                    {item.id === "custom" ? copy.custom : item.id}
                  </option>
                ))}
              </select>
              <label htmlFor="model">{copy.model}</label>
              <input
                id="model"
                type="text"
                list="model-choices"
                value={model}
                onChange={(event) => setModel(event.currentTarget.value)}
              />
              <datalist id="model-choices">
                {modelChoices.map((item) => (
                  <option key={item} value={item} />
                ))}
              </datalist>
              {provider === "custom" ? (
                <>
                  <label htmlFor="base-url">{copy.baseUrl}</label>
                  <input
                    id="base-url"
                    type="text"
                    placeholder="https://api.example.com/v1"
                    value={baseUrl}
                    onChange={(event) => setBaseUrl(event.currentTarget.value)}
                  />
                  <p className="hotkey-hint">{copy.baseUrlHint}</p>
                </>
              ) : null}
              <label htmlFor="key-name">{copy.newProfile}</label>
              <input
                id="key-name"
                type="text"
                autoComplete="off"
                placeholder={copy.profileName}
                value={name}
                onChange={(event) => setName(event.currentTarget.value)}
              />
              <input
                id="api-key"
                type="password"
                autoComplete="off"
                spellCheck={false}
                placeholder="API key"
                value={apiKey}
                onChange={(event) => setApiKey(event.currentTarget.value)}
              />
              <button type="submit" className="primary" disabled={saving}>
                {saving ? copy.adding : copy.add}
              </button>
            </form>
          </>
        ) : null}

        {tab === "advanced" ? (
          <>
            <label htmlFor="timeout">{copy.timeout}</label>
            <input
              id="timeout"
              type="number"
              min={5}
              max={120}
              value={timeoutSecs}
              onChange={(event) => setTimeoutSecs(Number(event.currentTarget.value))}
            />
            <label htmlFor="tokens">{copy.maxTokens}</label>
            <input
              id="tokens"
              type="number"
              min={128}
              max={8192}
              step={64}
              value={maxOutputTokens}
              onChange={(event) => setMaxOutputTokens(Number(event.currentTarget.value))}
            />
            <label className="check">
              <input
                type="checkbox"
                checked={fallbackModels}
                onChange={(event) => setFallbackModels(event.currentTarget.checked)}
              />
              {copy.fallback}
            </label>
            <label className="check">
              <input
                type="checkbox"
                checked={autostart}
                onChange={(event) => void toggleAutostart(event.currentTarget.checked)}
                disabled={!canAutostart}
              />
              {copy.autostart}
            </label>
            {!canAutostart ? <p className="hotkey-hint">{copy.devAutostart}</p> : null}
            <label className="check">
              <input
                type="checkbox"
                checked={closeToTray}
                onChange={(event) => void toggleCloseToTray(event.currentTarget.checked)}
              />
              {copy.closeToTray}
            </label>
            <label className="check">
              <input
                type="checkbox"
                checked={startMinimized}
                onChange={(event) => void toggleStartMinimized(event.currentTarget.checked)}
              />
              {copy.startMinimized}
            </label>
          </>
        ) : null}

        {tab !== "api" ? (
          <button type="button" className="primary" disabled={saving} onClick={() => void savePrefs()}>
            {copy.save}
          </button>
        ) : null}
        {status ? <p className="ok">{status}</p> : null}
        {error ? <p className="error">{error}</p> : null}
      </section>
    </div>
  );
}
