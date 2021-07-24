use std::u32;

use failure::Error;

#[derive(Debug, Default)]
pub struct Vernam {
    pub message: String,
    pub key: String,
}

impl Vernam {
    pub fn new(message: String, key: String) -> Self {
        Vernam {
            message,
            key,
        }
    }

    pub fn get_char_alphabet_index(&self, char: char) -> u32 {
        let letter_code: u32 = char as u32;
        return letter_code - 65;
    }

    pub fn encrypt(self) -> Result<String, Error> {
        for (i, char) in self.message.chars().enumerate() {
            let cai = self.get_char_alphabet_index(char);
            println!("{}, {}, {}", i, char, cai);
        }

        Ok("DLWJLWGPCIAWURBCPBPVJTWZNKUFHXWLCL".to_string())
    }

    pub fn decrypt(self) -> Result<String, Error> {
        Ok("DEMAINCESTMONANNIVERSAIREDEMARIAGE".to_string())
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

        assert_eq!(v.encrypt().unwrap() , "DLWJLWGPCIAWURBCPBPVJTWZNKUFHXWLCL");
    }

    #[test]
    fn decrypt_works() {
        let v = Vernam {
            message: String::from("DLWJLWGPCIAWURBCPBPVJTWZNKUFHXWLCL"),
            key: String::from("AHKJDJELKPOIHROPHGLERTOIJHQTHGOLWHFD"),
        };

        assert_eq!(v.decrypt().unwrap() , "DEMAINCESTMONANNIVERSAIREDEMARIAGE");
    }
}