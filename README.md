# Circuit Breaker

A robust Rust implementation of the Circuit Breaker pattern, designed to prevent cascading failures in distributed systems and improve system resilience.

## Features

- Three-state circuit breaker: Closed, Open, and Half-Open
- Configurable failure threshold and reset timeout
- Thread-safe implementation using `Arc` and `Mutex`
- Customizable callback functions for state transitions
- Automatic state transition from Open to Half-Open after reset timeout
- Comprehensive error handling and custom error types
- Designed for easy integration into existing Rust projects

## Table of Contents

1. [Installation](#installation)
2. [Usage](#usage)
3. [API Reference](#api-reference)
4. [Circuit Breaker States](#circuit-breaker-states)
5. [Configuration](#configuration)
6. [Error Handling](#error-handling)
7. [Thread Safety](#thread-safety)
8. [Examples](#examples)
9. [Testing](#testing)
10. [Contributing](#contributing)
11. [License](#license)

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
circuit_breaker = "0.1.0"
```


## Usage

Here's a basic example of how to use the Circuit Breaker:

```rust
use circuit_breaker::{CircuitBreaker, CircuitState};
use std::time::Duration;
fn main() {
// Create a new CircuitBreaker with a failure threshold of 3 and a reset timeout of 60 seconds
let cb = CircuitBreaker::new(3, Duration::from_secs(60));
// Set up callbacks for state transitions
cb.set_on_open(|| println!("Circuit opened!"));
cb.set_on_close(|| println!("Circuit closed!"));
cb.set_on_half_open(|| println!("Circuit is half-open!"));
// Execute an operation through the circuit breaker
for in 0..5 {
match cb.execute(|| {
// Simulating an operation that might fail
if rand::random::<f32>() < 0.5 {
Ok("Operation successful")
} else {
Err(std::io::Error::new(std::io::ErrorKind::Other, "Operation failed"))
}
}) {
Ok(result) => println!("Result: {}", result),
Err(e) => println!("Error: {}", e),
}
println!("Current state: {:?}", cb.state());
}
}
```


## API Reference

### `CircuitBreaker`

- `new(failure_threshold: u32, reset_timeout: Duration) -> Self`
  Creates a new `CircuitBreaker` instance.

- `execute<F, T, E>(&self, f: F) -> Result<T, Box<dyn std::error::Error>>`
  Executes the given function within the circuit breaker context.

- `state(&self) -> CircuitState`
  Returns the current state of the circuit breaker.

- `handle_failure(&self)`
  Manually records a failure, potentially opening the circuit.

- `handle_success(&self)`
  Manually records a success, potentially closing the circuit if it was half-open.

- `set_on_open<F>(&self, callback: F)`
  Sets a callback function to be executed when the circuit opens.

- `set_on_close<F>(&self, callback: F)`
  Sets a callback function to be executed when the circuit closes.

- `set_on_half_open<F>(&self, callback: F)`
  Sets a callback function to be executed when the circuit transitions to half-open.

### `CircuitState`

An enum representing the possible states of the circuit breaker:

- `Closed`: The circuit is closed and allowing requests to pass through.
- `Open`: The circuit is open and blocking requests.
- `HalfOpen`: The circuit is allowing a limited number of requests to test if the system has recovered.

## Circuit Breaker States

1. **Closed**: In this state, all requests are allowed to pass through. The circuit breaker keeps track of the number of failures.

2. **Open**: When the number of failures exceeds the threshold, the circuit breaker trips and enters the Open state. In this state, all requests are immediately rejected without calling the protected function.

3. **Half-Open**: After the reset timeout has elapsed, the circuit breaker enters the Half-Open state. In this state, a limited number of requests are allowed through to test if the system has recovered.

## Configuration

The `CircuitBreaker` can be configured with two main parameters:

1. `failure_threshold`: The number of consecutive failures that will cause the circuit to open.
2. `reset_timeout`: The duration after which the circuit will transition from Open to Half-Open.

## Error Handling

The circuit breaker uses a custom `CircuitBreakerError` type to represent errors specific to its operation. When the circuit is open, `execute()` will return a `CircuitBreakerError::CircuitOpen` error.

## Thread Safety

The `CircuitBreaker` is designed to be thread-safe and can be safely shared between multiple threads. It uses `Arc` and `Mutex` internally to ensure safe concurrent access.

## Examples

### Basic Usage

```rust
use circuit_breaker::CircuitBreaker;
use std::time::Duration;
let cb = CircuitBreaker::new(3, Duration::from_secs(60));
match cb.execute(|| {
// Your potentially failing operation here
Ok::<, std::io::Error>("Success")
}) {
Ok(result) => println!("Result: {}", result),
Err(e) => println!("Error: {}", e),
}

```


### With Callbacks

```rust
use circuit_breaker::CircuitBreaker;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
let cb = CircuitBreaker::new(3, Duration::from_secs(60));
let open_count = Arc::new(AtomicUsize::new(0));
let open_count_clone = open_count.clone();
cb.set_on_open(move || {
open_count_clone.fetch_add(1, Ordering::SeqCst);
println!("Circuit opened!");
});
```
// Similar setup for on_close and on_half_open callbacks


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the Apache-2.0 License - see the [LICENSE](LICENSE) file for details.