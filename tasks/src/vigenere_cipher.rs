mod vigenere {
    pub fn encrypt(plaintext: &str, key: &str) -> String {
        let mut cipher = String::new();

        for (i, ch) in plaintext.chars().enumerate() {
            if ch.is_alphabetic() {
                let shift = (key.chars().nth(i % key.len()).unwrap() as u8 - b'A') % 26;
                let new_char = if ch.is_lowercase() {
                    ((ch as u8) + shift) % 26 + b'a'
                }  else {
                    ((ch as u8) + shift) % 26 + b'A'
                };
                cipher.push(new_char as char);
            } else {
                cipher.push(ch);
            }
        }
        cipher
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let mut text = String::new();
        for (i, ch) in ciphertext.chars().enumerate() {
            if ch.is_alphabetic() {
                let shift = (key.chars().nth(i % key.len()).unwrap() as u8 - b'A') % 26;
                let new_char = if ch.is_lowercase() {
                    ((ch as u8) - shift + 26) % 26 + b'a'
                } else {
                    ((ch as u8) - shift + 26) % 26 + b'A'
                };
                text.push(new_char as char);
            } else {
                text.push(ch);
            }
        }
        text
    }
}

use vigenere::*;

#[test]
fn test_encrypt() {
    encrypt("PVCDJG", "WHYRUST");
}

#[test]
fn test_decrypt() {
    let plaintext = "PVC12DJG";
    let key = "WHYRUST";
    let encrypted = encrypt(plaintext, key);

    assert_eq!(decrypt(&encrypted, key), plaintext);
}
