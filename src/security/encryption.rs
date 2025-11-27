//! GPG-based encryption for MateriaTrack
//!
//! Provides transparent encryption/decryption using GPG.

use crate::error::{ConfigError, Result};
use crate::security::SecureStorage;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const GPG_BINARY: &str = "gpg2";
const GPG_FALLBACK: &str = "gpg";

pub struct GpgEncryption {
    key_id: String,
    armor: bool,
    gpg_binary: PathBuf,
}

impl GpgEncryption {
    pub fn new(key_id: &str) -> Result<Self> {
        if key_id.is_empty() {
            return Err(crate::error::Error::Config(ConfigError::MissingField(
                "GPG key ID required for encryption".into(),
            )));
        }

        let gpg_binary = find_gpg_binary().ok_or_else(|| {
            crate::error::Error::Config(ConfigError::InvalidPath(
                "GPG binary not found (tried gpg2, gpg)".into(),
            ))
        })?;

        Ok(Self {
            key_id: key_id.to_string(),
            armor: true,
            gpg_binary,
        })
    }

    pub fn with_armor(mut self, armor: bool) -> Self {
        self.armor = armor;
        self
    }

    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    pub fn verify_key(&self) -> Result<KeyInfo> {
        let output = Command::new(&self.gpg_binary)
            .args(["--list-keys", "--with-colons", &self.key_id])
            .output()
            .map_err(|e| {
                crate::error::Error::Config(ConfigError::EncryptionError(format!(
                    "Failed to verify GPG key: {}",
                    e
                )))
            })?;

        if !output.status.success() {
            return Err(crate::error::Error::Config(ConfigError::EncryptionError(
                format!("GPG key not found: {}", self.key_id),
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut key_info = KeyInfo {
            key_id: self.key_id.clone(),
            user_id: String::new(),
            creation_date: String::new(),
            expiration_date: None,
            can_encrypt: false,
        };

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() > 9 {
                match parts[0] {
                    "pub" => {
                        key_info.creation_date = parts.get(5).unwrap_or(&"").to_string();
                        if let Some(exp) = parts.get(6) {
                            if !exp.is_empty() {
                                key_info.expiration_date = Some(exp.to_string());
                            }
                        }
                        if let Some(caps) = parts.get(11) {
                            key_info.can_encrypt = caps.contains('e') || caps.contains('E');
                        }
                    }
                    "uid" => {
                        if key_info.user_id.is_empty() {
                            key_info.user_id = parts.get(9).unwrap_or(&"").to_string();
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(key_info)
    }

    pub fn encrypt_file<P: AsRef<Path>>(&self, input: P, output: P) -> Result<()> {
        let mut args = vec![
            "--encrypt",
            "--recipient",
            &self.key_id,
            "--output",
        ];

        let output_str = output.as_ref().to_string_lossy().to_string();
        args.push(&output_str);

        if self.armor {
            args.push("--armor");
        }

        let input_str = input.as_ref().to_string_lossy().to_string();
        args.push(&input_str);

        let result = Command::new(&self.gpg_binary)
            .args(&args)
            .output()
            .map_err(|e| {
                crate::error::Error::Config(ConfigError::EncryptionError(format!(
                    "Failed to encrypt file: {}",
                    e
                )))
            })?;

        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(crate::error::Error::Config(ConfigError::EncryptionError(
                format!("GPG encryption failed: {}", stderr),
            )));
        }

        Ok(())
    }

    pub fn decrypt_file<P: AsRef<Path>>(&self, input: P, output: P) -> Result<()> {
        let output_str = output.as_ref().to_string_lossy().to_string();
        let input_str = input.as_ref().to_string_lossy().to_string();

        let result = Command::new(&self.gpg_binary)
            .args(["--decrypt", "--output", &output_str, &input_str])
            .output()
            .map_err(|e| {
                crate::error::Error::Config(ConfigError::EncryptionError(format!(
                    "Failed to decrypt file: {}",
                    e
                )))
            })?;

        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(crate::error::Error::Config(ConfigError::EncryptionError(
                format!("GPG decryption failed: {}", stderr),
            )));
        }

        Ok(())
    }

    fn run_gpg_pipe(&self, args: &[&str], input: &[u8]) -> Result<Vec<u8>> {
        let mut child = Command::new(&self.gpg_binary)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                crate::error::Error::Config(ConfigError::EncryptionError(format!(
                    "Failed to spawn GPG: {}",
                    e
                )))
            })?;

        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(input).map_err(|e| {
                crate::error::Error::Config(ConfigError::EncryptionError(format!(
                    "Failed to write to GPG stdin: {}",
                    e
                )))
            })?;
        }

        let output = child.wait_with_output().map_err(|e| {
            crate::error::Error::Config(ConfigError::EncryptionError(format!(
                "GPG process failed: {}",
                e
            )))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::error::Error::Config(ConfigError::EncryptionError(
                format!("GPG operation failed: {}", stderr),
            )));
        }

        Ok(output.stdout)
    }
}

impl SecureStorage for GpgEncryption {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut args = vec!["--encrypt", "--recipient", &self.key_id];
        if self.armor {
            args.push("--armor");
        }
        self.run_gpg_pipe(&args, data)
    }

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.run_gpg_pipe(&["--decrypt"], data)
    }

    fn is_available(&self) -> bool {
        self.verify_key().is_ok()
    }
}

#[derive(Debug, Clone)]
pub struct KeyInfo {
    pub key_id: String,
    pub user_id: String,
    pub creation_date: String,
    pub expiration_date: Option<String>,
    pub can_encrypt: bool,
}

impl KeyInfo {
    pub fn display(&self) -> String {
        let expiry = self
            .expiration_date
            .as_ref()
            .map(|e| format!(" (expires: {})", e))
            .unwrap_or_default();

        format!(
            "Key: {}\nUser: {}\nCreated: {}{}\nCan encrypt: {}",
            self.key_id, self.user_id, self.creation_date, expiry, self.can_encrypt
        )
    }
}

pub fn is_gpg_available() -> bool {
    find_gpg_binary().is_some()
}

pub fn find_gpg_binary() -> Option<PathBuf> {
    for binary in [GPG_BINARY, GPG_FALLBACK] {
        if Command::new(binary)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Some(PathBuf::from(binary));
        }
    }
    None
}

