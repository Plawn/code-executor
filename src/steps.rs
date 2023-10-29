use std::time::Instant;

use std::error::Error;
use std::process::Command;
use std::str;
use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

///
///
/// ```yaml
///   rust:
/// steps:
/// - name: compile
///   runner: <compile string>
/// - name: run
///   runner: ./
/// ````
///
///
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Rule {
    pub steps: Vec<Step>,
    pub extension: String,
    pub clean: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub name: String,
    pub result: String,
    pub duration: u128,
}

fn to_str(v: &Vec<u8>) -> &str {
    match str::from_utf8(v) {
        Ok(r) => r,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}

impl Rule {
    fn do_clean(&self, input_name: &str) -> Result<(), Box<dyn Error>> {
        fs::remove_file(input_name)?;
        if let Some(clean) = &self.clean {
            clean.iter().for_each(|e| {
                fs::remove_file(e).expect("failed to delete artifact");
            });
        }
        Ok(())
    }

    pub fn run(&self, input: &str) -> Result<Vec<ExecutionResult>, Box<dyn Error>> {
        let path = format!("{}.{}", uuid::Uuid::new_v4(), &self.extension);
        fs::write(&path, input)?;
        let result = self
            .steps
            .iter()
            .map(|s| {
                let command = s
                    .params
                    .iter()
                    .map(|f| {
                        if f == "<input" {
                            &path
                        } else {
                            f
                        }
                    })
                    .collect::<Vec<&String>>();
                let start = Instant::now();
                
                let res = Command::new(&s.command)
                    .args(&command)
                    .output()
                    .expect("failed to execute process");
                let out = res.stdout;
                let e = to_str(&out);
                let duration = start.elapsed();
                ExecutionResult {
                    name: s.name.clone(),
                    result: e.into(),
                    duration: duration.as_millis(),
                }
            })
            .collect::<Vec<_>>();
        self.do_clean(&path)?;
        Ok(result)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Rules {
    pub rules: HashMap<String, Rule>,
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Step {
    pub name: String,
    pub command: String,
    pub params: Vec<String>,
}
