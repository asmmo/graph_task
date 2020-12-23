type ThreadSafeGenError = Box<dyn std::error::Error + Send + Sync>;
pub mod process_csv;
pub mod less_than_1000_distances_table;
pub mod visualize_less_than_100;