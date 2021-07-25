use std::u8;

use failure::{Error, bail};

#[derive(Debug, Default)]
pub struct Vernam {
    pub message: String,
    pub key: String,
}

impl Vernam {
    pub fn get_char_alphabet_index(&self, char: char) -> Result<u8, Error> {
        let letter_code: u8 = char as u8;

        if letter_code < 65 || letter_code > 90 {
            bail!("Invalid characted \"{}\", must be in A-Z range.", char)
        }

        Ok(letter_code - 65)
    }

    pub fn get_char_from_alphabet_index(&self, index: u8) -> char {
        let letter_index = index + 65;
        letter_index as char
    }

    pub fn encrypt(self) -> Result<String, Error> {
        let message_chars = self.message.chars();
        let mut key_chars = self.key.chars();
        let mut encrypted_message: Vec<char> = Vec::new();

        for message_char in message_chars {
            let message_char_index = self.get_char_alphabet_index(message_char)?;
            let key_char = key_chars.next().unwrap();
            let key_char_index = self.get_char_alphabet_index(key_char)?;
            let mut sum = message_char_index + key_char_index;

            if sum > 25 {
                sum -= 26;
            }

            encrypted_message.push(self.get_char_from_alphabet_index(sum));
        }

        Ok(encrypted_message.into_iter().collect())
    }

    pub fn decrypt(self) -> Result<String, Error> {
        let message_chars = self.message.chars();
        let mut key_chars = self.key.chars();
        let mut decrypted_message: Vec<char> = Vec::new();

        for message_char in message_chars {
            let mut message_char_index = self.get_char_alphabet_index(message_char)?;
            let key_char = key_chars.next().unwrap();
            let key_char_index = self.get_char_alphabet_index(key_char)?;

            if key_char_index > message_char_index {
                message_char_index += 26;
            }

            let sum = message_char_index - key_char_index;

            decrypted_message.push(self.get_char_from_alphabet_index(sum));
        }

        Ok(decrypted_message.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::Vernam;

    #[test]
    fn encrypt_works() {
        let v = Vernam {
            message: String::from("DEMAINCESTMONANNIVERSAIREDEMARIAGE"),
            key: String::from("AHKJDJELKPOIHROPHGLERTOIJHQTHGOLWHFD"),
        };

        assert_eq!(v.encrypt().unwrap(), "DLWJLWGPCIAWURBCPBPVJTWZNKUFHXWLCL");
    }

    #[test]
    fn decrypt_works() {
        let v = Vernam {
            message: String::from("DLWJLWGPCIAWURBCPBPVJTWZNKUFHXWLCL"),
            key: String::from("AHKJDJELKPOIHROPHGLERTOIJHQTHGOLWHFD"),
        };

        assert_eq!(v.decrypt().unwrap(), "DEMAINCESTMONANNIVERSAIREDEMARIAGE");
    }
}
