// data_acquisition.rs
use crate::types::StockData;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn start_data_acquisition_task(stock_data: Arc<Mutex<StockData>>, duration: Duration) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let start = Instant::now();
        while Instant::now().duration_since(start) < duration {
            thread::sleep(Duration::from_secs(1));
            let mut data = stock_data.lock().unwrap();
            data.price += 1.0;
            println!("Data Acquisition: Updated stock price to {}", data.price);
        }
    })
}

// Optionally, if you want this module to also be runnable as a standalone program for testing:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_acquisition() {
        let stock_data = Arc::new(Mutex::new(StockData { price: 100.0 }));
        thread::sleep(Duration::from_secs(5));
    }
}
