mod commands;
mod ssh;
mod git;
mod config;
mod settings;
mod credentials;
use commands::git_local::WatcherHandle;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(WatcherHandle(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            commands::system::check_git_installed,
            commands::system::get_git_version,
            commands::ssh::check_ssh_key,
            commands::ssh::generate_ssh_key,
            commands::ssh::get_public_key,
            commands::ssh::copy_public_key,
            commands::ssh::open_platform_ssh_settings,
            commands::ssh::test_ssh_connection,
            commands::project::is_git_repo,
            commands::project::init_repo,
            commands::project::open_project,
            commands::project::get_recent_projects,
            commands::project::save_recent_project,
            commands::git_local::get_status,
            commands::git_local::get_diff,
            commands::git_local::stage_files,
            commands::git_local::stage_all,
            commands::git_local::unstage_files,
            commands::git_local::commit,
            commands::git_local::get_log,
            commands::git_local::start_watching,
            commands::git_local::stop_watching,
            commands::git_local::get_ahead_behind,
            commands::git_local::reset_to_commit,
            commands::git_local::revert_to_commit,
            commands::git_local::discard_changes,
            commands::git_remote::add_remote,
            commands::git_remote::remove_remote,
            commands::git_remote::push,
            commands::git_remote::pull,
            commands::git_remote::fetch,
            commands::git_remote::get_remote_url,
            commands::settings::get_settings,
            commands::settings::set_settings,
            commands::settings::get_git_global_config,
            commands::settings::set_git_global_config,
            commands::settings::detect_git_path,
            commands::credentials::list_credentials,
            commands::credentials::save_credential,
            commands::credentials::remove_credential,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