pub fn list_secret_keys() -> Result<Vec<KeyInfo>> {
    let gpg = find_gpg_binary().ok_or_else(|| {
        crate::error::Error::Config(ConfigError::InvalidPath("GPG not found".into()))
    })?;

    let output = Command::new(&gpg)
        .args(["--list-secret-keys", "--with-colons"])
        .output()
        .map_err(|e| {
            crate::error::Error::Config(ConfigError::EncryptionError(format!(
                "Failed to list keys: {}",
                e
            )))
        })?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut keys = Vec::new();
    let mut current_key: Option<KeyInfo> = None;

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "sec" => {
                if let Some(key) = current_key.take() {
                    keys.push(key);
                }
                current_key = Some(KeyInfo {
                    key_id: parts.get(4).unwrap_or(&"").to_string(),
                    user_id: String::new(),
                    creation_date: parts.get(5).unwrap_or(&"").to_string(),
                    expiration_date: parts.get(6).filter(|s| !s.is_empty()).map(|s| s.to_string()),
                    can_encrypt: parts
                        .get(11)
                        .map(|c| c.contains('e') || c.contains('E'))
                        .unwrap_or(false),
                });
            }
            "uid" => {
                if let Some(ref mut key) = current_key {
                    if key.user_id.is_empty() {
                        key.user_id = parts.get(9).unwrap_or(&"").to_string();
                    }
                }
            }
            _ => {}
        }
    }

    if let Some(key) = current_key {
        keys.push(key);
    }

    Ok(keys)
}

pub struct PasswordEncryption {
    cipher: String,
}

impl PasswordEncryption {
    pub fn new() -> Self {
        Self {
            cipher: "AES256".to_string(),
        }
    }

    pub fn with_cipher(mut self, cipher: impl Into<String>) -> Self {
        self.cipher = cipher.into();
        self
    }
}

impl Default for PasswordEncryption {
    fn default() -> Self {
        Self::new()
    }
}

impl SecureStorage for PasswordEncryption {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let gpg = find_gpg_binary().ok_or_else(|| {
            crate::error::Error::Config(ConfigError::InvalidPath("GPG not found".into()))
        })?;

        let mut child = Command::new(&gpg)
            .args([
                "--symmetric",
                "--cipher-algo",
                &self.cipher,
                "--armor",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                crate::error::Error::Config(ConfigError::EncryptionError(e.to_string()))
            })?;

        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(data).map_err(|e| {
                crate::error::Error::Config(ConfigError::EncryptionError(e.to_string()))
            })?;
        }

        let output = child.wait_with_output().map_err(|e| {
            crate::error::Error::Config(ConfigError::EncryptionError(e.to_string()))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::error::Error::Config(ConfigError::EncryptionError(
                stderr.to_string(),
            )));
        }

        Ok(output.stdout)
    }

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let gpg = find_gpg_binary().ok_or_else(|| {
            crate::error::Error::Config(ConfigError::InvalidPath("GPG not found".into()))
        })?;

        let mut child = Command::new(&gpg)
            .args(["--decrypt"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                crate::error::Error::Config(ConfigError::EncryptionError(e.to_string()))
            })?;

        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(data).map_err(|e| {
                crate::error::Error::Config(ConfigError::EncryptionError(e.to_string()))
            })?;
        }

        let output = child.wait_with_output().map_err(|e| {
            crate::error::Error::Config(ConfigError::EncryptionError(e.to_string()))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::error::Error::Config(ConfigError::EncryptionError(
                stderr.to_string(),
            )));
        }

        Ok(output.stdout)
    }

    fn is_available(&self) -> bool {
        is_gpg_available()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_gpg_binary() {
        // This test may fail if GPG is not installed
        let result = find_gpg_binary();
        // Just verify it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_is_gpg_available() {
        let _ = is_gpg_available();
    }

    #[test]
    fn test_key_info_display() {
        let info = KeyInfo {
            key_id: "ABC123".into(),
            user_id: "Test User".into(),
            creation_date: "2024-01-01".into(),
            expiration_date: Some("2025-01-01".into()),
            can_encrypt: true,
        };

        let display = info.display();
        assert!(display.contains("ABC123"));
        assert!(display.contains("Test User"));
    }

    #[test]
    fn test_password_encryption_new() {
        let enc = PasswordEncryption::new();
        assert_eq!(enc.cipher, "AES256");
    }
}
