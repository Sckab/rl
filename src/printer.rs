use std::ffi::OsString;

use comfy_table::*;
use comfy_table::{Table, presets::NOTHING};
use file_owner::PathExt;

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

pub fn print(options: crate::cli::Args) -> anyhow::Result<()> {
    let mut table = Table::new();

    if options.long {
        table.load_style(NOTHING);
        table.set_header(vec!["Permissions", "Size", "User", "Group", "Name"]);

        let column = table.column_iter_mut().next().unwrap();
        column.set_padding((0, 1));
    }

    for item in std::fs::read_dir(options.path)? {
        let item = item?;

        let name = pretty_name(item.path(), options.icons)?;

        if options.long {
            let permissions = crate::utility::get_permission_string(item.metadata()?);
            let size = crate::utility::get_size_formatted(item.path())?;
            let owner = item
                .path()
                .owner()
                .unwrap()
                .name()?
                .unwrap_or(String::from("Unkown"));
            let group = item
                .path()
                .group()
                .unwrap()
                .name()?
                .unwrap_or(String::from("Unkown"));

            table.add_row(vec![
                Cell::new(permissions),
                Cell::new(format!("{} {}", size.0, size.1)).set_alignment(CellAlignment::Right),
                Cell::new(owner),
                Cell::new(group),
                Cell::new(name.to_string_lossy().into_owned()),
            ]);
        } else if options.one_line {
            println!("{}", name.display());
        } else {
            print!("{}  ", name.display());
        }
    }

    if options.long {
        println!("{table}")
    } else if !options.one_line {
        println!();
    }

    Ok(())
}
