use serde::{Deserialize, Serialize};
use std::fs;
use crate::config;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
    pub url: String,
    pub username: String,
    // password field is serialized but not included in "list" responses
    pub password: Option<String>,
}

fn load_or_create_key() -> Result<[u8; 32], String> {
    let path = config::master_key_path();
    if path.exists() {
        let data = fs::read(&path).map_err(|e| e.to_string())?;
        if data.len() == 32 {
            let mut key = [0u8; 32];
            key.copy_from_slice(&data);
            return Ok(key);
        }
    }

    // Generate new key
    let mut key = [0u8; 32];
    use rand::RngCore;
    rand::rngs::OsRng.fill_bytes(&mut key);

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    fs::write(&path, &key).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).ok();
    }

    Ok(key)
}

fn load_encrypted() -> Result<Vec<Credential>, String> {
    let path = config::credentials_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let key_bytes = load_or_create_key()?;
    let data = fs::read(&path).map_err(|e| e.to_string())?;
    if data.is_empty() {
        return Ok(Vec::new());
    }
    if data.len() < 12 {
        return Err("凭据文件损坏".into());
    }

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|_| "凭据解密失败".to_string())?;

    serde_json::from_slice(&plaintext).map_err(|e| e.to_string())
}

fn save_encrypted(creds: &[Credential]) -> Result<(), String> {
    let path = config::credentials_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let key_bytes = load_or_create_key()?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let plaintext = serde_json::to_string(creds).map_err(|e| e.to_string())?;
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes())
        .map_err(|_| "凭据加密失败".to_string())?;

    let mut out = nonce.to_vec();
    out.extend_from_slice(&ciphertext);
    fs::write(&path, &out).map_err(|e| e.to_string())
}

pub fn list_credentials() -> Result<Vec<Credential>, String> {
    let creds = load_encrypted()?;
    // Return credentials without password for listing
    let safe: Vec<Credential> = creds.into_iter().map(|mut c| {
        c.password = None;
        c
    }).collect();
    Ok(safe)
}

pub fn save_credential(url: &str, username: &str, password: &str) -> Result<(), String> {
    let mut creds = load_encrypted()?;
    // Remove existing entry for same URL
    creds.retain(|c| c.url != url);
    creds.push(Credential {
        url: url.to_string(),
        username: username.to_string(),
        password: Some(password.to_string()),
    });
    save_encrypted(&creds)
}

pub fn remove_credential(url: &str) -> Result<(), String> {
    let mut creds = load_encrypted()?;
    creds.retain(|c| c.url != url);
    save_encrypted(&creds)
}

#[allow(dead_code)]
pub fn find_credential(url: &str) -> Option<Credential> {
    load_encrypted().ok()?.into_iter()
        .find(|c| url.starts_with(&c.url))
}
