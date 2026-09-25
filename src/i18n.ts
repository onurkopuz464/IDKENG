export const LANGUAGES = [
  { code: "tr", label: "Türkçe" },
  { code: "en", label: "English" },
  { code: "de", label: "Deutsch" },
  { code: "fr", label: "Français" },
  { code: "es", label: "Español" },
  { code: "it", label: "Italiano" },
  { code: "pt", label: "Português" },
  { code: "ru", label: "Русский" },
  { code: "ar", label: "العربية" },
  { code: "ja", label: "日本語" },
  { code: "zh", label: "中文" },
  { code: "ko", label: "한국어" },
] as const;

export type LangCode = (typeof LANGUAGES)[number]["code"];

export type Copy = {
  settings: string;
  close: string;
  activeKey: string;
  translate: string;
  translating: string;
  copy: string;
  copied: string;
  placeholder: string;
  shortcutPrefix: string;
  emptyTranslation: string;
  noKeys: string;
  newProfile: string;
  profileName: string;
  add: string;
  adding: string;
  added: string;
  remove: string;
  autostart: string;
  closeToTray: string;
  startMinimized: string;
  devAutostart: string;
  hotkey: string;
  changeHotkey: string;
  cancel: string;
  pressKeys: string;
  doubleAgain: string;
  escCancel: string;
  savedHotkey: string;
  uiLanguage: string;
  nativeLanguage: string;
  foreignLanguage: string;
  languageHint: string;
  prompt: string;
  promptHint: string;
  resetPrompt: string;
  addRule: string;
  removeRule: string;
  save: string;
  saved: string;
  traffic: string;
  noTraffic: string;
  clearTraffic: string;
  request: string;
  response: string;
  elapsed: string;
  provider: string;
  model: string;
  baseUrl: string;
  baseUrlHint: string;
  fallback: string;
  timeout: string;
  maxTokens: string;
  advanced: string;
  api: string;
  appearance: string;
  translationTab: string;
  custom: string;
  errName: string;
  errKey: string;
  errDuplicate: string;
  errProvider: string;
  errModel: string;
  errBaseUrl: string;
  errNotFound: string;
  errNoKey: string;
  errNoText: string;
  errTooLong: string;
  errTimeout: string;
  errNetwork: string;
  errInvalidKey: string;
  errQuota: string;
  errBusy: string;
  errEmpty: string;
  errFailed: string;
  errUnreadable: string;
  errLanguage: string;
  errSameLanguage: string;
  errTimeoutRange: string;
  errTokens: string;
  errGeneric: string;
};

const en: Copy = {
  settings: "Settings",
  close: "Close",
  activeKey: "Active API",
  translate: "Translate",
  translating: "Translating...",
  copy: "Copy",
  copied: "Copied",
  placeholder: "Type or paste text to translate",
  shortcutPrefix: "or select text and",
  emptyTranslation: "Translation shows up here",
  noKeys: "No API yet.",
  newProfile: "New API",
  profileName: "Name, e.g. Work",
  add: "Add",
  adding: "Adding...",
  added: "API added.",
  remove: "Delete",
  autostart: "Start with Windows",
  closeToTray: "Close to tray",
  startMinimized: "Start in the tray",
  devAutostart: "This is the npm run tauri dev build. Windows startup needs the installed app.",
  hotkey: "Shortcut",
  changeHotkey: "Change",
  cancel: "Cancel",
  pressKeys: "Press the keys...",
  doubleAgain: "Press the same key once more for a double tap.",
  escCancel: "Hold the modifiers, then press the key. Esc cancels.",
  savedHotkey: "Shortcut saved.",
  uiLanguage: "App language",
  nativeLanguage: "Native language",
  foreignLanguage: "Other language",
  languageHint:
    "The other language is where text in your native language is translated. Anything else comes back to your native language.",
  prompt: "Prompt",
  promptHint:
    "The first rule is the target language. It rewrites itself when you change your native language or the language in that rule. You can add and remove the rules below it.",
  resetPrompt: "Restore defaults",
  addRule: "Add rule",
  removeRule: "Remove",
  save: "Save",
  saved: "Saved.",
  traffic: "Requests",
  noTraffic: "No requests yet.",
  clearTraffic: "Clear log",
  request: "Sent",
  response: "Received",
  elapsed: "ms",
  provider: "API",
  model: "Model",
  baseUrl: "Base URL",
  baseUrlHint: "Required for a custom API. Must start with https://",
  fallback: "If Gemini is busy, try the next model",
  timeout: "Timeout (seconds)",
  maxTokens: "Max output tokens",
  advanced: "Advanced",
  api: "APIs",
  appearance: "Interface",
  translationTab: "Translation",
  custom: "Custom",
  errName: "Give the API a name.",
  errKey: "The API key looks too short.",
  errDuplicate: "That name is already used.",
  errProvider: "Pick an API provider.",
  errModel: "Pick a model.",
  errBaseUrl: "Custom APIs need an https:// base URL.",
  errNotFound: "API not found.",
  errNoKey: "Add an API key in Settings.",
  errNoText: "No text selected.",
  errTooLong: "Text is too long (8000 characters).",
  errTimeout: "Translation timed out.",
  errNetwork: "Could not reach the API.",
  errInvalidKey: "API key is invalid.",
  errQuota: "API quota is used up. Try again later.",
  errBusy: "The model is busy. Try again later.",
  errEmpty: "The API returned an empty translation.",
  errFailed: "Translation failed.",
  errUnreadable: "The API response could not be read.",
  errLanguage: "Pick a supported language.",
  errSameLanguage: "Native and other language must differ.",
  errTimeoutRange: "Timeout must be between 5 and 120 seconds.",
  errTokens: "Max tokens must be between 128 and 8192.",
  errGeneric: "Something went wrong.",
};

