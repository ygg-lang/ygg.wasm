use super::*;

use crate::exports::peg::core::types::*;

impl Guest for YggdrasilHost {
    type Language = NativeLanguage;
}

impl GuestLanguage for NativeLanguage {
    fn get_language_name(&self) -> String {
        self.name.to_string()
    }

    fn get_glob_pattern(&self) -> Vec<String> {
        self.glob.iter().map(|s| s.to_string()).collect()
    }
}
