use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum ConfigType {
    String,
    Boolean,
    Vector,
}

#[derive(Debug)]
struct ConfigVariable {
    var_type: ConfigType,
    value: String,
}

impl ConfigVariable {
    fn extract_value_as_string(&self, typ: ConfigType) -> Result<String, String> {
        if self.var_type == typ {
            Ok(self.value.to_owned())
        } else {
            Err(format!("Incorrect variable type. Expected: {:?}. Actual: {:?}",
                        typ,
                        self.var_type))
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
        // First space splits the key from value, then if a list is separated by a comma
        let mut parts = line.splitn(2, " ").map(|x| x.to_owned());
        let key = parts.next().unwrap().to_lowercase();
        match parts.next() {
            None => {
                self.params.insert(key,
                                   ConfigVariable {
                                       var_type: ConfigType::Boolean,
                                       value: "true".to_owned(),
                                   });
            }
            Some(value) => {
                if value.contains(",") {
                    self.params.insert(key,
                                       ConfigVariable {
                                           var_type: ConfigType::Vector,
                                           value: value,
                                       });
                } else {
                    self.params.insert(key,
                                       ConfigVariable {
                                           var_type: ConfigType::String,
                                           value: value,
                                       });
                }
            }
        }

    }

    fn string_param(&self, key: &str) -> Result<String, String> {
        key.to_lowercase();
        match self.params.get(key) {
            Some(variable) => variable.extract_value_as_string(ConfigType::String),
            None => Err(format!("No such configuration paramter {:?}", key)),
        }

    }

    fn bool_param(&self, key: &str) -> Result<bool, String> {
        key.to_lowercase();
        match self.params.get(key) {
            Some(variable) => {
                variable.extract_value_as_string(ConfigType::Boolean)
                    .map(|x| x.parse::<bool>().unwrap())
            }
            None => Ok(false),
        }
    }

    fn vec_param(&self, key: &str) -> Result<Vec<String>, String> {
        key.to_lowercase();
        match self.params.get(key) {
            Some(variable) => {
                // Get param as string
                // split string at comma
                // trim each item and convert to String
                // collect to vector
                variable.extract_value_as_string(ConfigType::Vector)
                    .map(|string| string.split(",").map(|item| item.trim().to_owned()).collect())
            }
            None => Err(format!("No such configuration paramter {:?}", key)),
        }
    }
}

fn main() {
    const CONF: &'static str = "test.conf";
    let params = ConfigParams::parse(CONF);

    println!("{:?}", params.string_param("fullname"));
    println!("{:?}", params.string_param("favouritefruit"));
    println!("{:?}", params.bool_param("needspeeling"));
    println!("{:?}", params.bool_param("seedsremoved"));
    println!("{:?}", params.vec_param("otherfamily"));
}

#[cfg(test)]
mod tests {

    #[test]
    fn main_test() {
        const CONF: &'static str = "test.conf";
        let params = super::ConfigParams::parse(CONF);
        assert_eq!(params.string_param("fullname").unwrap(), "Foo Barber");
        assert_eq!(params.string_param("favouritefruit").unwrap(), "banana");
        assert!(params.bool_param("needspeeling").unwrap());
        assert!(!params.bool_param("seedsremoved").unwrap());
        assert_eq!(params.vec_param("otherfamily").unwrap(),
                   vec!["Rhu Barber", "Harry Barber"]);
    }
}
