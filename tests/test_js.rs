use std::{error::Error, fs};

use code_executor::{Rule, Rules};

const FILE_PATH: &str = "rules.yaml";

fn load_rules() -> Result<Rules, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let rules: Rules = serde_yaml::from_str(&contents)?;
    Ok(rules)
}

fn get_rule(name: &str) -> Result<Rule, Box<dyn Error>> {
    let rules = load_rules()?;
    let rule = rules
        .rules
        .get(name)
        .expect(&format!("Missing {} runner configuration", name));
    Ok(rule.clone())
}

#[test]
fn test_load_rules() -> Result<(), Box<dyn Error>> {
    load_rules()?;
    Ok(())
}

#[test]
fn test_js() -> Result<(), Box<dyn Error>> {
    let rule = get_rule("js")?;
    let result = rule.run("console.log('Hello, World! from JS')")?;
    assert!("Hello, World! from JS\n" == &result.get(0).unwrap().result);
    Ok(())
}

#[test]
fn test_c() -> Result<(), Box<dyn Error>> {
    let rule = get_rule("c")?;
    let content = r#"
        #include <stdio.h>
        int main() {
        // printf() displays the string inside quotation
        printf("Hello, World! from C\n");
        return 0;
        }
    "#;
    let result = rule.run(content)?;
    assert!("Hello, World! from C\n" == &result.get(1).unwrap().result);
    Ok(())
}

#[test]
fn test_rust() -> Result<(), Box<dyn Error>> {
    let rule = get_rule("rust")?;
    let content = r#"
    fn main() {    
        println!("Hello, World! from rust");
     }
    "#;
    let result = rule.run(content)?;
    assert!("Hello, World! from rust\n" == &result.get(1).unwrap().result);
    Ok(())
}
