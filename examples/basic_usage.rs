use circuit_breaker::CircuitBreaker;
use std::time::Duration;

fn main() {
    let cb = CircuitBreaker::new(3, Duration::from_secs(60));

    // Example usage of the circuit breaker
    match cb.execute(|| {
        // Simulating an operation that might fail
        Ok::<_, std::io::Error>("Operation successful")
    }) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