const tr: Copy = {
  ...en,
  settings: "Ayarlar",
  close: "Kapat",
  activeKey: "Aktif API",
  translate: "Çevir",
  translating: "Çevriliyor...",
  copy: "Kopyala",
  copied: "Kopyalandı",
  placeholder: "Çevirmek için yaz veya yapıştır",
  shortcutPrefix: "veya metin seçip",
  emptyTranslation: "Çeviri burada görünür",
  noKeys: "Henüz API yok.",
  newProfile: "Yeni API",
  profileName: "İsim, örn. İş",
  add: "Ekle",
  adding: "Ekleniyor...",
  added: "API eklendi.",
  remove: "Sil",
  autostart: "Windows ile başlat",
  closeToTray: "Kapatınca tepsiye küçült",
  startMinimized: "Başlangıçta tepside başlat",
  devAutostart: "Bu npm run tauri dev sürümü. Açılış için kurulu uygulamayı kullan.",
  hotkey: "Kısayol",
  changeHotkey: "Değiştir",
  cancel: "Vazgeç",
  pressKeys: "Tuşlara bas...",
  doubleAgain: "Çift vuruş için aynı tuşa bir kez daha bas.",
  escCancel: "Ctrl basılı tut, sonra tuşa bas. Esc iptal.",
  savedHotkey: "Kısayol kaydedildi.",
  uiLanguage: "Uygulama dili",
  nativeLanguage: "Ana dil",
  foreignLanguage: "Diğer dil",
  languageHint:
    "Diğer dil, ana dilde yazılmış metnin çevrileceği dildir. Ana dil dışındaki her metin ana dile döner.",
  prompt: "Prompt",
  promptHint:
    "İlk madde hedef dildir. Ana dil veya o maddedeki dil değişince kendiliğinden yazılır. Alttaki maddeleri ekleyip silebilirsin.",
  resetPrompt: "Varsayılanlara dön",
  addRule: "Madde ekle",
  removeRule: "Kaldır",
  save: "Kaydet",
  saved: "Kaydedildi.",
  traffic: "İstekler",
  noTraffic: "Henüz istek yok.",
  clearTraffic: "Günlüğü temizle",
  request: "Giden",
  response: "Gelen",
  provider: "API",
  model: "Model",
  baseUrl: "Adres",
  baseUrlHint: "Özel API için gerekli. https:// ile başlamalı.",
  fallback: "Gemini yoğunsa sıradaki modeli dene",
  timeout: "Zaman aşımı (saniye)",
  maxTokens: "En fazla çıktı token",
  advanced: "Gelişmiş",
  api: "API'ler",
  appearance: "Arayüz",
  translationTab: "Çeviri",
  custom: "Özel",
  errName: "API'ye bir isim ver.",
  errKey: "API anahtarı çok kısa görünüyor.",
  errDuplicate: "Bu isimde bir API zaten var.",
  errProvider: "API sağlayıcısını seç.",
  errModel: "Model seç.",
  errBaseUrl: "Özel API için https:// adresi gerekli.",
  errNotFound: "API bulunamadı.",
  errNoKey: "Ayarlardan bir API anahtarı ekle.",
  errNoText: "Seçili metin yok.",
  errTooLong: "Metin çok uzun (8000 karakter).",
  errTimeout: "Çeviri zaman aşımına uğradı.",
  errNetwork: "API'ye ulaşılamadı.",
  errInvalidKey: "API anahtarı geçersiz.",
  errQuota: "API kotası doldu, biraz sonra dene.",
  errBusy: "Model şu an yoğun, biraz sonra dene.",
  errEmpty: "API boş çeviri döndürdü.",
  errFailed: "Çeviri başarısız.",
  errUnreadable: "API yanıtı okunamadı.",
  errLanguage: "Desteklenen bir dil seç.",
  errSameLanguage: "Ana dil ile çevrilecek dil farklı olmalı.",
  errTimeoutRange: "Zaman aşımı 5 ile 120 saniye arasında olmalı.",
  errTokens: "Token sınırı 128 ile 8192 arasında olmalı.",
  errGeneric: "İşlem başarısız.",
};

