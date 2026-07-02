// order_processing.rs
use crate::types::StockData;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn start_order_processing_task(stock_data: Arc<Mutex<StockData>>, duration: Duration) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let start = Instant::now();
        while Instant::now().duration_since(start) < duration {
            thread::sleep(Duration::from_secs(2));
            let stock_data = stock_data.lock().unwrap();
            if stock_data.price > 105.0 {
                println!("Order Processing: Selling stock at price {}", stock_data.price);
            } else {
                println!("Order Processing: Buying stock at price {}", stock_data.price);
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_order_processing() {
        let stock_data = Arc::new(Mutex::new(StockData { price: 100.0 }));

        thread::sleep(Duration::from_secs(10));
    }
}
