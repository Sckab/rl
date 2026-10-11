use std::ffi::OsString;
use std::fs::Metadata;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use anyhow::Result;

pub fn get_file_name<P>(path: P) -> Result<OsString, io::Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    // We can't lowercase the name because `OsStr` doesn't have a `.to_lowercase()` method and i
    // don't want to make it a `&str` as `OsStr` isn't guaranteed to be an UTF-8 string
    Ok(match path.file_name() {
        Some(n) => n.to_os_string(),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidFilename,
                format!("The filename of {} is not correct.", path.display()),
            ));
        }
    })
}

pub fn get_permission_string(metadata: Metadata) -> String {
    let mode = metadata.permissions().mode();
    let mut permissions = String::with_capacity(10);

    if metadata.is_dir() {
        permissions.push('d');
    } else if metadata.is_symlink() {
        permissions.push('l');
    } else if metadata.is_file() {
        permissions.push('-');
    } else {
        permissions.push('?'); // unknown file type
    }

    for (byte, character) in [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ] {
        if mode & byte != 0 {
            permissions.push(character);
        } else {
            permissions.push('-');
        }
    }

    permissions
}

fn get_size<P>(path: P) -> Result<u64, std::io::Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let metadata = path.symlink_metadata()?;

    let mut size: u64 = 0;

    if metadata.is_dir() {
        for item in std::fs::read_dir(path)? {
            let item_metadata = item?.metadata()?;

            if item_metadata.is_file() {
                size += item_metadata.len();
            }

            // we won't iterate into subdirectories as it's slow on large directories
        }
    } else {
        size = metadata.len();
    }

    Ok(size)
}

pub fn get_size_formatted<P>(path: P) -> Result<(String, String), std::io::Error>
where
    P: AsRef<Path>,
{
    let size = get_size(path)?;

    let extensions = ["B  ", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let unit = 1024.0;

    let mut bytes = size as f64;
    let mut i = 0;

    while bytes >= unit && i < extensions.len() - 1 {
        bytes /= unit;

        i += 1;
    }

    Ok((
        format!("{bytes:.2}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string(),
        extensions[i].to_string(),
    ))
}

pub fn get_time(metadata: Metadata) -> anyhow::Result<String> {
    let format = time::format_description::parse_owned::<3>(
        "[day]/[month]/[year] [hour]:[minute]:[second]",
    )?;

    let time = metadata.modified()?;

    Ok(time::OffsetDateTime::from(time).format(&format)?)
}
