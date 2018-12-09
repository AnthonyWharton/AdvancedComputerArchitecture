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
    ($fmt_lhs:expr, $fmt_str:expr, $fmt_rhs:expr, $option:expr) => {
        if $option.is_some() {
            format!(concat!($fmt_lhs, $fmt_str, $fmt_rhs), $option.unwrap())
        } else {
            format!(concat!($fmt_lhs, "{:?}", $fmt_rhs), $option)
        }
    };
}
