use std::fs::File;
use std::io::{self, Read};

use crate::algorithm::Algorithm;

pub fn hash_file(path: &std::path::Path, algo: Algorithm) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(hash_bytes(&buffer, algo))
}

pub fn hash_stdin(algo: Algorithm) -> anyhow::Result<String> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    Ok(hash_bytes(&buffer, algo))
}

fn hash_bytes(data: &[u8], algo: Algorithm) -> String {
    match algo {
        Algorithm::Sha256 => {
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha512 => {
            use sha2::{Digest, Sha512};
            let mut hasher = Sha512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha1 => {
            use sha1::{Digest, Sha1};
            let mut hasher = Sha1::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Md5 => {
            use md5::{Digest, Md5};
            let mut hasher = Md5::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Blake2b => {
            use blake2::Blake2b512;
            use blake2::Digest;
            let mut hasher = Blake2b512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
    }
}