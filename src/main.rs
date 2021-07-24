#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("encrypt", Some(_sub)) => {
        }
        ("decrypt", Some(_sub)) => {
        }
        _ => {
            println!("Command not implemented");
        }
    };
}
