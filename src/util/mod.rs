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
///
/// Variants:
///  - `format_option!(optional)`
///  - `format_option!(fmt_str, optional)`
///  - `format_option!(lhs, fmt_str, optional)`
///  - `format_option!(lhs, fmt_str, rhs, optional)`
///
/// Where:
///  - `optional`: The optional to be formatted.
///  - `fmt_str`: The format string for the contents of the optional.
///  - `lhs`/`rhs`: The strings to the left/right hand side of the optional.
macro_rules! format_option {
    ($option:expr) => {
        format!("{:?}", $option)
    };
    ($fmt_str:expr, $option:expr) => {
        if $option.is_some() {
            format!($fmt_str, $option.clone().unwrap())
        } else {
            format!("{:?}", $option)
        }
    };
    ($fmt_lhs:expr, $fmt_str:expr, $option:expr) => {
        if $option.is_some() {
            format!(concat!($fmt_lhs, $fmt_str), $option.clone().unwrap())
        } else {
            format!(concat!($fmt_lhs, "{:?}"), $option)
        }
    };
    ($fmt_lhs:expr, $fmt_str:expr, $fmt_rhs:expr, $option:expr) => {
        if $option.is_some() {
            format!(concat!($fmt_lhs, $fmt_str, $fmt_rhs), $option.clone().unwrap())
        } else {
            format!(concat!($fmt_lhs, "{:?}", $fmt_rhs), $option)
        }
    };
}