const de: Copy = {
  ...en,
  settings: "Einstellungen",
  close: "Schließen",
  activeKey: "Aktive API",
  translate: "Übersetzen",
  translating: "Übersetzt...",
  copy: "Kopieren",
  copied: "Kopiert",
  placeholder: "Text zum Übersetzen eingeben oder einfügen",
  shortcutPrefix: "oder Text markieren und",
  emptyTranslation: "Die Übersetzung erscheint hier",
  noKeys: "Noch keine API.",
  newProfile: "Neue API",
  profileName: "Name, z. B. Arbeit",
  add: "Hinzufügen",
  adding: "Wird hinzugefügt...",
  added: "API hinzugefügt.",
  remove: "Löschen",
  autostart: "Mit Windows starten",
  closeToTray: "Schließen in die Taskleiste",
  startMinimized: "Im Infobereich starten",
  hotkey: "Kürzel",
  changeHotkey: "Ändern",
  cancel: "Abbrechen",
  pressKeys: "Tasten drücken...",
  savedHotkey: "Kürzel gespeichert.",
  uiLanguage: "App-Sprache",
  nativeLanguage: "Muttersprache",
  foreignLanguage: "Andere Sprache",
  languageHint:
    "Die andere Sprache ist die Zielsprache für Text in der Muttersprache. Alles andere kommt in die Muttersprache zurück.",
  prompt: "Prompt",
  promptHint:
    "Jede Zeile ist eine Regel. Unveränderte Regeln werden neu erzeugt, wenn du App-Sprache, Muttersprache oder andere Sprache änderst.",
  resetPrompt: "Standard wiederherstellen",
  addRule: "Regel hinzufügen",
  removeRule: "Entfernen",
  save: "Speichern",
  saved: "Gespeichert.",
  traffic: "Anfragen",
  noTraffic: "Noch keine Anfragen.",
  clearTraffic: "Protokoll leeren",
  request: "Gesendet",
  response: "Empfangen",
  provider: "API",
  model: "Modell",
  baseUrl: "Adresse",
  fallback: "Bei Auslastung das nächste Gemini-Modell versuchen",
  timeout: "Zeitlimit (Sekunden)",
  maxTokens: "Maximale Ausgabetokens",
  advanced: "Erweitert",
  api: "APIs",
  appearance: "Oberfläche",
  translationTab: "Übersetzung",
  custom: "Eigen",
  errTimeout: "Übersetzung hat das Zeitlimit überschritten.",
  errBusy: "Das Modell ist ausgelastet. Später erneut versuchen.",
  errNoKey: "Füge in den Einstellungen einen API-Schlüssel hinzu.",
  errNoText: "Kein Text ausgewählt.",
};

