//! A circuit breaker implementation in Rust.
//!
//! This module provides a `CircuitBreaker` struct that implements the circuit breaker pattern.
//! It can be used to improve the stability and resilience of a system by preventing
//! cascading failures when a part of the system becomes unavailable.

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::error::Error;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/// Represents the state of a circuit breaker.
#[derive(Debug, Clone, Copy)]
pub enum CircuitState {
    /// The circuit is closed and allowing requests to pass through.
    Closed,
    /// The circuit is open and blocking requests.
    Open,
    /// The circuit is allowing a limited number of requests to test if the system has recovered.
    HalfOpen,
}

/// A circuit breaker that can be used to wrap potentially failing operations.
pub struct CircuitBreaker {
    failure_threshold: u32,
    reset_timeout: Duration,
    state: Arc<Mutex<CircuitBreakerState>>,
}

struct CircuitBreakerState {
    state: CircuitState,
    failures: u32,
    last_failure_time: Option<Instant>,
}

impl CircuitBreaker {
    /// Creates a new `CircuitBreaker` with the specified failure threshold and reset timeout.
    ///
    /// # Arguments
    ///
    /// * `failure_threshold` - The number of consecutive failures that will cause the circuit to open.
    /// * `reset_timeout` - The duration after which the circuit will transition from `Open` to `HalfOpen`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use circuit_breaker::CircuitBreaker;
    ///
    /// let cb = CircuitBreaker::new(5, Duration::from_secs(30));
    /// ```
    pub fn new(failure_threshold: u32, reset_timeout: Duration) -> Self {
        CircuitBreaker {
            failure_threshold,
            reset_timeout,
            state: Arc::new(Mutex::new(CircuitBreakerState {
                state: CircuitState::Closed,
                failures: 0,
                last_failure_time: None,
            })),
        }
    }

    /// Executes the given function within the circuit breaker.
    ///
    /// If the circuit is open, this method will return an error without executing the function.
    /// If the circuit is closed or half-open, it will execute the function and update the
    /// circuit state based on the result.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that returns a `Result`.
    ///
    /// # Returns
    ///
    /// Returns the result of the function if it was executed, or a `Box<dyn Error>` if the circuit was open
    /// or if the function returned an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use circuit_breaker::CircuitBreaker;
    /// # use std::time::Duration;
    /// # use std::error::Error;
    /// let cb = CircuitBreaker::new(5, Duration::from_secs(30));
    /// let result = cb.execute(|| -> Result<i32, std::io::Error> {
    ///     // Your potentially failing operation here
    ///     Ok(42)
    /// });
    /// assert_eq!(result.unwrap(), 42);
    ///
    /// // Example with an error
    /// let error_result = cb.execute(|| -> Result<(), std::io::Error> {
    ///     Err(std::io::Error::new(std::io::ErrorKind::Other, "test error"))
    /// });
    /// assert!(error_result.is_err());
    /// ```
    pub fn execute<F, T, E>(&self, f: F) -> Result<T, Box<dyn Error>>
    where
        F: FnOnce() -> Result<T, E>,
        E: Error + 'static,
    {
        let mut state = self.state.lock().unwrap();

        match state.state {
            CircuitState::Open => {
                if let Some(last_failure) = state.last_failure_time {
                    if last_failure.elapsed() >= self.reset_timeout {
                        state.state = CircuitState::HalfOpen;
                    } else {
                        return Err(Box::new(CircuitBreakerError::CircuitOpen));
                    }
                }
            }
            CircuitState::HalfOpen => {}
            CircuitState::Closed => {}
        }

        drop(state);

        match f() {
            Ok(result) => {
                let mut state = self.state.lock().unwrap();
                state.failures = 0;
                state.state = CircuitState::Closed;
                Ok(result)
            }
            Err(e) => {
                let mut state = self.state.lock().unwrap();
                state.failures += 1;
                state.last_failure_time = Some(Instant::now());

                if state.failures >= self.failure_threshold {
                    state.state = CircuitState::Open;
                }

                Err(Box::new(e))
            }
        }
    }

    pub fn state(&self) -> CircuitState {
        self.state.lock().unwrap().state
    }
}

#[derive(Debug)]
pub enum CircuitBreakerError {
    CircuitOpen,
}

impl std::error::Error for CircuitBreakerError {}

impl std::fmt::Display for CircuitBreakerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitBreakerError::CircuitOpen => write!(f, "Circuit is open"),
        }
    }
}
