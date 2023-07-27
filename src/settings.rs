use crate::LOG_DRAIN;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use slog::info;

// Describe the settings your policy expects when
// loaded by the policy server.
#[derive(Deserialize, Default, Debug, Serialize)]
#[serde(default)]
pub(crate) struct Settings {
    pub invalid_names: HashSet<String>,
}

impl kubewarden::settings::Validatable for Settings {
    fn validate(&self) -> Result<(), String> {
        if self.invalid_names.is_empty() {
            Err(String::from("No invalid name specified. Specify at least one invalid name to match"))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use kubewarden_policy_sdk::settings::Validatable;

    #[test]
    fn accept_settings_with_a_list_of_invalid_names() -> Result<(), ()> {
        let mut invalid_names = HashSet::new();
        invalid_names.insert(String::from("bad_name1"));
        invalid_names.insert(String::from("bad_name2"));

        let settings = Settings { invalid_names };

        assert!(settings.validate().is_ok());
        Ok(())
    }

    #[test]
    fn reject_settings_without_a_list_of_invalid_names() -> Result<(), ()> {
        let invalid_names = HashSet::<String>::new();
        let settings = Settings { invalid_names };

        assert!(settings.validate().is_err());
        Ok(())
    }
}
