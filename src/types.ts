function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

export function errorMessage(caught: unknown): string {
  if (typeof caught === "string") {
    return caught;
  }
  if (caught instanceof Error) {
    return caught.message;
  }
  if (isRecord(caught) && typeof caught.message === "string") {
    return caught.message;
  }
  return "İşlem başarısız.";
}

export type TranslationState =
  | { kind: "idle" }
  | { kind: "loading"; source: string }
  | { kind: "ready"; source: string; translation: string }
  | { kind: "error"; source?: string; message: string };

export function isTranslationState(value: unknown): value is TranslationState {
  if (!isRecord(value) || typeof value.kind !== "string") {
    return false;
  }

  switch (value.kind) {
    case "idle":
      return true;
    case "loading":
      return typeof value.source === "string";
    case "ready":
      return typeof value.source === "string" && typeof value.translation === "string";
    case "error":
      return (
        typeof value.message === "string" &&
        (value.source === undefined ||
          value.source === null ||
          typeof value.source === "string")
      );
    default:
      return false;
  }
}

export type ApiKeyInfo = {
  id: string;
  name: string;
  provider: string;
  model: string;
};

export type SettingsState = {
  hasApiKey: boolean;
  activeKeyId: string | null;
  apiKeys: ApiKeyInfo[];
  autostart: boolean;
  closeToTray: boolean;
  startMinimized: boolean;
  hotkeyLabel: string;
  canAutostart: boolean;
  uiLanguage: string;
  nativeLanguage: string;
  foreignLanguage: string;
  promptRules: string[];
  rulesCustomized: boolean;
  timeoutSecs: number;
  maxOutputTokens: number;
  fallbackModels: boolean;
};

export function hotkeyParts(label: string): string[] {
  return label.split("+").filter((part) => part.length > 0);
}

export type HotkeyBinding = {
  ctrl: boolean;
  alt: boolean;
  shift: boolean;
  win: boolean;
  vk: number;
  taps: number;
};

function isModifierCode(code: string): boolean {
  return (
    code === "ControlLeft" ||
    code === "ControlRight" ||
    code === "ShiftLeft" ||
    code === "ShiftRight" ||
    code === "AltLeft" ||
    code === "AltRight" ||
    code === "MetaLeft" ||
    code === "MetaRight"
  );
}

export function codeToVk(code: string): number | null {
  if (isModifierCode(code)) {
    return null;
  }
  if (code.length === 4 && code.startsWith("Key")) {
    const letter = code.charCodeAt(3);
    if (letter >= 65 && letter <= 90) {
      return letter;
    }
  }
  if (code.length === 6 && code.startsWith("Digit")) {
    const digit = code.charCodeAt(5);
    if (digit >= 48 && digit <= 57) {
      return digit;
    }
  }
  if (code.length >= 2 && code.startsWith("F")) {
    let n = 0;
    for (let i = 1; i < code.length; i += 1) {
      const digit = code.charCodeAt(i) - 48;
      if (digit < 0 || digit > 9) {
        n = 0;
        break;
      }
      n = n * 10 + digit;
    }
    if (n >= 1 && n <= 24) {
      return 0x6f + n;
    }
  }
  if (code.length === 7 && code.startsWith("Numpad")) {
    const digit = code.charCodeAt(6);
    if (digit >= 48 && digit <= 57) {
      return 0x60 + (digit - 48);
    }
  }
  switch (code) {
    case "Space":
      return 0x20;
    case "Tab":
      return 0x09;
    case "Enter":
      return 0x0d;
    case "Backspace":
      return 0x08;
    case "Delete":
      return 0x2e;
    case "Insert":
      return 0x2d;
    case "Home":
      return 0x24;
    case "End":
      return 0x23;
    case "PageUp":
      return 0x21;
    case "PageDown":
      return 0x22;
    case "ArrowLeft":
      return 0x25;
    case "ArrowUp":
      return 0x26;
    case "ArrowRight":
      return 0x27;
    case "ArrowDown":
      return 0x28;
    case "Minus":
      return 0xbd;
    case "Equal":
      return 0xbb;
    case "BracketLeft":
      return 0xdb;
    case "BracketRight":
      return 0xdd;
    case "Backslash":
      return 0xdc;
    case "Semicolon":
      return 0xba;
    case "Quote":
      return 0xde;
    case "Backquote":
      return 0xc0;
    case "Comma":
      return 0xbc;
    case "Period":
      return 0xbe;
    case "Slash":
      return 0xbf;
    case "PrintScreen":
      return 0x2c;
    case "NumpadAdd":
      return 0x6b;
    case "NumpadSubtract":
      return 0x6d;
    case "NumpadMultiply":
      return 0x6a;
    case "NumpadDivide":
      return 0x6f;
    case "NumpadDecimal":
      return 0x6e;
    default:
      return null;
  }
}

