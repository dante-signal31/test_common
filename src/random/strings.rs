/// Random operations with strings.

use rand::prelude::*;
use rand::distributions::Alphanumeric;

/// Generate a random string of desired length.
///
/// # Parameters:
/// * len: Desired character length for generated string.
///
/// # Returns:
/// * Generated random string.
pub fn random_string(len: usize)-> String {
    let generated_string: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(len)
        .collect();
    generated_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_string() {
        let desired_length: usize = 7;
        let generated_string = random_string(desired_length);
        let generated_length = generated_string.len();
        assert_eq!(desired_length, generated_length,
                   "Generated random string has not desired length of {} but {} instead",
                   desired_length, generated_length);
    }
}