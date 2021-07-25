use std::process;

#[macro_use]
extern crate clap;
use clap::App;

mod lib;

#[macro_use]
extern crate failure;

use failure::Error;
use lib::Vernam;

fn app() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("encrypt", Some(submatches)) => {
            let message = submatches.value_of("message").unwrap().to_string();
            let key = submatches.value_of("key").unwrap().to_string();

            if key.len() < message.len() {
                bail!("Key length must be equal or greater than the message")
            }

            let v = Vernam {
                message,
                key,
            };
            let encrypted_message = v.encrypt().unwrap();
            println!("{}", encrypted_message);
        }
        ("decrypt", Some(submatches)) => {
            let message = submatches.value_of("message").unwrap().to_string();
            let key = submatches.value_of("key").unwrap().to_string();

            if key.len() < message.len() {
                bail!("Key length must be equal or greater than the message")
            }

            let v = Vernam {
                message,
                key,
            };

            let decrypted_message = v.decrypt().unwrap();
            println!("{}", decrypted_message);
        }
        _ => {
            println!("Command not implemented");
        }
    };

    Ok(())
}


fn main() {
    process::exit(match app() {
        Ok(_) => 0,
        Err(err) => {
            println!("{}", err.to_string());
            1
        }
    });
}
