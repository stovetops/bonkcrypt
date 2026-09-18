//based mostly on the Megabonk Save Decryptor by GuardianN06, rewritten in Rust as a learning exercise for myself
//https://github.com/GuardianN06/Megabonk-Save-Decryptor/ for the original python version

use aes::cipher::generic_array::GenericArray;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{engine::general_purpose, Engine as _};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
type AesEnc = cbc::Encryptor<aes::Aes256>;
type AesDec = cbc::Decryptor<aes::Aes256>;

//32bit key and 16bit IV for Megabonk as of latest update 1.0.17
const KEY: [u8; 32] = hex_literal::hex!("D940840D5AE7C7907B092437BC0C5B44AAF70E273E12D0FB4DA2B8C767CC911D");
const IV: [u8; 16] = hex_literal::hex!("37864EF15C24BC0ACBC60E3978EF1F06");

fn prompt(msg: &str) -> bool {
    print!("{msg}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().eq_ignore_ascii_case("y")
}

fn crypt_file(path: &Path, encrypt: bool) {
    let data = match fs::read(path) {
        Ok(d) => d,
        Err(e) => { 
            eprintln!("Failed to read {}: {e}", path.display());
            return;
        }
    };
    let is_json = serde_json::from_slice::<serde_json::Value>(&data).is_ok();
    let filename = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();

    if encrypt && !is_json {
        if !prompt(&format!(
            "{filename} probably already encrypted. Continue encrypting?\n[WARNING, CONTINUING MAY CORRUPT YOUR FILE]\n(y/n): "
        )) {
            println!("Skipped {}, already encrypted.", path.display());
            return;
        }
    } else if !encrypt && is_json {
        if !prompt(&format!(
            "{filename} probably already decrypted. Continue decrypting?\n[WARNING, CONTINUING MAY CORRUPT YOUR FILE]\n(y/n): "
        )) {
            println!("Skipped {}, already decrypted.", path.display());
            return;
        }
    }

    let out = if encrypt {
        let cipher = AesEnc::new(GenericArray::from_slice(&KEY), GenericArray::from_slice(&IV));
        let encrypted = cipher.encrypt_padded_vec_mut::<Pkcs7>(&data);
        general_purpose::STANDARD.encode(encrypted).into_bytes()
    } else {
        let decoded = match general_purpose::STANDARD.decode(&data) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Base64 decode failed for {}: {e}", path.display());
                return;
            }
        };
        let cipher = AesDec::new(GenericArray::from_slice(&KEY), GenericArray::from_slice(&IV));
        match cipher.decrypt_padded_vec_mut::<Pkcs7>(&decoded) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Decryption failed for {}: {e}", path.display());
                return;
            }
        }
    };

    if let Err(e) = fs::write(path, &out) {
        eprintln!("Failed to write {}: {e}", path.display());
        return;
    }

    println!(
        "{} {}",
        if encrypt { "Encrypted" } else { "Decrypted" },
        path.display()
    );
}

fn find_saves(choice: &[String; 2]) -> Vec<PathBuf> {
    let base = if cfg!(target_os = "windows") {
        PathBuf::from(std::env::var("USERPROFILE").unwrap_or_default()).join("AppData/LocalLow/Ved/Megabonk/Saves/CloudDir")
    } else {
        PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".config/unity3d/Ved/Megabonk/Saves/CloudDir")
    };
    //println!("{}", base.display());
    let mut results = Vec::new();
    let entries = match fs::read_dir(&base) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to read {}: {e}", base.display());
            return results;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() { continue; }
        let filename = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        if !filename.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        for fname in choice {
            let f = path.join(fname);
            if f.exists() {
                results.push(f);
            }
        }
    }
    results
}

fn main() {
    println!("Encrypting or Decrypting? (E/D)");
    io::stdout().flush().unwrap();
    let mut action = String::new();
    io::stdin().read_line(&mut action).unwrap();
    let encrypt = action.trim().eq_ignore_ascii_case("e");

    let mut action = String::new();
    println!("Which files?\nProgression, Stats, or Both (P/S/B)");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut action).unwrap();
    let choices = match action.trim().to_lowercase().as_str() {
        "p" => &["progression.json".into(), "".into()],
        "s" => &["stats.json".into(), "".into()],
        "b" => &["progression.json".into(), "stats.json".into()],
        _ => &["".into(), "".into()],
    };
    for file in find_saves(choices) {
        //println!("{}", file.display());
        if !file.is_empty() { crypt_file(&file, encrypt); }
    }
}
