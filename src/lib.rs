use std::error::Error;
use std::fmt;

use alloy::hex::encode;
use alloy::primitives::keccak256;

#[derive(Debug)]
pub struct Selector {
    pub function_name: String,
}

#[derive(Debug)]
pub enum SelectorError {
    InvalidInput(String),
    Utf8Error(String),
}

impl fmt::Display for SelectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectorError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            SelectorError::Utf8Error(msg) => write!(f, "UTF-8 conversion error: {}", msg),
        }
    }
}

impl Error for SelectorError {}

impl Selector {
    pub fn new(function_name: String) -> Self {
        let function = parse_the_function(function_name);
        Self {
            function_name: function,
        }
    }

    pub fn turn_function_name_to_bytes(&self) -> Result<String, SelectorError> {
        if self.function_name.is_empty() {
            return Err(SelectorError::InvalidInput(
                "Function name is empty.".to_string(),
            ));
        }

        let hash = keccak256(&self.function_name);
        let first_four_bytes = &hash[0..4];
        Ok(encode(first_four_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_function_name() {
        let selector = Selector::new("transfer(address,uint256)".to_string());
        let result = selector.turn_function_name_to_bytes().unwrap();
        assert_eq!("a9059cbb", result);
    }
}

fn parse_the_function(function_signature: impl AsRef<str>) -> String {
    let signature = function_signature.as_ref().trim();
    let parts: Vec<&str> = signature.splitn(2, '(').collect();

    if parts.len() < 2 {
        return "Error: Invalid function signature.".to_string();
    }

    let func_name = parts[0].trim().split_whitespace().last().unwrap_or("");
    let params = parts[1]
        .find(')')
        .map(|end| &parts[1][..end])
        .unwrap_or("Error: Missing closing parenthesis.");

    let param_types = params
        .split(',')
        .map(|param| param.trim().split_whitespace().next().unwrap_or("unknown"))
        .collect::<Vec<&str>>()
        .join(",");

    format!("{}({})", func_name, param_types)
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn test_valid_input_with_function_keyword() {
        let signature = "function transfer(address recipient, uint256 amount)";
        let expected = "transfer(address,uint256)";
        let output = parse_the_function(signature);
        assert_eq!(output, expected);
    }
}
