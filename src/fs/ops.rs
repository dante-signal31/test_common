/// Module to perform filesystem typical operations, like copy or remove files.

use std::fs::{remove_file, copy};
use std::io;
use std::path::Path;

/// Delete an specific file.
///
/// # Parameters:
/// * file_path: &str with the absolute path to file.
///
/// # Returns:
/// Returns Ok(()) if sucessful and std::io::Error if not.
pub fn delete_file(file_path: &str)-> Result<(), io::Error>{
    remove_file(file_path)?;
    Ok(())
}

/// Delete all files set in given list.
///
/// # Parameters:
/// * files: Vector with filepath list to remove.
/// * ignore_missing: If true does not return an error if any of files actually does not exists.
///
/// # Returns:
/// Returns an io::Error if any file does not exists unless ignore_missing was true.
pub fn delete_files<T>(files: Vec<T>, ignore_missing: bool)-> Result<(), io::Error>
    where T: AsRef<str>{
    for file in files{
        let file_name = file.as_ref();
        if ignore_missing {
            let _ = delete_file(file_name);
        } else {
            let _ = delete_file(file_name)?;
        }
    }
    Ok(())
}

/// Copy an specific file.
///
/// # Parameters:
/// * source_file_path: &str with absolute pathname to original file.
/// * destination_file_path: &str with absolute pathname to copied file.
///
/// # Returns:
/// Returns an Ok(u64) with copied file size if operation was successful. Otherwise
/// it returns an io::Error.
pub fn copy_file(source_file_path: &str, destination_file_path: &str)-> Result<u64, io::Error>{
    Ok(copy(source_file_path, destination_file_path)?)
}

/// Copy all files in an given list to a given destination folder. Original file names
/// are kept untouched.
///
/// # Parameters:
/// * files: Vector with absolute file path names as &str.
/// * destination_folder_path: Absolute path name to folder where to copy files into.
///
/// # Returns:
/// Returns an io::Error if destination_folder does not exists or any to be copied file does not..
pub fn copy_files<T>(files: Vec<T>, destination_folder_path: &str)-> Result<(), io::Error>
    where T: AsRef<str> {
    for file in files{
        let file_name = file.as_ref();
        let path = Path::new(&file_name);
        if let Some(filename) = path.file_name() {
            let destination_filename = Path::new(destination_folder_path).join(filename);
            copy_file(file_name, destination_filename.as_path().to_str()
                .expect("Destination file name for copy has non valid unicode characters."))?;
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, tempdir, TempDir};

    #[test]
    fn test_delete_file() {
        let temp_file = NamedTempFile::new()
            .expect("Error creating temporal file for deletion test.");
        assert!(Path::new(temp_file.path()).exists());
        let _ = delete_file(temp_file.path().to_str()
            .expect("Error getting temporal file path for deletion test."));
        assert!(!Path::new(temp_file.path()).exists());
    }

    #[test]
    fn test_delete_existing_files() {
        let temp_file = NamedTempFile::new()
            .expect("Error creating temporal file for deletion test.");
        let temp_file2 = NamedTempFile::new()
            .expect("Error creating temporal file for deletion test.");
        assert!(Path::new(temp_file.path()).exists());
        assert!(Path::new(temp_file2.path()).exists());
        let files_to_delete: Vec<&str> = [temp_file.path().as_os_str().to_str().expect("Error getting path for temporal file1."),
            temp_file2.path().as_os_str().to_str().expect("Error getting path for temporal file1.")].to_vec();
        match delete_files(files_to_delete, false) {
            Ok(())=> assert!(true),
            Err(_)=> assert!(false, "Error deleting files.")
        };
        assert!(!Path::new(temp_file.path()).exists());
        assert!(!Path::new(temp_file2.path()).exists());
    }

    #[test]
    fn test_deleting_non_existing_files() {
        let temp_file = NamedTempFile::new()
            .expect("Error creating temporal file for deletion test.");
        assert!(Path::new(temp_file.path()).exists());
        let files_to_delete: Vec<&str> = [temp_file.path().as_os_str().to_str().expect("Error getting path for temporal file1."),
            "/tmp/123456789.nex"].to_vec();
        match delete_files(files_to_delete, false) {
            Ok(())=> assert!(false, "Deletion did not detect missing file."),
            Err(_)=> assert!(true, "Error deleting files.")
        };
        assert!(!Path::new(temp_file.path()).exists());
        let temp_file = NamedTempFile::new()
            .expect("Error creating temporal file for deletion test.");
        assert!(Path::new(temp_file.path()).exists());
        let files_to_delete: Vec<&str> = [temp_file.path().as_os_str().to_str().expect("Error getting path for temporal file1."),
            "/tmp/123456789.nex"].to_vec();
        match delete_files(files_to_delete, true) {
            Ok(())=> assert!(true),
            Err(_)=> assert!(false, "Deletion did not ignore missing file")
        };
        assert!(!Path::new(temp_file.path()).exists());
    }

    #[test]
    fn test_copy_files() {
        let temp_file = NamedTempFile::new()
            .expect("Error creating temporal file for deletion test.");
        let temp_file2 = NamedTempFile::new()
            .expect("Error creating temporal file for deletion test.");
        let temp_folder: TempDir = tempdir()
            .expect("Error creating temporal folder.");
        match copy_files(
            [temp_file.path().to_str().expect("Error gettint temporal file pathname."),
                  temp_file2.path().to_str().expect("Error getting temporal file2")].to_vec(),
            temp_folder.path().to_str().expect("Error getting temporal folder path.")) {
            Ok(())=> assert!(true),
            Err(_)=> assert!(false, "Copy failed.")
        };
        let temp_file_name = temp_file.path().file_name()
            .expect("Error getting temporal file name.");
        let temp_file_name2 = temp_file2.path().file_name()
            .expect("Error getting temporal file2 name.");
        let temporal_file_name_path = temp_folder.path().join(temp_file_name);
        let temporal_file_name_path2 = temp_folder.path().join(temp_file_name2);
        assert!(temporal_file_name_path.exists());
        assert!(temporal_file_name_path2.exists());
    }
}