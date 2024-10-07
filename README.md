# Circuit Breaker

A Rust implementation of the Circuit Breaker pattern, designed to improve the stability and resilience of your system by preventing cascading failures.

## Features

- Simple and easy-to-use API
- Configurable failure threshold and reset timeout
- Thread-safe implementation using `Arc` and `Mutex`
- Supports three states: Closed, Open, and Half-Open

## Installation

Add this to your `Cargo.toml`:
toml
[dependencies]
circuit_breaker = "0.1.0"


## Usage

Here's a basic example of how to use the Circuit Breaker:

rust
'''
use circuit_breaker::CircuitBreaker;
use std::time::Duration;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
// Create a new CircuitBreaker with a failure threshold of 3 and a reset timeout of 5 seconds
let cb = CircuitBreaker::new(3, Duration::from_secs(5));
// Use the circuit breaker to execute a function
let result = cb.execute(|| {
// Your potentially failing operation here
Ok(42)
});
println!("Result: {:?}", result);
// Example with an error
let error_result = cb.execute(|| {
Err(std::io::Error::new(std::io::ErrorKind::Other, "test error"))
});
println!("Error Result: {:?}", error_result);
Ok(())
}
'''


## API

### `CircuitBreaker::new(failure_threshold: u32, reset_timeout: Duration) -> CircuitBreaker`

Creates a new `CircuitBreaker` instance.

- `failure_threshold`: The number of consecutive failures that will cause the circuit to open.
- `reset_timeout`: The duration after which the circuit will transition from Open to Half-Open.

### `CircuitBreaker::execute<F, T, E>(&self, f: F) -> Result<T, Box<dyn Error>>`

Executes the given function within the circuit breaker.

- `f`: A function that returns a `Result<T, E>` where `E: Error + 'static`.

Returns the result of the function if it was executed successfully, or a `Box<dyn Error>` if the circuit was open or if the function returned an error.

### `CircuitBreaker::state(&self) -> CircuitState`

Returns the current state of the circuit breaker (Closed, Open, or Half-Open).

## Circuit Breaker States

- **Closed**: The circuit is closed and allowing requests to pass through.
- **Open**: The circuit is open and blocking requests.
- **Half-Open**: The circuit is allowing a limited number of requests to test if the system has recovered.

## Error Handling

The `execute` method will return a `CircuitBreakerError::CircuitOpen` error when the circuit is open and not allowing requests. All other errors are passed through from the executed function.

## Thread Safety

This implementation is thread-safe and can be safely shared between threads.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgements

This implementation is inspired by various circuit breaker patterns and adapted for Rust.