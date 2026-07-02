// main.rs
mod data_acquisition;
mod order_processing;
mod user_interaction;
mod types;

use types::StockData;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let stock_data = Arc::new(Mutex::new(StockData { price: 100.0 }));
    let duration = Duration::from_secs(100); // Define the runtime duration

    let data_acquisition_handle = data_acquisition::start_data_acquisition_task(stock_data.clone(), duration);
    let order_processing_handle = order_processing::start_order_processing_task(stock_data.clone(), duration);
    let user_interaction_handle = user_interaction::start_user_interaction_task(stock_data, duration);

    // Wait for all threads to complete their execution
    data_acquisition_handle.join().unwrap();
    order_processing_handle.join().unwrap();
    user_interaction_handle.join().unwrap();
}