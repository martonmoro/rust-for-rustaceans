struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    #[cfg(test)]
    fn get_value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut c = Counter::new();
        c.increment();
        assert_eq!(c.get_value(), 1); // works in tests
    }
}

fn main() {
    let mut c = Counter::new();
    c.increment();
    // c.get_value() - won't compile
}
