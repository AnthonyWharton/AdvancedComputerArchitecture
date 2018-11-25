///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// Command line config parsing and option structs.
pub mod config;

/// Exit helper functions for successful exits and error exits.
pub mod exit;

/// The ELF file loader and utilities.
pub mod loader;

/// Helper functions for a panic that deals better with raw terminals.
pub mod panic;

///////////////////////////////////////////////////////////////////////////////
//// MACROS

/// Formats the contents of an Option if possible, and prints with the given
/// format specifier. Otherwise formats as "None".
macro_rules! format_option {
    ($fmt_str:expr, $optional:expr) => (
        if $optional.is_some() {
            format!($fmt_str, $optional.unwrap())
        } else {
            std::string::String::from("None")
        }
    )
}

