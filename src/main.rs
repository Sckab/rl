use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = rl::cli::Args::parse();

    rl::printer::print(args)?;

    Ok(())
}
