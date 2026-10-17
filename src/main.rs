#![cfg_attr(mobile, tauri::mobile_entry_point)]

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use std::time::Duration;
use reqwest;
use tokio::time::timeout;

#[tauri::command]
async fn open_animeon_site(app_handle: tauri::AppHandle) {
    // Try to access the main site first
    let main_site = "https://animeon.cc/";
    let mirror_site = "https://v2.animeon.co/";
    
    let is_accessible = check_site_accessibility(main_site).await;
    
    let url_to_open = if is_accessible {
        main_site.to_string()
    } else {
        mirror_site.to_string()
    };
    
    // Open the URL in default browser
    tauri::shell::open(&app_handle.shell_scope(), url_to_open, None).unwrap();
}

#[tauri::command]
async fn check_site_accessibility(url: &str) -> bool {
    let client = reqwest::Client::new();
    
    match timeout(Duration::from_secs(5), client.get(url).send()).await {
        Ok(Ok(_response)) => true,
        _ => false
    }
}

#[tauri::command]
async fn open_google_login(app_handle: tauri::AppHandle) {
    // Open Google login URL in default browser
    let google_url = "https://accounts.google.com/o/oauth2/auth?client_id=691835435970-8j9k5p8g1q2v3w4x5y6z7a8b9c0d1e2f.apps.googleusercontent.com&redirect_uri=http://localhost:3000/oauth2callback&response_type=code&scope=email%20profile";
    tauri::shell::open(&app_handle.shell_scope(), google_url, None).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_animeon_site, check_site_accessibility, open_google_login])
        .setup(|app| {
            // Create the main window
            let window = WebviewWindowBuilder::new(
                app,
                "main".to_string(),
                WebviewUrl::External("https://animeon.cc/".parse().unwrap())
            )
            .title("AnimeOn")
            .min_width(800.0)
            .min_height(600.0)
            .width(1200.0)
            .height(800.0)
            .build()
            .unwrap();
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}