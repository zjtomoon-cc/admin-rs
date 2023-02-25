use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

pub fn typed_example() -> Result<()> {
    let data = r#"
    {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
    }
    "#;
    let p:Person = serde_json::from_str(data)?;
    println!("Please call {} at the number {}",p.name,p.phones[0]);
    Ok(())
}
