#[cfg(test)]
mod tests {
    use circuit_breaker::{CircuitBreaker, CircuitState, CircuitBreakerError};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use std::time::Duration;

  
    #[test]
    fn test_circuit_breaker_state_transitions() {
        let cb = CircuitBreaker::new(3, Duration::from_millis(100));

        assert_eq!(cb.state(), CircuitState::Closed);

        cb.handle_failure();
        cb.handle_failure();
        assert_eq!(cb.state(), CircuitState::Closed);

        cb.handle_failure();
        assert_eq!(cb.state(), CircuitState::Open);

        thread::sleep(Duration::from_millis(150));

        assert_eq!(cb.state(), CircuitState::HalfOpen);

        // The circuit should remain in HalfOpen state until a successful execution
        assert_eq!(cb.state(), CircuitState::HalfOpen);

        // Simulate a successful execution
        cb.execute(|| Ok::<_, std::io::Error>(())).unwrap();

        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[test]
    fn test_circuit_breaker_callbacks() {
        let cb = CircuitBreaker::new(2, Duration::from_millis(100));

        let open_called = Arc::new(AtomicBool::new(false));
        let open_called_clone = open_called.clone();
        cb.set_on_open(move || {
            open_called_clone.store(true, Ordering::SeqCst);
        });

        let close_called = Arc::new(AtomicBool::new(false));
        let close_called_clone = close_called.clone();
        cb.set_on_close(move || {
            close_called_clone.store(true, Ordering::SeqCst);
        });

        let half_open_called = Arc::new(AtomicBool::new(false));
        let half_open_called_clone = half_open_called.clone();
        cb.set_on_half_open(move || {
            half_open_called_clone.store(true, Ordering::SeqCst);
        });

        // Open the circuit
        cb.handle_failure();
        cb.handle_failure();
        assert!(open_called.load(Ordering::SeqCst));

        // Wait for reset timeout
        thread::sleep(Duration::from_millis(150));

        // This should set the state to HalfOpen and trigger the callback
        assert_eq!(cb.state(), CircuitState::HalfOpen);
        assert!(half_open_called.load(Ordering::SeqCst));

        // Simulate a successful execution to close the circuit
        cb.execute(|| Ok::<_, std::io::Error>(())).unwrap();
    }

    #[test]
    fn test_circuit_breaker_execute() {
        let cb = CircuitBreaker::new(2, Duration::from_millis(100));

        let result = cb.execute(|| Ok::<_, std::io::Error>(42));
        assert_eq!(result.unwrap(), 42);

        let _ = cb.execute(|| Err::<i32, _>(std::io::Error::new(std::io::ErrorKind::Other, "error")));
        let _ = cb.execute(|| Err::<i32, _>(std::io::Error::new(std::io::ErrorKind::Other, "error")));

        let result = cb.execute(|| Ok::<_, std::io::Error>(42));
        assert!(matches!(result.unwrap_err().downcast_ref::<CircuitBreakerError>(),
                         Some(CircuitBreakerError::CircuitOpen)));

        thread::sleep(Duration::from_millis(150));

        let result = cb.execute(|| Ok::<_, std::io::Error>(42));
        assert_eq!(result.unwrap(), 42);

        assert_eq!(cb.state(), CircuitState::Closed);
    }
}