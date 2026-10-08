use std::fs;

fn main() -> anyhow::Result<()> {
    for paths in fs::read_dir(std::env::current_dir()?)? {
        print!("{}  ", paths?.file_name().display());
    }

    println!();

    Ok(())
}
