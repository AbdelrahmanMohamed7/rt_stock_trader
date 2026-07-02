# Real-Time Systems: Multi-Threaded Stock Trading Simulator (Rust)

A high-performance, concurrent simulation of a real-time stock trading platform engineered in **Rust**. Built to investigate optimizations in task processing, thread synchronization latency, and resource scheduling under real-time constraints, this system utilizes an independent-thread architecture that decouples data acquisition from automated trade execution.

This project was developed to analyze safe in-memory shared state management and deterministic task processing loops without the overhead of runtime garbage collection.

---

## 🛠️ System Architecture Design

The application decouples competitive data pipelines into independent worker threads running concurrently. Thread communication is synchronized via safe, shared in-memory boundaries:

```text
                      +---------------------------------------+
                      |         Shared In-Memory State        |
                      |          [Arc<Mutex<StockData>>]      |
                      +-------------------+-------------------+
                                          |
                +-------------------------+-------------------------+
                |                         |                         |
                v                         v                         v
       +------------------+     +------------------+     +------------------+
       | Data Acquisition |     | Order Processing |     | User Interaction |
       |   Thread (1s)    |     |   Thread (2s)    |     |   Thread (5s)    |
       |  Simulates Live  |     | Evaluates Limits |     | Simulates Live   |
       |  Price Updates   |     | Executing Orders |     | Strategy Tweaks  |
       +------------------+     +------------------+     +------------------+

```

### Decoupled Task Processing Modules

* **Data Ingestion (`data_acquisition.rs`):** Simulates real-time market data streaming at a deterministic **1-second** interval, updating the shared stock price.
* **Order Processing (`order_processing.rs`):** Tracks current stock prices and evaluates trade trigger criteria (arbitrage boundaries) every **2 seconds** without causing thread starvation.
* **Strategy Tweaking (`user_interaction.rs`):** Simulates asynchronous user adjustments to target prices at a **5-second** interval.
* **Shared Data Model (`types.rs`):** Defines the core thread-safe `StockData` struct shared globally.

---

## ⚡ Key Architectural Engineering Highlights

* **Fearless Concurrency & Memory Safety:** Leverages Rust's compile-time ownership semantics. Mutual Exclusion (`Mutex`) is used to protect against asynchronous data corruption, and Atomic Reference Counting (`Arc`) safely duplicates state pointers across thread contexts.
* **Resource Contention Avoidance:** Ensures that the data acquisition thread updating the stock values doesn't create deadlocks or data races with the order execution thread inspecting those same values under sub-second constraints.
* **Deterministic Benchmarking:** Runs the simulation over controlled execution windows using standard high-precision monotones (`std::time::Instant` and `std::time::Duration`) to track precise response latencies.

---

## 💻 Compilation & Local Execution

To run this concurrent Rust systems engine on your Mac locally:

### Prerequisites

Ensure you have the **Rust Toolchain** installed. You can check this by running:

```bash
cargo --version

```

If it is not installed, download it via your Terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh

```

### Execution Steps

1. Open your Mac **Terminal** (or VS Code Integrated Terminal).
2. Navigate directly to your project root folder:
```bash
cd /Users/your_username/Desktop/rt_stock_trader

```


3. Run the concurrent engine in development mode:
```bash
cargo run

```


4. To test under maximized optimization and zero context-switch overhead (for benchmarking metrics), run:
```bash
cargo run --release

```



---

## 🔬 Evaluation & System Metrics

* **Task Processing Speeds:** Tested and confirmed to run deterministic intervals stably over extended periods.
* **Concurrence Resource Efficiency:** Benchmarked to verify negligible lock contention overhead, highly stable memory usage, and absolute protection against data races without requiring a runtime garbage collector.

```

```
