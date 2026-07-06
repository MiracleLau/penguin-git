use rand::rngs::OsRng;
use serde::Serialize;
use ssh_key::{Algorithm, LineEnding, PrivateKey};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SshKeyInfo {
    pub exists: bool,
    pub public_key: Option<String>,
    pub path: Option<String>,
}

pub fn ssh_dir() -> PathBuf {
    dirs::home_dir().expect("home dir not found").join(".ssh")
}

pub fn find_existing_key() -> Option<PathBuf> {
    let dir = ssh_dir();
    for name in &["id_ed25519", "id_ed25519_penguin", "id_rsa", "id_ecdsa"] {
        let path = dir.join(name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

pub fn generate_ed25519_key(comment: &str) -> Result<SshKeyInfo, String> {
    let dir = ssh_dir();
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let key_path = if dir.join("id_ed25519").exists() {
        dir.join("id_ed25519_penguin")
    } else {
        dir.join("id_ed25519")
    };

    let private = PrivateKey::random(&mut OsRng, Algorithm::Ed25519).map_err(|e| e.to_string())?;

    let private_openssh = private
        .to_openssh(LineEnding::LF)
        .map_err(|e| e.to_string())?;

    let public_openssh = private
        .public_key()
        .to_openssh()
        .map_err(|e| e.to_string())?;

    // Append comment to public key
    let public_key_str = format!("{} {}", public_openssh.trim(), comment);

    fs::write(&key_path, &*private_openssh).map_err(|e| e.to_string())?;
    fs::write(key_path.with_extension("pub"), &public_key_str).map_err(|e| e.to_string())?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&key_path, fs::Permissions::from_mode(0o600)).ok();
        fs::set_permissions(
            key_path.with_extension("pub"),
            fs::Permissions::from_mode(0o644),
        )
        .ok();
    }

    Ok(SshKeyInfo {
        exists: true,
        public_key: Some(public_key_str.trim().to_string()),
        path: Some(key_path.to_string_lossy().to_string()),
    })
}

pub fn read_public_key(path: &std::path::Path) -> Result<String, String> {
    let pub_path = path.with_extension("pub");
    if !pub_path.exists() {
        return Err("公钥文件不存在".into());
    }
    fs::read_to_string(&pub_path)
        .map(|s| s.trim().to_string())
        .map_err(|e| e.to_string())
}
