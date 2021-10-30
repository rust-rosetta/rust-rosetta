use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Cursor, Read};
use std::path::Path;
use std::str::FromStr;

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

fn is_comment(line: &str) -> bool {
    let comment_chars = ['#', ';'];
    line.starts_with(&comment_chars[..]) || line.is_empty()
}

impl ConfigParams {
    fn new() -> ConfigParams {
        ConfigParams {
            params: HashMap::new(),
        }
    }

    fn parse<P: AsRef<Path>>(path: P) -> io::Result<ConfigParams> {
        Self::from_reader(File::open(path)?)
    }

    fn from_reader(reader: impl Read) -> io::Result<ConfigParams> {
        let reader = BufReader::new(reader);

        let mut params = ConfigParams::new();
        for line in reader.lines() {
            let line = line?;
            if is_comment(&line) {
                continue;
            }
            params.update_config(&line);
        }

        Ok(params)
    }

    // Will parse the line and update the internal structure
    fn update_config(&mut self, line: &str) {
        let mut parts = line.splitn(2, ' ').map(|x| x.to_owned());
        let key = parts.next().unwrap().to_lowercase();
        match parts.next() {
            None => {
                self.params.insert(key, ConfigVariable::Boolean(true));
            }
            Some(value) => {
                if value.contains(',') {
                    self.params.insert(
                        key,
                        ConfigVariable::Vector(
                            value
                                .split(',')
                                .map(|item| item.trim().to_owned())
                                .collect(),
                        ),
                    );
                } else {
                    self.params.insert(key, ConfigVariable::String(value));
                }
            }
        }
    }

    fn param<T: FromConfig>(&self, key: &str) -> Result<T, String> {
        FromConfig::from_config(self, &key.to_lowercase())
    }
}

impl FromStr for ConfigParams {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_reader(Cursor::new(s.as_bytes())).unwrap())
    }
}

fn main() {
    const CONF: &str = "test.conf";
    let params = ConfigParams::parse(CONF).unwrap();

    println!("{:?}", params.param::<String>("fullname"));
    println!("{:?}", params.param::<String>("favouritefruit"));
    println!("{:?}", params.param::<bool>("needspeeling"));
    println!("{:?}", params.param::<bool>("seedsremoved"));
    println!("{:?}", params.param::<Vec<String>>("otherfamily"));
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::ConfigParams;

    #[test]
    fn main_test() {
        const CONF: &str = "test.conf";
        let params = super::ConfigParams::parse(CONF).unwrap();
        assert_eq!(params.param::<String>("fullname").unwrap(), "Foo Barber");
        assert_eq!(params.param::<String>("favouritefruit").unwrap(), "banana");
        assert!(params.param::<bool>("needspeeling").unwrap());
        assert!(!params.param::<bool>("seedsremoved").unwrap());
        assert_eq!(
            params.param::<Vec<String>>("otherfamily").unwrap(),
            vec!["Rhu Barber", "Harry Barber"]
        );
    }

    #[test]
    fn options_names_are_case_insensitive() {
        let config = indoc! {"
            FULLNAME Foo Barber
        "}
        .parse::<ConfigParams>()
        .unwrap();

        assert_eq!(config.param::<String>("FULLNAME").unwrap(), "Foo Barber");
        assert_eq!(config.param::<String>("fullname").unwrap(), "Foo Barber");
    }
}
