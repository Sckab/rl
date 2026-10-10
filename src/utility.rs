use std::ffi::OsString;
use std::fs::Metadata;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

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
