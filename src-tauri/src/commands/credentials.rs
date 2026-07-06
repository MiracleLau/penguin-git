use crate::credentials;

#[tauri::command]
pub fn list_credentials() -> Result<Vec<credentials::Credential>, String> {
    credentials::list_credentials()
}

#[tauri::command]
pub fn save_credential(url: String, username: String, password: String) -> Result<(), String> {
    credentials::save_credential(&url, &username, &password)
}

#[tauri::command]
pub fn remove_credential(url: String) -> Result<(), String> {
    credentials::remove_credential(&url)
}
