use std::fs::write;
use std::fs::{self, create_dir_all};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use anyhow::{Error, Result};
use dirs::cache_dir;

use crate::RoxError;

/// This function checks if an item is located on a different device than the trash directory.
///
/// On Linux-based systems, files and directories are associated with specific devices.
/// When performing a move operation, the item is not physically copied but instead its
/// pointer is updated to the target location. However, this behavior only works when the
/// source and target are on the same device. If the source and target are on different
/// devices, the move operation would fail, as it cannot change the pointer across devices.
///
/// In this case, to avoid panic the function will create a temporary file in
/// users local cache directory and checks if the item given as argument and the temporary file
/// are on the same device with its metadata.
/// If not, it returns an error indicating that the operation would cross devices, which would
/// lead to a failure when attempting to move the file.
///
/// # Arguments
/// - `item`: The path of the item to check for cross-device movement.
///
/// # Returns
/// Returns `Ok(())` if the item and the temporary test file are on the same device,
/// or an error indicating a cross-device move attempt if they are not.
///
/// # Example
/// ```rust
/// use std::path::Path;
/// use roxide::check_cross_device;
///
/// let result = check_cross_device(Path::new("/path/to/file"));
/// match result {
///     Ok(_) => println!("The item is on the same device."),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
pub fn check_cross_device(item: &Path) -> Result<(), Error> {
    let item_metadata = fs::metadata(item)?.dev();

    let cache_dir = cache_dir()
        .ok_or(RoxError::CantFindCacheDir)?
        .join("roxide");

    create_dir_all(&cache_dir)?;
    write(
        cache_dir.join("state.txt"),
        "Just a file to check CrossesDevices Error.",
    )?;
    let file_in_device = cache_dir.join("state.txt").metadata()?.dev();
    // check if the devices are different
    if item_metadata != file_in_device {
        return Err(RoxError::CrossesDevices(item.to_path_buf()).into());
    }
    Ok(())
}

/// Checks if the current process is running as the root user.
///
/// This function reads the `/proc/self/status` file, which contains information
/// about the current process. It looks for the line starting with "Uid:",
/// which provides the User ID (UID) of the process. If the UID is `0`,
/// it indicates that the process is running as root (superuser).
///
/// # Returns
/// - `true` if the current process is running as root (UID = 0).
/// - `false` otherwise.
///
/// # Example
/// ```rust
/// use roxide::check_root;
///
/// if check_root() {
///     println!("The process is running as root.");
/// } else {
///     println!("The process is not running as root.");
/// }
/// ```
pub fn check_root() -> bool {
    if let Ok(status) = fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("Uid:") {
                let uid = line.split_whitespace().nth(1);
                return uid == Some("0");
            }
        }
    }
    false
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use std::fs::{create_dir_all, remove_dir_all};
    use std::path;

    use super::{check_cross_device, check_root};

    #[test]
    fn check_cross_device_test() {
        create_dir_all("trash/tests/check_cross_device_test/test_dir").unwrap();
        check_cross_device(path::Path::new(
            "trash/tests/check_cross_device_test/test_dir",
        ))
        .unwrap();

        let path = path::Path::new("/media/usb");
        if path.exists() {
            create_dir_all(path.join("test_dir")).unwrap();
            if check_cross_device(&path.join("test_dir")).is_ok() {
                panic!("check_cross_device function failed!");
            }
        }
        remove_dir_all("trash/tests/check_cross_device_test").unwrap();
    }

    #[test]
    fn check_root_test() {
        if path::Path::new("/proc/self/status").exists() {
            if check_root() {
                panic!("is root!");
            }
        } else {
            panic!("/proc/self/status not found!");
        }
    }
}
