type ThreadSafeGenError = Box<dyn std::error::Error + Send + Sync>;
pub mod process_csv;
pub mod filter;
pub mod visualize;