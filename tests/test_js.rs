use std::{fs, error::Error};

use code_executor::Rules;

const FILE_PATH: &str = "rules.yaml";

#[test]
fn test_js()-> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let rules: Rules = serde_yaml::from_str(&contents)?;
    let js_rule = rules.rules.get("js").expect("Missing js runner configuration");
    let result = js_rule.run("console.log('Hello, World! from JS')").unwrap();
    assert!("Hello, World! from JS\n" == &result.get(0).unwrap().result);
    Ok(())
}


#[test]
fn test_c()-> Result<(), Box<dyn Error>> {
    
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let rules: Rules = serde_yaml::from_str(&contents)?;
    let content = r#"
        #include <stdio.h>
        int main() {
        // printf() displays the string inside quotation
        printf("Hello, World! from C\n");
        return 0;
        }
    "#;
    let engine = rules.rules.get("c").expect("Missing c runner configuration");
    let result = engine.run(content).unwrap();
    assert!("Hello, World! from C\n" == &result.get(1).unwrap().result);
    Ok(())
}


#[test]
fn test_rust()-> Result<(), Box<dyn Error>> {
    
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let rules: Rules = serde_yaml::from_str(&contents)?;
    let content = r#"
    fn main() {    
        println!("Hello, World! from rust");
     }
    "#;
    let engine = rules.rules.get("rust").expect("Missing rust runner configuration");
    let result = engine.run(content).unwrap();
    assert!("Hello, World! from rust\n" == &result.get(1).unwrap().result);
    Ok(())
}