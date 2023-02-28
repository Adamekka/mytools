#[macro_use]
extern crate mytools;

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn test_pretty_panic() {
        pretty_panic!("This is a test");
    }

    #[test]
    fn test_warn() {
        warn!("This is a test");
    }
}
