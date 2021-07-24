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

    pub fn encrypt() -> Result<(), Error> {
        Ok(())
    }

    pub fn decrypt() -> Result<(), Error> {
        Ok(())
    }
}