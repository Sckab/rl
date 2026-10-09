use std::ffi::OsString;

fn pretty_name(path: std::path::PathBuf, icons: bool) -> Result<OsString, std::io::Error> {
    let mut name: OsString = OsString::new();

    if icons {
        name.push(crate::icons::get_icon(&path)? + " ");
    }

    name.push(crate::utility::get_file_name(&path)?);

    if path.is_dir() {
        name.push("/");
    }

    Ok(name)
}

pub fn print(options: crate::cli::Args) -> Result<(), std::io::Error> {
    for item in std::fs::read_dir(options.path)? {
        let name = pretty_name(item?.path(), options.icons)?;

        if options.one_line {
            println!("{}", name.display());
        } else {
            print!("{}  ", name.display());
        }
    }

    if !options.one_line {
        println!();
    }

    Ok(())
}
