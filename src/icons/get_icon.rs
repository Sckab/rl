use std::io;
use std::path::Path;

pub fn get_icon<P>(path: P) -> Result<String, io::Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let file_name = crate::utility::get_file_name(path)?;

    let icon: String;

    if path.is_dir() {
        icon = match super::mappings::DIR_NAMES.get(file_name.as_os_str()) {
            Some(i) => i.to_string(),
            None => "".to_string(),
        }
    } else {
        icon = match super::mappings::FILE_NAMES.get(file_name.as_os_str()) {
            Some(i) => i.to_string(),
            None => super::mappings::ICONS
                .get(path.extension().unwrap_or_default())
                .unwrap_or(&"")
                .to_string(),
        };
    }

    Ok(icon)
}
