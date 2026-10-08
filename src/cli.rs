use clap::{Parser, builder::styling::AnsiColor};

const STYLES: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(AnsiColor::Green.on_default().bold())
    .usage(AnsiColor::Green.on_default().bold())
    .literal(AnsiColor::Cyan.on_default())
    .placeholder(AnsiColor::Yellow.on_default());

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(styles = STYLES)]
pub struct Args {
    /// The path to work on.
    #[arg(default_value = ".")]
    pub path: std::path::PathBuf,

    /// Display one entry per line.
    #[arg(short = '1', long)]
    pub one_line: bool,
}