const fr: Copy = {
  ...en,
  settings: "Réglages",
  close: "Fermer",
  activeKey: "API active",
  translate: "Traduire",
  translating: "Traduction...",
  copy: "Copier",
  copied: "Copié",
  placeholder: "Écris ou colle le texte à traduire",
  shortcutPrefix: "ou sélectionne le texte et",
  emptyTranslation: "La traduction s'affiche ici",
  noKeys: "Pas encore d'API.",
  newProfile: "Nouvelle API",
  add: "Ajouter",
  adding: "Ajout...",
  added: "API ajoutée.",
  remove: "Supprimer",
  autostart: "Démarrer avec Windows",
  closeToTray: "Fermer dans la barre",
  startMinimized: "Démarrer dans la barre",
  hotkey: "Raccourci",
  changeHotkey: "Modifier",
  cancel: "Annuler",
  pressKeys: "Appuie sur les touches...",
  savedHotkey: "Raccourci enregistré.",
  uiLanguage: "Langue de l'app",
  nativeLanguage: "Langue maternelle",
  foreignLanguage: "Autre langue",
  languageHint:
    "L'autre langue est celle vers laquelle part le texte écrit dans la langue maternelle. Tout le reste revient vers la langue maternelle.",
  prompt: "Prompt",
  promptHint:
    "Chaque ligne est une règle. Les règles non modifiées se régénèrent si tu changes la langue de l'app, la langue maternelle ou l'autre langue.",
  resetPrompt: "Revenir au défaut",
  addRule: "Ajouter une règle",
  removeRule: "Retirer",
  save: "Enregistrer",
  saved: "Enregistré.",
  traffic: "Requêtes",
  noTraffic: "Pas encore de requête.",
  clearTraffic: "Effacer le journal",
  request: "Envoyé",
  response: "Reçu",
  baseUrl: "Adresse",
  fallback: "Si Gemini est saturé, essayer le modèle suivant",
  timeout: "Délai (secondes)",
  maxTokens: "Jetons de sortie max",
  advanced: "Avancé",
  appearance: "Interface",
  translationTab: "Traduction",
  custom: "Perso",
  errTimeout: "La traduction a expiré.",
  errBusy: "Le modèle est saturé. Réessaie plus tard.",
  errNoKey: "Ajoute une clé API dans les réglages.",
  errNoText: "Aucun texte sélectionné.",
};

const es: Copy = {
  ...en,
  settings: "Ajustes",
  close: "Cerrar",
  activeKey: "API activa",
  translate: "Traducir",
  translating: "Traduciendo...",
  copy: "Copiar",
  copied: "Copiado",
  placeholder: "Escribe o pega el texto a traducir",
  shortcutPrefix: "o selecciona texto y",
  emptyTranslation: "La traducción aparece aquí",
  noKeys: "Todavía no hay API.",
  newProfile: "Nueva API",
  add: "Añadir",
  adding: "Añadiendo...",
  added: "API añadida.",
  remove: "Borrar",
  autostart: "Iniciar con Windows",
  closeToTray: "Cerrar a la bandeja",
  startMinimized: "Iniciar en la bandeja",
  hotkey: "Atajo",
  changeHotkey: "Cambiar",
  cancel: "Cancelar",
  pressKeys: "Pulsa las teclas...",
  savedHotkey: "Atajo guardado.",
  uiLanguage: "Idioma de la app",
  nativeLanguage: "Idioma nativo",
  foreignLanguage: "Otro idioma",
  languageHint:
    "El otro idioma es al que se traduce el texto escrito en el idioma nativo. Todo lo demás vuelve al idioma nativo.",
  prompt: "Prompt",
  promptHint:
    "Cada línea es una regla. Las reglas que no editaste se regeneran al cambiar el idioma de la app, el nativo u el otro idioma.",
  resetPrompt: "Volver al predeterminado",
  addRule: "Añadir regla",
  removeRule: "Quitar",
  save: "Guardar",
  saved: "Guardado.",
  traffic: "Peticiones",
  noTraffic: "Todavía no hay peticiones.",
  clearTraffic: "Borrar registro",
  request: "Enviado",
  response: "Recibido",
  baseUrl: "Dirección",
  fallback: "Si Gemini está saturado, probar el siguiente modelo",
  timeout: "Tiempo límite (segundos)",
  maxTokens: "Máximo de tokens de salida",
  advanced: "Avanzado",
  appearance: "Interfaz",
  translationTab: "Traducción",
  custom: "Propia",
  errTimeout: "La traducción agotó el tiempo.",
  errBusy: "El modelo está saturado. Prueba más tarde.",
  errNoKey: "Añade una clave API en Ajustes.",
  errNoText: "No hay texto seleccionado.",
};

