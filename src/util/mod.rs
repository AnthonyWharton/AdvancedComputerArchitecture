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
