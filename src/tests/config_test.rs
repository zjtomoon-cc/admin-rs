#[cfg(test)]
mod tests {
    use crate::config;

    #[test]
    fn test_typed_example() {
        match config::typed_example() {
            Ok(_) => println!("program ran ok"),
            Err(_) => println!("program ran with error"),
        }
    }
}