#[cfg(test)]
mod tests {
    use crate::config;

    #[test]
    fn test_typed_example() {
        match config::json_decode::typed_example() {
            Ok(_) => println!("program ran ok"),
            Err(_) => println!("program ran with error"),
        }
    }

    fn test_read_json_file() {
    }
}

/*
fn main() {
    let mut f5 = File::new("5.txt");
    let mut buffer: Vec<u8> = vec![];
    if f5.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }
    f5 = open(f5).unwrap();
    let f5_length = f5.read(&mut buffer).unwrap();
    f5 = close(f5).unwrap();
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}",f5);
    println!("{} is {} bytes long",&f5.name,f5_length);

    println!("{}",text);
}
 */
