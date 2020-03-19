/// Module to manage temporal files and folders.

use std::path::Path;
use tempfile::{tempdir, TempDir};

/// Context manager like struct to create temporal folder to perform tests inside.
///
/// TempDir type is stored in private attribute folder. TempDir removes generated temp folder
/// and its contents when it detects it es falling out of scope, So you do not need to remove
/// manually generated temp folder.
///
/// # Example
/// ```rust
/// use test_common::fs::tmp::TestEnvironment;
///
/// {
///     let test_folder = TestEnvironment::new();
///     let test_folder_path = test_folder.path();
///     // Do your operations in test folder.
/// } // Here test folder is automatically removed.
/// ```
pub struct TestEnvironment {
    folder: TempDir,
}

// TempDir automatically removes generated test folder, so implementing Drop trait is not needed.
impl TestEnvironment {
    #[must_use]
    pub fn new()-> Self {
        let temp_folder = tempdir().expect("Could not create a temporal test environment.");
        TestEnvironment{folder: temp_folder}
    }

    /// Return a Path reference to generated test environment.
    pub fn path(&self)-> &Path{
        self.folder.as_ref()
    }
}

impl AsRef<Path> for TestEnvironment {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::fs::read_dir;
    use std::path::PathBuf;

    #[test]
    fn test_create_test_environment() {
        let env_temp_dir = temp_dir();
        let mut previous_folders: Vec<PathBuf> = Vec::new();
        let folder_entries = read_dir(&env_temp_dir)
            .expect("Error getting entries from system default temp folder.");
        for entry in folder_entries {
            let entry = entry
                .expect("Error getting default temp folder entries") ;
            let path = entry.path();
            if path.is_dir() {previous_folders.push(path)}
        }
        let mut test_folder_path = PathBuf::from("/");
        {
            let test_folder = TestEnvironment::new();
            test_folder_path = PathBuf::from(test_folder.path());
            // Check this folder didn't exist previously.
            assert!(!previous_folders.contains(&test_folder_path),
                    "Test folder already existed.");
            // Check now exists.
            assert!(test_folder_path.exists(),
                    "Test folder does not exists.");
            // Check folder is empty.
            let mut number_of_files = 0;
            let dir_iterator = read_dir(test_folder.path())
                .expect("Error getting folder iterator");
            for _ in dir_iterator {
                number_of_files+=1;
            }
            assert_eq!(0, number_of_files,
                       "Test folder was not empty.");
        } // test_folder should disappear here as variable is dropped.
        // Check test_folder does not exists now.
        assert!(!test_folder_path.exists());
    }

}