const it: Copy = {
  ...en,
  settings: "Impostazioni",
  close: "Chiudi",
  translate: "Traduci",
  translating: "Traduzione...",
  copy: "Copia",
  copied: "Copiato",
  placeholder: "Scrivi o incolla il testo da tradurre",
  shortcutPrefix: "oppure seleziona il testo e",
  emptyTranslation: "La traduzione appare qui",
  noKeys: "Nessuna API.",
  newProfile: "Nuova API",
  add: "Aggiungi",
  remove: "Elimina",
  hotkey: "Scorciatoia",
  changeHotkey: "Cambia",
  cancel: "Annulla",
  uiLanguage: "Lingua dell'app",
  nativeLanguage: "Lingua madre",
  foreignLanguage: "Altra lingua",
  languageHint:
    "L'altra lingua è quella in cui va il testo scritto nella lingua madre. Tutto il resto torna alla lingua madre.",
  promptHint:
    "Ogni riga è una regola. Le regole non modificate si rigenerano se cambi lingua dell'app, lingua madre o altra lingua.",
  resetPrompt: "Ripristina predefiniti",
  addRule: "Aggiungi regola",
  removeRule: "Rimuovi",
  save: "Salva",
  saved: "Salvato.",
  traffic: "Richieste",
  advanced: "Avanzate",
  appearance: "Interfaccia",
  translationTab: "Traduzione",
  custom: "Personalizzata",
  errTimeout: "Traduzione scaduta.",
  errNoText: "Nessun testo selezionato.",
  errNoKey: "Aggiungi una chiave API nelle impostazioni.",
};

const pt: Copy = {
  ...en,
  settings: "Definições",
  close: "Fechar",
  translate: "Traduzir",
  translating: "A traduzir...",
  copy: "Copiar",
  copied: "Copiado",
  placeholder: "Escreve ou cola o texto a traduzir",
  shortcutPrefix: "ou seleciona o texto e",
  emptyTranslation: "A tradução aparece aqui",
  noKeys: "Ainda não há API.",
  newProfile: "Nova API",
  add: "Adicionar",
  remove: "Apagar",
  hotkey: "Atalho",
  changeHotkey: "Mudar",
  cancel: "Cancelar",
  uiLanguage: "Idioma da app",
  nativeLanguage: "Língua nativa",
  foreignLanguage: "Outra língua",
  languageHint:
    "A outra língua é o destino do texto escrito na língua nativa. Tudo o resto volta para a língua nativa.",
  promptHint:
    "Cada linha é uma regra. Regras que não editaste regeneram-se ao mudares o idioma da app, a língua nativa ou a outra língua.",
  resetPrompt: "Repor predefinição",
  addRule: "Adicionar regra",
  removeRule: "Remover",
  save: "Guardar",
  saved: "Guardado.",
  traffic: "Pedidos",
  advanced: "Avançado",
  appearance: "Interface",
  translationTab: "Tradução",
  custom: "Própria",
  errTimeout: "A tradução expirou.",
  errNoText: "Nenhum texto selecionado.",
  errNoKey: "Adiciona uma chave API nas definições.",
};

const ru: Copy = {
  ...en,
  settings: "Настройки",
  close: "Закрыть",
  activeKey: "Активный API",
  translate: "Перевести",
  translating: "Перевод...",
  copy: "Копировать",
  copied: "Скопировано",
  placeholder: "Введите или вставьте текст",
  shortcutPrefix: "или выделите текст и",
  emptyTranslation: "Перевод появится здесь",
  noKeys: "API ещё нет.",
  newProfile: "Новый API",
  add: "Добавить",
  adding: "Добавление...",
  added: "API добавлен.",
  remove: "Удалить",
  hotkey: "Сочетание",
  changeHotkey: "Изменить",
  cancel: "Отмена",
  pressKeys: "Нажмите клавиши...",
  savedHotkey: "Сочетание сохранено.",
  uiLanguage: "Язык приложения",
  nativeLanguage: "Родной язык",
  foreignLanguage: "Другой язык",
  languageHint:
    "Другой язык — это язык, на который переводится текст на родном. Всё остальное возвращается на родной.",
  prompt: "Промпт",
  promptHint:
    "Каждая строка — отдельное правило. Неизменённые правила обновляются при смене языка приложения, родного или другого языка.",
  resetPrompt: "Вернуть по умолчанию",
  addRule: "Добавить правило",
  removeRule: "Убрать",
  save: "Сохранить",
  saved: "Сохранено.",
  traffic: "Запросы",
  noTraffic: "Запросов ещё нет.",
  clearTraffic: "Очистить журнал",
  request: "Отправлено",
  response: "Получено",
  fallback: "Если Gemini занят, пробовать следующую модель",
  timeout: "Таймаут (секунды)",
  maxTokens: "Макс. токенов ответа",
  advanced: "Дополнительно",
  appearance: "Интерфейс",
  translationTab: "Перевод",
  custom: "Свой",
  errTimeout: "Время перевода истекло.",
  errBusy: "Модель перегружена. Попробуйте позже.",
  errNoKey: "Добавьте ключ API в настройках.",
  errNoText: "Текст не выбран.",
};

