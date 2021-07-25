use std::process;

#[macro_use]
extern crate clap;
use clap::App;

mod lib;
use failure::Error;
use lib::Vernam;

fn app() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("encrypt", Some(submatches)) => {
            let v = Vernam {
                message: submatches.value_of("message").unwrap().to_string(),
                key: submatches.value_of("key").unwrap().to_string(),
            };
            let encrypted_message = v.encrypt().unwrap();
            println!("{}", encrypted_message);
        }
        ("decrypt", Some(submatches)) => {
            let v = Vernam {
                message: submatches.value_of("message").unwrap().to_string(),
                key: submatches.value_of("key").unwrap().to_string(),
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
