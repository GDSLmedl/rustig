use clap::{Command};
use std::{env, fs};

const RUSTIG_DIR: &str = ".rustig";

fn cli() -> Command<'static> {
    Command::new("rustig")
        .about(r#"
        ____  _   _ ____ _____ ___ ____ 
       |  _ \| | | / ___|_   _|_ _/ ___|
       | |_) | | | \___ \ | |  | | |  _ 
       |  _ <| |_| |___) || |  | | |_| |
       |_| \_\\___/|____/ |_| |___\____|
       
       A minimalist Git CLI in Rust
       "#)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("Initialize directory for rustig")
        )
}

fn init_cmd() -> std::io::Result<()> {
    let _result = fs::create_dir(RUSTIG_DIR);
    let current_dir = env::current_dir()?;
    println!("Initialized empty rustig repository in {}/{}", current_dir.display(), RUSTIG_DIR);
    Ok(())
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _sub_matches)) => {
            init_cmd();
        }
        _ => unreachable!(),
    }
}
