/// Struct holding two objects of arbitrary type
#[derive(Debug)]
pub struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    /// Creates a new pair <br/>
    /// Example
    /// ```Rust
    /// let pair = Pair::new(1, 2);
    /// ```
    pub fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    /// Returns a read-only reference to first value
    pub fn first(&self) -> &T {
        &self.first
    }

    /// Returns a read-only reference to first value
    pub fn second(&self) -> &U {
        &self.second
    }

    /// Sets the first value
    pub fn set_first(&mut self, first: T) {
        self.first = first;
    }

    /// Sets the second value
    pub fn set_second(&mut self, second: U) {
        self.second = second;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair() {
        let mut pair = Pair::new(1, 2);
        assert_eq!(*(pair.first()), 1);
        assert_eq!(*(pair.second()), 2);
        pair.set_first(4);
        pair.set_second(5);
        assert_eq!(*(pair.first()), 4);
        assert_eq!(*(pair.second()), 5);
    }
}
