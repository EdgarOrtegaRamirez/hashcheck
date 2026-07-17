use crate::algorithm::Algorithm;
use std::fs;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ChecksumEntry {
    pub expected_hash: String,
    pub filename: String,
}

pub fn verify_checksum_file(
    checksum_file: &PathBuf,
    base: Option<PathBuf>,
) -> anyhow::Result<(Vec<(PathBuf, String, bool)>, Vec<(PathBuf, String)>)> {
    let content = fs::read_to_string(checksum_file)?;
    let mut results: Vec<(PathBuf, String, bool)> = Vec::new();
    let mut errors: Vec<(PathBuf, String)> = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse SHA256SUMS / MD5SUMS format: "hash  filename" or "hash *filename"
        // Hash is always 32/40/64/128 hex chars, followed by whitespace, then filename
        let parts: Vec<&str> = line.splitn(2, char::is_whitespace).collect();
        if parts.len() < 2 {
            continue;
        }

        let expected_hash = parts[0].to_string();
        // Trim leading whitespace from filename part and handle binary mode indicator (*)
        let filename_part = parts[1].trim_start();
        let has_star = filename_part.starts_with('*');
        let filename = filename_part[if has_star { 1 } else { 0 }..].trim_end().to_string();

        let filepath = if let Some(ref base_path) = base {
            base_path.join(&filename)
        } else {
            PathBuf::from(&filename)
        };

        if !filepath.exists() {
            errors.push((filepath.clone(), "file not found".to_string()));
            continue;
        }

        let actual_hash = match expected_hash.len() {
            32 => crate::hasher::hash_file(&filepath, Algorithm::Md5),
            40 => crate::hasher::hash_file(&filepath, Algorithm::Sha1),
            64 => crate::hasher::hash_file(&filepath, Algorithm::Sha256),
            128 => crate::hasher::hash_file(&filepath, Algorithm::Blake2b),
            _ => {
                errors.push((filepath.clone(), format!("unknown hash length: {}", expected_hash.len())));
                continue;
            }
        };

        match actual_hash {
            Ok(hash) => {
                let ok = hash == expected_hash;
                results.push((filepath, hash, ok));
            }
            Err(e) => {
                errors.push((filepath.clone(), e.to_string()));
            }
        }
    }

    Ok((results, errors))
}