use crate::httpu::Error;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn check_required(headers: &HashMap<String, String>, required: &[&str]) -> Result<(), Error> {
    let missing_headers: Vec<String> = required
        .iter()
        .cloned()
        .take_while(|h| !headers.contains_key(*h))
        .map(String::from)
        .collect();
    if missing_headers.is_empty() {
        Ok(())
    } else {
        error!("message missing headers '{:?}'", missing_headers);
        Err(Error::MessageFormat)
    }
}

pub fn check_parsed_value<T>(header_value: &String, name: &str) -> Result<T, Error>
where
    T: FromStr,
{
    match header_value.parse::<T>() {
        Ok(v) => Ok(v),
        Err(e) => {
            error!(
                "header '{}', value '{}' could not be parsed",
                name, header_value
            );
            Err(Error::MessageFormat)
        }
    }
}

pub fn check_regex(header_value: &String, name: &str, regex: &Regex) -> Result<String, Error> {
    match regex.captures(&header_value) {
        Some(captured) => Ok(captured.get(0).unwrap().as_str().to_string()),
        None => {
            error!(
                "header '{}', value '{}' did not match regex",
                name, header_value
            );
            Err(Error::MessageFormat)
        }
    }
}

pub fn check_empty(header_value: &String, name: &str) -> Result<(), Error> {
    if header_value.trim().is_empty() {
        Ok(())
    } else {
        error!(
            "header '{}', value '{}' should be empty",
            name, header_value
        );
        Err(Error::MessageFormat)
    }
}

pub fn check_not_empty(header_value: &String, name: &str) -> Result<String, Error> {
    if !header_value.trim().is_empty() {
        Ok(header_value.clone())
    } else {
        error!(
            "header '{}', value '{}' should not be empty",
            name, header_value
        );
        Err(Error::MessageFormat)
    }
}