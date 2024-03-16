use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version =  env!("CARGO_PKG_VERSION"))]
#[command(author = "Roméo KAKPO")]
#[command(name = "echo",about = "Echo the STRING(s) to standard output", long_about=None)]
//trailing var arg pour accepter les options même après la dernière valeur
//allow hyphen values pour accepter les tirets dans la valeur du string
#[command(trailing_var_arg = true, allow_hyphen_values = true)]
struct Cli {
    ///Input String
    string: Vec<String>,
    ///do not output the trailing newline
    #[arg(short = 'n')]
    no_newline: bool,
    ///enable interpretation of backslash escapes
    #[arg(short = 'e', overrides_with = "disabe_backslash_escape")]
    enable_backslash_escape: bool,
    ///disable interpretation of backslash escapes (default)
    #[arg(short = 'E')]
    disabe_backslash_escape: bool,
}

fn main() {
    let cli = Cli::parse();
    fs::write("echo.md", "Hi");
    dbg!(cli);
}
