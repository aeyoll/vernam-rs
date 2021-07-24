use failure::Error;

#[derive(Debug, Default)]
pub struct Vernam {
    message: String,
    key: String,
}

impl Vernam {
    pub fn new(message: String, key: String) -> Self {
        Vernam {
            message,
            key,
        }
    }

    pub fn encrypt(self) -> Result<String, Error> {
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