const ar: Copy = {
  ...en,
  settings: "الإعدادات",
  close: "إغلاق",
  translate: "ترجم",
  translating: "جار الترجمة...",
  copy: "نسخ",
  copied: "تم النسخ",
  placeholder: "اكتب النص أو الصقه للترجمة",
  shortcutPrefix: "أو حدد النص ثم",
  emptyTranslation: "تظهر الترجمة هنا",
  noKeys: "لا توجد واجهة بعد.",
  newProfile: "واجهة جديدة",
  add: "إضافة",
  remove: "حذف",
  hotkey: "الاختصار",
  changeHotkey: "تغيير",
  cancel: "إلغاء",
  uiLanguage: "لغة التطبيق",
  nativeLanguage: "اللغة الأم",
  foreignLanguage: "اللغة الأخرى",
  languageHint: "اللغة الأخرى هي التي يُترجم إليها النص المكتوب باللغة الأم. أي نص بغير اللغة الأم يعود إليها.",
  promptHint: "كل سطر قاعدة. القواعد التي لم تعدّلها تتجدد عند تغيير لغة التطبيق أو اللغة الأم أو اللغة الأخرى.",
  resetPrompt: "استعادة الافتراضي",
  addRule: "إضافة قاعدة",
  removeRule: "إزالة",
  save: "حفظ",
  saved: "تم الحفظ.",
  traffic: "الطلبات",
  advanced: "متقدم",
  appearance: "الواجهة",
  translationTab: "الترجمة",
  custom: "مخصص",
  errTimeout: "انتهت مهلة الترجمة.",
  errNoText: "لا يوجد نص محدد.",
  errNoKey: "أضف مفتاح واجهة من الإعدادات.",
};

const ja: Copy = {
  ...en,
  settings: "設定",
  close: "閉じる",
  translate: "翻訳",
  translating: "翻訳中...",
  copy: "コピー",
  copied: "コピーしました",
  placeholder: "翻訳する文を入力または貼り付け",
  shortcutPrefix: "またはテキストを選んで",
  emptyTranslation: "翻訳はここに出ます",
  noKeys: "APIはまだありません。",
  newProfile: "新しいAPI",
  add: "追加",
  remove: "削除",
  hotkey: "ショートカット",
  changeHotkey: "変更",
  cancel: "キャンセル",
  uiLanguage: "アプリの言語",
  nativeLanguage: "母語",
  foreignLanguage: "もう一方の言語",
  languageHint: "もう一方の言語は、母語で書いた文の訳し先です。母語以外の文は母語に戻します。",
  promptHint: "1行が1つの規則です。編集していない規則は、アプリの言語・母語・もう一方の言語を変えると作り直されます。",
  resetPrompt: "初期状態に戻す",
  addRule: "規則を追加",
  removeRule: "削除",
  save: "保存",
  saved: "保存しました。",
  traffic: "リクエスト",
  advanced: "詳細",
  appearance: "表示",
  translationTab: "翻訳",
  custom: "カスタム",
  errTimeout: "翻訳がタイムアウトしました。",
  errNoText: "テキストが選択されていません。",
  errNoKey: "設定でAPIキーを追加してください。",
};

