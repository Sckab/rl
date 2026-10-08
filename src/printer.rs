pub fn print(options: crate::cli::Args) -> Result<(), std::io::Error> {
    for item in std::fs::read_dir(options.path)? {
        if options.one_line {
            println!("{}", item?.file_name().display());
        } else {
            print!("{}  ", item?.file_name().display());
        }
    }

    if !options.one_line {
        println!();
    }

    Ok(())
}
