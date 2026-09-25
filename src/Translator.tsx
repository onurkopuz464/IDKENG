import { useEffect, useState, type KeyboardEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  errorMessage,
  hotkeyParts,
  isSettingsState,
  isTranslationState,
  type ApiKeyInfo,
} from "./types";
import { copyFor, displayError, isRtl } from "./i18n";
import { SettingsPanel } from "./SettingsPanel";

export function Translator() {
  const [source, setSource] = useState("");
  const [translation, setTranslation] = useState("");
  const [status, setStatus] = useState<"idle" | "loading" | "error">("idle");
  const [error, setError] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [apiKeys, setApiKeys] = useState<ApiKeyInfo[]>([]);
  const [activeKeyId, setActiveKeyId] = useState<string | null>(null);
  const [hotkeyLabel, setHotkeyLabel] = useState("Ctrl+C+C");
  const [uiLanguage, setUiLanguage] = useState("tr");

  useEffect(() => {
    let cancelled = false;
    let unlistenState: (() => void) | undefined;
    let unlistenSettings: (() => void) | undefined;

    void listen<unknown>("translation-state", (event) => {
      if (cancelled || !isTranslationState(event.payload)) {
        return;
      }
      const payload = event.payload;
      setCopied(false);
      switch (payload.kind) {
        case "loading":
          setSource(payload.source);
          setStatus("loading");
          setError(null);
          break;
        case "ready":
          setSource(payload.source);
          setTranslation(payload.translation);
          setStatus("idle");
          setError(null);
          break;
        case "error":
          if (payload.source) {
            setSource(payload.source);
          }
          setStatus("error");
          setError(payload.message);
          break;
        case "idle":
          setStatus("idle");
          break;
        default: {
          const _exhaustive: never = payload;
          return _exhaustive;
        }
      }
    }).then((fn) => {
      if (cancelled) {
        fn();
        return;
      }
      unlistenState = fn;
    });

    void listen("open-settings", () => {
      if (!cancelled) {
        setSettingsOpen(true);
      }
    }).then((fn) => {
      if (cancelled) {
        fn();
        return;
      }
      unlistenSettings = fn;
    });

    void invoke<unknown>("get_settings_state").then((value) => {
      if (cancelled || !isSettingsState(value)) {
        return;
      }
      setApiKeys(value.apiKeys);
      setActiveKeyId(value.activeKeyId);
      setHotkeyLabel(value.hotkeyLabel);
      setUiLanguage(value.uiLanguage);
    });

    return () => {
      cancelled = true;
      unlistenState?.();
      unlistenSettings?.();
    };
  }, []);

  async function selectKey(id: string) {
    try {
      await invoke("select_api_key", { id });
      setActiveKeyId(id);
    } catch (caught) {
      setError(errorMessage(caught));
    }
  }

  async function translate() {
    if (!source.trim()) {
      return;
    }
    setStatus("loading");
    setError(null);
    try {
      await invoke("translate_text", { source });
    } catch (caught) {
      setStatus("error");
      setError(errorMessage(caught));
    }
  }

  async function copyResult() {
    await invoke("copy_translation");
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1500);
  }

  function onSourceKey(event: KeyboardEvent<HTMLTextAreaElement>) {
    if (event.key === "Enter" && (event.ctrlKey || event.metaKey)) {
      event.preventDefault();
      void translate();
    }
  }

  const copy = copyFor(uiLanguage);
  const busy = status === "loading";
  const canTranslate = source.trim().length > 0 && !busy;
  const canCopy = translation.length > 0 && !busy;
  const shownError = error ? displayError(error, copy) : null;

  useEffect(() => {
    document.documentElement.lang = uiLanguage;
    document.documentElement.dir = isRtl(uiLanguage) ? "rtl" : "ltr";
  }, [uiLanguage]);

  return (
    <div className="app">
      <header className="topbar">
        <span className="brand">IDKENG</span>
        <div className="topbar-actions">
          {apiKeys.length > 0 ? (
            <select
              className="key-select"
              value={activeKeyId ?? ""}
              onChange={(event) => void selectKey(event.currentTarget.value)}
              aria-label={copy.activeKey}
            >
              {apiKeys.map((entry) => (
                <option key={entry.id} value={entry.id}>
                  {entry.name}
                </option>
              ))}
            </select>
          ) : null}
          <button type="button" className="ghost" onClick={() => setSettingsOpen(true)}>
            {copy.settings}
          </button>
        </div>
      </header>

      <div className="split">
        <section className="pane pane-source">
          <textarea
            value={source}
            onChange={(event) => setSource(event.currentTarget.value)}
            onKeyDown={onSourceKey}
            placeholder={copy.placeholder}
            spellCheck={false}
          />
          <div className="pane-actions">
            {source.length === 0 ? (
              <p className="hint">
                {copy.shortcutPrefix}{" "}
                {hotkeyParts(hotkeyLabel).map((part, index) => (
                  <span key={`${part}-${index}`}>
                    {index > 0 ? " + " : null}
                    <kbd>{part}</kbd>
                  </span>
                ))}
              </p>
            ) : (
              <span />
            )}
            <button
              type="button"
              className="primary"
              disabled={!canTranslate}
              onClick={() => void translate()}
            >
              {busy ? copy.translating : copy.translate}
            </button>
          </div>
        </section>

        <section className="pane pane-target">
          {busy ? <div className="skeleton" aria-label={copy.translating} /> : null}
          {!busy && shownError ? <p className="error">{shownError}</p> : null}
          {!busy && !error && translation ? (
            <p className="output">{translation}</p>
          ) : null}
          {!busy && !error && !translation ? (
            <p className="empty">{copy.emptyTranslation}</p>
          ) : null}
          <div className="pane-actions">
            <button type="button" disabled={!canCopy} onClick={() => void copyResult()}>
              {copied ? copy.copied : copy.copy}
            </button>
          </div>
        </section>
      </div>

      <SettingsPanel
        open={settingsOpen}
        copy={copy}
        onClose={() => setSettingsOpen(false)}
        onKeysChanged={(keys, id) => {
          setApiKeys(keys);
          setActiveKeyId(id);
        }}
        onHotkeyChanged={setHotkeyLabel}
        onLanguageChanged={setUiLanguage}
      />
    </div>
  );
}
