#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{WindowBuilder, WindowUrl};

mod chat;
mod commands;
mod error;
mod project;
mod prompt;
mod result;
mod setting;
mod state;
mod utils;
mod window;

#[tokio::main]
async fn main() {
    env_logger::init();
    tauri::Builder::default()
        .manage(state::AppState::init().await.unwrap())
        .setup(|app| {
            let mut main_window_builder =
                WindowBuilder::new(app, "main", WindowUrl::App("index.html".into()))
                    .title("AskAI")
                    .inner_size(860.0, 720.0)
                    .min_inner_size(720.0, 640.0)
                    .resizable(true)
                    .visible(false);

            #[cfg(target_os = "macos")]
            {
                main_window_builder = main_window_builder
                    .title("")
                    .title_bar_style(tauri::TitleBarStyle::Overlay);
            }

            main_window_builder.build().unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::all_chats,
            commands::load_chat,
            commands::new_chat,
            commands::update_chat,
            commands::delete_chat,
            commands::send_message,
            commands::resend_message,
            commands::set_api_key,
            commands::check_api_key,
            commands::get_settings,
            commands::get_theme,
            commands::set_theme,
            commands::set_proxy,
            commands::get_proxy,
            commands::has_api_key,
            commands::get_locale,
            commands::set_locale,
            commands::save_as_markdown,
            commands::all_prompts,
            commands::create_prompt,
            commands::update_prompt,
            commands::delete_prompt,
            commands::load_prompt,
            commands::show_window,
            commands::debug_log,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}