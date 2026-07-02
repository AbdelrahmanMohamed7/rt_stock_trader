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
