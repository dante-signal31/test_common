/// Module to perform cryptographic file related operations.

use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Error};
use std::str;

/// Hash file content with SHA-256.
///
/// This way we can check two files have same content.
///
/// Original code got from [Rust Cookbok](https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html)
///
/// # Parameters:
/// * file_path: Absolute path name as a &str.
///
/// # Returns:
/// File has as a Digest or a Error if any ocurred.
pub fn hash_file(file_path: &str) -> Result<Digest, Error> {
    let mut reader = BufReader::new(File::open(file_path)?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use data_encoding::HEXUPPER;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_hash_file() {
        let mut file = NamedTempFile::new()
            .expect("Error creating temporal file.");
        file.write_all(b"foobar")
            .expect("Error writing content to temporal file for hashing.");
        let file_path = file.path();
        let expected_hash = "c3ab8ff13720e8ad9047dd39466b3c8974e592c2fa383d4a3960714caef0c4f2";
        let hash = hash_file(file_path.as_os_str().to_str()
            .expect("Error getting temporal file path."))
            .expect("Error getting temporal file hash.");
        let hash_value = hash.as_ref();
        let recovered_hash = HEXUPPER.encode(hash_value).to_lowercase();
        assert_eq!(expected_hash, recovered_hash,
            "Recovered hash is not what we were expecting. Expected {} but got {}.",
            expected_hash, recovered_hash);
    }
}