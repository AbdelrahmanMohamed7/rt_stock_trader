// user_interaction.rs
use crate::types::StockData;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn start_user_interaction_task(stock_data: Arc<Mutex<StockData>>, duration: Duration) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let start = Instant::now();
        while Instant::now().duration_since(start) < duration {
            thread::sleep(Duration::from_secs(5));
            let mut data = stock_data.lock().unwrap();
            data.price += 5.0;
            println!("User Interaction: Increased target price to {}", data.price);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_user_interaction() {
        let stock_data = Arc::new(Mutex::new(StockData { price: 100.0 }));
        start_user_interaction_task(stock_data);

        thread::sleep(Duration::from_secs(10));
    }
}
