use std::time::Duration;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

fn get_target_url() -> String {
    let main_site = "https://animeon.cc/";
    let mirror_site = "https://v2.animeon.co/";

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(3))
        .build();

    if let Ok(cli) = client {
        if let Ok(res) = cli.get(main_site).send() {
            if res.status().is_success() || res.status().is_redirection() {
                return main_site.to_string();
            }
        }
    }

    mirror_site.to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let start_url = get_target_url();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let parsed_url = url::Url::parse(&start_url).expect("Invalid URL");
            let handle = app.handle().clone();

            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(parsed_url))
                .title("AnimeOn")
                .inner_size(1280.0, 800.0)
                .resizable(true)
                .on_navigation(move |url| {
                    let u = url.as_str();
                    if u.contains("accounts.google.com") || u.contains("auth/google") {
                        let _ = handle.opener().open_url(u, None::<&str>);
                        false
                    } else {
                        true
                    }
                })
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
