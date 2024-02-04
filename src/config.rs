use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::rules;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Config {
    pub src: String,
    pub enabled_rules: Vec<String>,
    pub disable_rules: Vec<String>,
    pub rules: HashMap<String, JsonValue>,
}

impl Default for Config {
    fn default() -> Self {
        let enabled_rules: Vec<String> = vec![];
        let disable_rules: Vec<String> = vec![];

        let mut rules = HashMap::new();
        rules.insert(
            String::from(rules::e7::CODE),
            serde_json::to_value(rules::e7::Settings::default()).unwrap(),
        );

        Config {
            src: String::from("./"),
            enabled_rules,
            disable_rules,
            rules,
        }
    }
}

impl Config {
    pub(crate) fn save(&self, path: PathBuf) {
        let t = serde_yaml::to_string(&self).unwrap();

        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(t.as_bytes()).unwrap();
    }
}
