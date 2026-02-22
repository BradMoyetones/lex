pub mod contract;
pub mod engine;
pub mod validator;
pub mod registry;
pub mod sanitizer;

// Re-exporting LexModule for easier access
pub use contract::LexModule;