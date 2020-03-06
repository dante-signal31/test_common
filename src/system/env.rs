/// Module to deal with environment variables.
use std::env;

/// Context manager like struct to create an environment variable to perform test with it.
///
/// If environment variable already existed then former value is stored before setting the new
/// one. Former value is restored when this type falls out of scope.
///
/// # Example
/// ```rust
/// use test_common::system::env::TemporalEnvironmentVariable;
///
/// {
///     let test_env = TemporalEnvironmentVariable::new("ENVTEST", "test_value");
///     // Do your operations with environment variable.
/// } // Here test environment variable is set to former value if any..
///
pub struct TemporalEnvironmentVariable {
    pub name: String,
    old_value: Option<String>,
    pub current_value: String
}


impl TemporalEnvironmentVariable {

    /// Create a TemporalEnvironmentVariable instance with given name and value.
    ///
    /// If an env var with the same name already exists, previous value is stored at
    /// old_value attribute. That old_value is restored as env var value when this instance
    /// is dropped.
    pub fn new<T, U>(name: T, value:  U)-> Self
        where T: AsRef<str> + AsRef<std::ffi::OsStr>,
              U: AsRef<str>  {
        let old_value= match env::var(&name) {
            Ok(value)=> Some(value.to_string()),
            Err(_)=> None
        };
        let temp_env = TemporalEnvironmentVariable{
                name: (name.as_ref() as &str).to_string(),
                old_value: old_value,
                current_value: value.as_ref().to_string()
            };
        env::set_var(&temp_env.name, &temp_env.current_value);
        temp_env
    }

    /// Set a new current value for env var.
    ///
    /// # Parameters:
    /// * new_value: New value to store.
    pub fn set_var<T>(&mut self, new_value: T)
        where T: AsRef<str> {
        self.current_value = new_value.as_ref().to_string();
        env::set_var(&self.name, &self.current_value);
    }
}


impl Drop for TemporalEnvironmentVariable {

    /// Leave environment variable with value it had before, if any.
    fn drop(&mut self) {
        if let Some(value) = &self.old_value {
            env::set_var(&self.name, value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use crate::random::strings::random_string;
    // use std::ops::Deref;

    fn _get_not_existing_env_var_name()-> String {
        let desired_length: usize = 10;
        let mut new_var_name = random_string(desired_length);
        // Assert there is no previous generated var with that name.
        while env::var(&new_var_name).is_ok() {
            new_var_name = random_string(desired_length);
        };
        new_var_name
    }

    #[test]
    fn test_not_previously_existing_environment_variable_creation() {
        let new_var_name = _get_not_existing_env_var_name();
        let desired_value: &str = "Hello";
        let _temp_env_var = TemporalEnvironmentVariable::new(&new_var_name, desired_value);
        match env::var(&new_var_name) {
            Ok(val)=> assert_eq!(desired_value, val),
            Err(_)=> panic!("Environment var was not found")
        }
    }

    #[test]
    fn test_previously_existing_environment_variable_creation() {
        // Create previous env var.
        let new_var_name = _get_not_existing_env_var_name();
        let old_value = "Bye";
        env::set_var(&new_var_name, old_value);
        {
            // Check we can store a new value.
            let desired_value: &str = "Hello";
            let _temp_env_var = TemporalEnvironmentVariable::new(&new_var_name, desired_value);
            match env::var(&new_var_name) {
                Ok(val)=> assert_eq!(desired_value, val),
                Err(_)=> panic!("Environment var was not found")
            }
        }
        // Check old value has been restored.
        match env::var(&new_var_name) {
            Ok(val)=> assert_eq!(old_value, val),
            Err(_)=> panic!("Environment var was not found")
        }
    }

    #[test]
    fn test_set_var() {
        // Give a previous value to env var.
        let new_var_name = _get_not_existing_env_var_name();
        let desired_value = "Hello";
        let mut temp_env_var = TemporalEnvironmentVariable::new(&new_var_name, desired_value);
        match env::var(new_var_name) {
            Ok(val)=> assert_eq!(desired_value, val),
            Err(_)=> panic!("Environment var was not found")
        }
        // Update env var value.
        let new_value = "Hello world";
        temp_env_var.set_var(new_value);
        assert_eq!(new_value, temp_env_var.current_value,
                   "Value has not been correctly updated. Expected {} but got {}",
                   new_value, temp_env_var.current_value)
    }

}