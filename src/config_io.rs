use crate::config::Config;

#[derive(Debug)]
pub enum ConfigFormat {
    Json,
    Yaml,
}


fn read_config_as_str(config_in: &str) -> Result<String, String> {
    use std::io::prelude::*;
    let mut buffer = String::new();

    if config_in == "-" {
        let mut stdin = std::io::stdin(); // We get `Stdin` here.
        stdin.read_to_string(&mut buffer).map_err(|_x| "We could not read STDIN to get the config".to_string())?;
        return Ok(buffer);
    }

    let mut f = std::fs::File::open(config_in).map_err(|_x| format!("We could not open the file '{}' to get the config", config_in))?;
    f.read_to_string(&mut buffer).map_err(|_x| format!("We could not open the file '{}' to get the config", config_in))?;
    Ok(buffer)
}


pub fn parse_config_str(fmt: &ConfigFormat, config_str: &str) -> Result<Config, String> {
    match fmt {
        ConfigFormat::Json => serde_json::from_str::<Config>(config_str).map_err(|e| format!("Could not parse json config ({}) [{}]", e, config_str)),
        ConfigFormat::Yaml => serde_yaml::from_str::<Config>(config_str).map_err(|e| format!("Could not parse yaml config ({}) [{}]", e, config_str)),
    }
}


fn result_flatten<X>(x: Result<Result<X, String>, String>) -> Result<X, String> {
    match x {
        Ok(Ok(x)) => Ok(x),
        Ok(Err(s)) => Err(s),
        Err(s) => Err(s),
    }
}


pub fn read_config(fmt: &ConfigFormat, config_in: &str) -> Result<Config, String> {
    result_flatten(read_config_as_str(config_in).map(|cas| parse_config_str(fmt, &cas)))
}


