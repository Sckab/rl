use std::ffi::OsString;
use std::io;
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
