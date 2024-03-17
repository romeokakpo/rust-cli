use std::{
    fs,
    io::{self, stdout, Write},
    ops::ControlFlow,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Roméo KAKPO, romeokakpo3@gmail.com")]
#[command(version =  env!("CARGO_PKG_VERSION"))]
#[command(name = "echo",about = "Echo the STRING(s) to standard output", long_about=None)]
#[command(after_help = fs::read_to_string("echo.txt").unwrap_or_else(|_| String::new()))]
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
    #[arg(short = 'e', overrides_with = "disable_backslash_escape")]
    enable_backslash_escape: bool,
    ///disable interpretation of backslash escapes (default)
    #[arg(short = 'E')]
    disable_backslash_escape: bool,
}

fn main() {
    let cli = Cli::parse();
    execute(cli).unwrap_or_else(|error| println!("{error}"));
}

fn execute(cli: Cli) -> io::Result<()> {
    let mut lock = stdout().lock();

    for (i, input) in cli.string.iter().enumerate() {
        if i > 0 {
            write!(lock, " ")?;
        }
        if cli.enable_backslash_escape {
            if print_escaped(input, &mut lock)?.is_break() {
                return Ok(());
            };
        } else {
            write!(lock, "{input}")?;
        }
    }

    //Alsways print newline, but if flag -n is present don't print
    if !cli.no_newline {
        writeln!(lock)?;
    }
    Ok(())
}

fn print_escaped(input: &str, mut lock: impl Write) -> io::Result<ControlFlow<()>> {
    let mut iter = input.chars().peekable();
    while let Some(c) = iter.next() {
        //If char is not \
        if c != '\\' {
            write!(lock, "{c}")?;
            continue;
        }

        //If char is backslash see the next
        if let Some(c) = iter.next() {
            let unescaped = match c {
                '\\' => '\\',
                'a' => '\x07',
                'b' => '\x08',
                'c' => return Ok(ControlFlow::Break(())),
                'e' => '\x1B',
                'f' => '\x0C',
                'n' => '\x0A',
                'r' => '\r',
                't' => '\t',
                'v' => '\x0b',
                '0' => c,
                'x' => c,
                other => {
                    write!(lock, "\\")?;
                    other
                }
            };
            write!(lock, "{unescaped}")?;
        } else {
            write!(lock, "\\")?;
        }
    }
    Ok(ControlFlow::Continue(()))
}
