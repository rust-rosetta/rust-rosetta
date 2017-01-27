use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
enum ConfigVariable {
    String(String),
    Boolean(bool),
    Vector(Vec<String>),
}

trait FromConfig: Sized {
    fn from_config(param: &ConfigParams, key: &str) -> Result<Self, String>;
}

impl FromConfig for String {
    fn from_config(param: &ConfigParams, key: &str) -> Result<String, String> {
        match param.params.get(key) {
            Some(&ConfigVariable::String(ref value)) => Ok(value.clone()),
            _ => Err(format!("Unknown configuration parameter: {:?}", key)),
        }
    }
}

impl FromConfig for bool {
    fn from_config(param: &ConfigParams, key: &str) -> Result<bool, String> {
        match param.params.get(key) {
            Some(&ConfigVariable::Boolean(value)) => Ok(value),
            _ => Ok(false),
        }

    }
}

impl FromConfig for Vec<String> {
    fn from_config(param: &ConfigParams, key: &str) -> Result<Vec<String>, String> {
        match param.params.get(key) {
            Some(&ConfigVariable::Vector(ref value)) => Ok(value.clone()),
            _ => Err(format!("Unknown configuration parameter: {:?}", key)),
        }

    }
}

#[derive(Debug)]
struct ConfigParams {
    params: HashMap<String, ConfigVariable>,
}

impl ConfigParams {
    fn new() -> ConfigParams {
        ConfigParams { params: HashMap::new() }
    }

    fn parse<P: AsRef<Path>>(path: P) -> ConfigParams {
        let conf_file = File::open(path).unwrap();
        let content = BufReader::new(conf_file);

        let is_not_comment = |x: &Result<String, io::Error>| {
            match x {
                &Err(_) => false,
                &Ok(ref line) => {
                    !(line.starts_with("#") || line.starts_with(";") || line.is_empty())
                }
            }
        };
        let mut params = ConfigParams::new();
        for line in content.lines().filter(is_not_comment) {
            if line.is_ok() {
                params.update_config(line.unwrap());
            }
        }

        params
    }
    // Will parse the line and update the internal structure
    fn update_config(&mut self, line: String) {
        let mut parts = line.splitn(2, " ").map(|x| x.to_owned());
        let key = parts.next().unwrap().to_lowercase();
        match parts.next() {
            None => {
                self.params.insert(key, ConfigVariable::Boolean(true));
            }
            Some(value) => {
                if value.contains(",") {
                    self.params.insert(key,
                                       ConfigVariable::Vector(value.split(",")
                                           .map(|item| item.trim().to_owned())
                                           .collect()));
                } else {
                    self.params.insert(key, ConfigVariable::String(value));
                }
            }
        }

    }

    fn param<T: FromConfig>(&self, key: &str) -> Result<T, String> {
        key.to_lowercase();
        FromConfig::from_config(&self, key)
    }
}

fn main() {
    const CONF: &'static str = "test.conf";
    let params = ConfigParams::parse(CONF);

    println!("{:?}", params.param::<String>("fullname"));
    println!("{:?}", params.param::<String>("favouritefruit"));
    println!("{:?}", params.param::<bool>("needspeeling"));
    println!("{:?}", params.param::<bool>("seedsremoved"));
    println!("{:?}", params.param::<Vec<String>>("otherfamily"));
}

#[cfg(test)]
mod tests {

    #[test]
    fn main_test() {
        const CONF: &'static str = "test.conf";
        let params = super::ConfigParams::parse(CONF);
        assert_eq!(params.param::<String>("fullname").unwrap(), "Foo Barber");
        assert_eq!(params.param::<String>("favouritefruit").unwrap(), "banana");
        assert!(params.param::<bool>("needspeeling").unwrap());
        assert!(!params.param::<bool>("seedsremoved").unwrap());
        assert_eq!(params.param::<Vec<String>>("otherfamily").unwrap(),
                   vec!["Rhu Barber", "Harry Barber"]);
    }
}
