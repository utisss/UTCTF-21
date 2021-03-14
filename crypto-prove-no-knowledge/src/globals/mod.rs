#[cfg(debug_assertions)]
pub mod debug_defaults;
#[cfg(not(debug_assertions))]
pub mod release_defaults;
pub mod varnames;
