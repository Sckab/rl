use std::fs;
use std::path;

use clap::{
    Arg, ArgAction,
    builder::{Styles, styling::AnsiColor},
    command, value_parser,
};

fn main() -> anyhow::Result<()> {
    let mut cli = command!().styles(
        Styles::styled()
            .header(AnsiColor::Green.on_default().bold())
            .usage(AnsiColor::Green.on_default().bold())
            .literal(AnsiColor::Cyan.on_default())
            .placeholder(AnsiColor::Yellow.on_default()),
    );

    cli = cli.arg(
        Arg::new("path")
            .help("Path to work on.")
            .default_value(".")
            .value_parser(value_parser!(path::PathBuf)),
    );

    cli = cli.arg(
        Arg::new("one-line")
            .short('1')
            .long("one-line")
            .help("Display one entry per line.")
            .required(false)
            .action(ArgAction::SetTrue),
    );

    let matches = cli.get_matches();

    let path = matches.get_one::<path::PathBuf>("path").unwrap();
    let one_line = matches.get_one::<bool>("one-line").unwrap().to_owned();

    for paths in fs::read_dir(path)? {
        if one_line {
            println!("{}", paths?.file_name().display());
        } else {
            print!("{}  ", paths?.file_name().display());
        }
    }

    if !one_line {
        println!();
    }

    Ok(())
}
