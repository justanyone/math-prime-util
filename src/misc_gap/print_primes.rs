use std::io::{self, Write};

/// Write the primes in `low..=high`, with one prime per line.
///
/// Port of `util.c`'s `print_primes` in MPU 0.75. Uses the crate's prime
/// iterator and the C routine's 8000-byte buffer threshold.
/// The writer stays open. Repeated calls append the same output.
///
/// # Errors
/// Returns the writer's error if a write fails. Some output may be written.
///
/// # Examples
/// ```
/// use math_prime_util::print_primes;
/// let mut output = Vec::new();
/// print_primes(2, 7, &mut output).unwrap();
/// assert_eq!(output, b"2\n3\n5\n7\n");
/// ```
pub fn print_primes(low: u64, high: u64, mut writer: impl Write) -> io::Result<()> {
    let mut buffer = Vec::with_capacity(8025);
    for prime in crate::primes_in(low..=high) {
        writeln!(buffer, "{prime}")?;
        if buffer.len() > 8000 {
            writer.write_all(&buffer)?;
            buffer.clear();
        }
    }
    if !buffer.is_empty() {
        writer.write_all(&buffer)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::print_primes;
    use std::io::{self, Write};

    #[test]
    fn ranges_and_repeated_calls() {
        let mut out = Vec::new();
        for (lo, hi) in [(0, 1), (9, 2), (14, 16)] {
            print_primes(lo, hi, &mut out).unwrap();
        }
        assert_eq!(out, [] as [u8; 0]);
        for _ in 0..2 {
            print_primes(0, 7, &mut out).unwrap();
        }
        assert_eq!(out, b"2\n3\n5\n7\n2\n3\n5\n7\n");
    }

    #[test]
    fn buffer_boundary() {
        let mut out = Vec::new();
        print_primes(0, 30_000, &mut out).unwrap();
        let mut expected = Vec::new();
        for prime in crate::primes(0, 30_000) {
            writeln!(expected, "{prime}").unwrap();
        }
        assert!(out.len() > 8000);
        assert_eq!(out, expected);
    }

    struct Fails;
    impl Write for Fails {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("test write failure"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn write_errors() {
        for high in [2, 30_000] {
            let error = print_primes(0, high, Fails).unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::Other);
        }
        Fails.flush().unwrap();
    }

    #[test]
    fn separate_writers_in_threads() {
        let handles: Vec<_> = (0..4)
            .map(|_| {
                std::thread::spawn(|| {
                    let mut out = Vec::new();
                    print_primes(2, 7, &mut out).unwrap();
                    out
                })
            })
            .collect();
        for handle in handles {
            assert_eq!(handle.join().unwrap(), b"2\n3\n5\n7\n");
        }
    }
}