const zh: Copy = {
  ...en,
  settings: "设置",
  close: "关闭",
  translate: "翻译",
  translating: "翻译中...",
  copy: "复制",
  copied: "已复制",
  placeholder: "输入或粘贴要翻译的文字",
  shortcutPrefix: "或选中文字后按",
  emptyTranslation: "翻译显示在这里",
  noKeys: "还没有 API。",
  newProfile: "新建 API",
  add: "添加",
  remove: "删除",
  hotkey: "快捷键",
  changeHotkey: "更改",
  cancel: "取消",
  uiLanguage: "界面语言",
  nativeLanguage: "母语",
  foreignLanguage: "另一种语言",
  languageHint: "另一种语言是母语文本要译成的语言。不是母语的文本都会译回母语。",
  promptHint: "每一行是一条规则。没改过的规则会在界面语言、母语或另一种语言变化时重新生成。",
  resetPrompt: "恢复默认",
  addRule: "添加规则",
  removeRule: "移除",
  save: "保存",
  saved: "已保存。",
  traffic: "请求",
  advanced: "高级",
  appearance: "界面",
  translationTab: "翻译",
  custom: "自定义",
  errTimeout: "翻译超时。",
  errNoText: "没有选中文字。",
  errNoKey: "请在设置里添加 API 密钥。",
};

const ko: Copy = {
  ...en,
  settings: "설정",
  close: "닫기",
  translate: "번역",
  translating: "번역 중...",
  copy: "복사",
  copied: "복사됨",
  placeholder: "번역할 글을 입력하거나 붙여넣기",
  shortcutPrefix: "또는 글을 선택하고",
  emptyTranslation: "번역이 여기에 표시됩니다",
  noKeys: "아직 API가 없습니다.",
  newProfile: "새 API",
  add: "추가",
  remove: "삭제",
  hotkey: "단축키",
  changeHotkey: "변경",
  cancel: "취소",
  uiLanguage: "앱 언어",
  nativeLanguage: "모국어",
  foreignLanguage: "다른 언어",
  languageHint: "다른 언어는 모국어로 쓴 글이 번역될 언어입니다. 모국어가 아닌 글은 모국어로 돌아옵니다.",
  promptHint: "한 줄이 한 규칙입니다. 직접 고치지 않은 규칙은 앱 언어, 모국어, 다른 언어를 바꾸면 다시 만들어집니다.",
  resetPrompt: "기본값으로",
  addRule: "규칙 추가",
  removeRule: "제거",
  save: "저장",
  saved: "저장됨.",
  traffic: "요청",
  advanced: "고급",
  appearance: "화면",
  translationTab: "번역",
  custom: "사용자 지정",
  errTimeout: "번역 시간이 초과되었습니다.",
  errNoText: "선택한 글이 없습니다.",
  errNoKey: "설정에서 API 키를 추가하세요.",
};

const TABLE: Record<string, Copy> = { en, tr, de, fr, es, it, pt, ru, ar, ja, zh, ko };

export function copyFor(language: string): Copy {
  return TABLE[language] ?? en;
}

export function isRtl(language: string): boolean {
  return language === "ar";
}

const ERRORS: Record<string, keyof Copy> = {
  "err:name": "errName",
  "err:key": "errKey",
  "err:duplicate": "errDuplicate",
  "err:provider": "errProvider",
  "err:model": "errModel",
  "err:base_url": "errBaseUrl",
  "err:not_found": "errNotFound",
  "err:no_key": "errNoKey",
  "err:no_text": "errNoText",
  "err:too_long": "errTooLong",
  "err:timeout": "errTimeout",
  "err:network": "errNetwork",
  "err:invalid_key": "errInvalidKey",
  "err:quota": "errQuota",
  "err:busy": "errBusy",
  "err:empty": "errEmpty",
  "err:failed": "errFailed",
  "err:unreadable": "errUnreadable",
  "err:language": "errLanguage",
  "err:same_language": "errSameLanguage",
  "err:timeout_range": "errTimeoutRange",
  "err:tokens": "errTokens",
};

export function displayError(message: string, copy: Copy): string {
  const key = ERRORS[message];
  if (key) {
    return copy[key];
  }
  return message.length > 0 ? message : copy.errGeneric;
}

export const PROVIDERS = [
  { id: "gemini", models: ["gemini-3.1-flash-lite", "gemini-3.5-flash", "gemini-3-flash-preview"] },
  { id: "openai", models: ["gpt-4o-mini", "gpt-4.1-mini", "gpt-4o"] },
  { id: "groq", models: ["llama-3.3-70b-versatile", "llama-3.1-8b-instant"] },
  { id: "openrouter", models: ["google/gemini-2.5-flash", "openai/gpt-4o-mini"] },
  { id: "custom", models: [] },
] as const;