export function formatHotkeyBinding(binding: HotkeyBinding): string {
  const parts: string[] = [];
  if (binding.ctrl) {
    parts.push("Ctrl");
  }
  if (binding.alt) {
    parts.push("Alt");
  }
  if (binding.shift) {
    parts.push("Shift");
  }
  if (binding.win) {
    parts.push("Win");
  }
  const key = vkName(binding.vk);
  parts.push(key);
  if (binding.taps === 2) {
    parts.push(key);
  }
  return parts.join("+");
}

function vkName(vk: number): string {
  if (vk >= 0x41 && vk <= 0x5a) {
    return String.fromCharCode(vk);
  }
  if (vk >= 0x30 && vk <= 0x39) {
    return String.fromCharCode(vk);
  }
  if (vk >= 0x70 && vk <= 0x87) {
    return `F${vk - 0x6f}`;
  }
  if (vk >= 0x60 && vk <= 0x69) {
    return `Num${vk - 0x60}`;
  }
  switch (vk) {
    case 0x08:
      return "Backspace";
    case 0x09:
      return "Tab";
    case 0x0d:
      return "Enter";
    case 0x20:
      return "Space";
    case 0x21:
      return "PageUp";
    case 0x22:
      return "PageDown";
    case 0x23:
      return "End";
    case 0x24:
      return "Home";
    case 0x25:
      return "Left";
    case 0x26:
      return "Up";
    case 0x27:
      return "Right";
    case 0x28:
      return "Down";
    case 0x2c:
      return "PrintScreen";
    case 0x2d:
      return "Insert";
    case 0x2e:
      return "Delete";
    case 0x6a:
      return "Num*";
    case 0x6b:
      return "Num+";
    case 0x6d:
      return "Num-";
    case 0x6e:
      return "Num.";
    case 0x6f:
      return "Num/";
    case 0xba:
      return ";";
    case 0xbb:
      return "=";
    case 0xbc:
      return ",";
    case 0xbd:
      return "-";
    case 0xbe:
      return ".";
    case 0xbf:
      return "/";
    case 0xc0:
      return "`";
    case 0xdb:
      return "[";
    case 0xdc:
      return "\\";
    case 0xdd:
      return "]";
    case 0xde:
      return "'";
    default:
      return `Vk${vk.toString(16).toUpperCase()}`;
  }
}

function isApiKeyInfo(value: unknown): value is ApiKeyInfo {
  return (
    isRecord(value) &&
    typeof value.id === "string" &&
    typeof value.name === "string" &&
    typeof value.provider === "string" &&
    typeof value.model === "string"
  );
}

export function isSettingsState(value: unknown): value is SettingsState {
  return (
    isRecord(value) &&
    typeof value.hasApiKey === "boolean" &&
    (value.activeKeyId === null || typeof value.activeKeyId === "string") &&
    Array.isArray(value.apiKeys) &&
    value.apiKeys.every(isApiKeyInfo) &&
    typeof value.autostart === "boolean" &&
    typeof value.closeToTray === "boolean" &&
    typeof value.startMinimized === "boolean" &&
    typeof value.hotkeyLabel === "string" &&
    typeof value.canAutostart === "boolean" &&
    typeof value.uiLanguage === "string" &&
    typeof value.nativeLanguage === "string" &&
    typeof value.foreignLanguage === "string" &&
    Array.isArray(value.promptRules) &&
    value.promptRules.every((rule) => typeof rule === "string") &&
    typeof value.rulesCustomized === "boolean" &&
    typeof value.timeoutSecs === "number" &&
    typeof value.maxOutputTokens === "number" &&
    typeof value.fallbackModels === "boolean"
  );
}

export type KeySnapshot = {
  hasApiKey: boolean;
  activeKeyId: string | null;
  apiKeys: ApiKeyInfo[];
};

export function isKeySnapshot(value: unknown): value is KeySnapshot {
  return (
    isRecord(value) &&
    typeof value.hasApiKey === "boolean" &&
    (value.activeKeyId === null || typeof value.activeKeyId === "string") &&
    Array.isArray(value.apiKeys) &&
    value.apiKeys.every(isApiKeyInfo)
  );
}

export type TrafficEntry = {
  id: string;
  atMs: number;
  provider: string;
  model: string;
  profileName: string;
  elapsedMs: number;
  source: string;
  systemPrompt: string;
  userMessage: string;
  response: string;
  error: string;
};

export function isTrafficEntry(value: unknown): value is TrafficEntry {
  return (
    isRecord(value) &&
    typeof value.id === "string" &&
    typeof value.atMs === "number" &&
    typeof value.provider === "string" &&
    typeof value.model === "string" &&
    typeof value.profileName === "string" &&
    typeof value.elapsedMs === "number" &&
    typeof value.source === "string" &&
    typeof value.systemPrompt === "string" &&
    typeof value.userMessage === "string" &&
    typeof value.response === "string" &&
    typeof value.error === "string"
  );
}

export function isTrafficList(value: unknown): value is TrafficEntry[] {
  return Array.isArray(value) && value.every(isTrafficEntry);
}
