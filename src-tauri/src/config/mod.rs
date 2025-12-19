//! Configuration modules

pub mod loader;
pub mod schema;

/// Re-exports
pub use loader::{save_config, get_default_data_dir, get_user_home, load_config, load_settings};
pub use schema::{AppConfig, AppSettings, DatabaseSettings};