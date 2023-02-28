#[cfg(test)]

mod tests {
    #[test]
    fn test_get_home_folder() {
        let home = mytools::env::get_home_folder();
        assert!(home.len() > 0);
    }
}
