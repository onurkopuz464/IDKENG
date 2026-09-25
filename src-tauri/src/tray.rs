use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::AppHandle;

use crate::overlay;

pub fn setup(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let menu = menu(app)?;
    let icon = app
        .default_window_icon()
        .ok_or("pencere ikonu yok")?
        .clone();

    TrayIconBuilder::with_id("tray")
        .icon(icon)
        .tooltip("IDKENG")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => overlay::show_main(app),
            "settings" => overlay::open_settings(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::DoubleClick { .. } = event {
                overlay::show_main(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}

pub fn refresh(app: &AppHandle) {
    let Ok(menu) = menu(app) else {
        return;
    };
    if let Some(tray) = app.tray_by_id("tray") {
        let _ = tray.set_menu(Some(menu));
    }
}

fn menu(app: &AppHandle) -> Result<Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    let lang = crate::settings::ui_language();
    let (show, settings, quit) = match lang.as_str() {
        "en" => ("Show window", "Settings", "Quit"),
        "de" => ("Fenster zeigen", "Einstellungen", "Beenden"),
        "fr" => ("Afficher", "Réglages", "Quitter"),
        "es" => ("Mostrar ventana", "Ajustes", "Salir"),
        "it" => ("Mostra finestra", "Impostazioni", "Esci"),
        "pt" => ("Mostrar janela", "Definições", "Sair"),
        "ru" => ("Показать окно", "Настройки", "Выход"),
        "ar" => ("إظهار النافذة", "الإعدادات", "خروج"),
        "ja" => ("ウィンドウを表示", "設定", "終了"),
        "zh" => ("显示窗口", "设置", "退出"),
        "ko" => ("창 표시", "설정", "종료"),
        _ => ("Pencereyi göster", "Ayarlar", "Çıkış"),
    };
    let show_item = MenuItem::with_id(app, "show", show, true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", settings, true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", quit, true, None::<&str>)?;
    Ok(Menu::with_items(app, &[&show_item, &settings_item, &quit_item])?)
}
