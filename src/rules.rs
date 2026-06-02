use chrono::NaiveDate;
use regex::Regex;
use std::collections::HashSet;

pub fn validate_unique(values: &[&str]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut errors = Vec::new();
    for value in values {
        if !seen.insert(*value) {
            errors.push(format!("duplicate value '{}'", value));
        }
    }
    errors
}

pub trait Rule {
    fn validate(&self, value: &str) -> Option<String>;
    fn name(&self) -> String;
}

pub struct NotNull;
pub struct NotEmpty;
pub struct GreaterThan(pub f64);
pub struct LessThan(pub f64);
pub struct GreaterEqualThan(pub f64);
pub struct LessEqualThan(pub f64);
pub struct Equal(pub f64);
pub struct IsPositive;
pub struct IsNegative;
pub struct Between(pub f64, pub f64);
pub struct IsFinite;
pub struct IsIn(pub Vec<String>);
pub struct Contains(pub String);
pub struct StartsWith(pub String);
pub struct EndsWith(pub String);
pub struct MaxLength(pub usize);
pub struct MinLength(pub usize);
pub struct LengthBetween(pub usize, pub usize);
pub struct MatchRegex(pub String);
pub struct DateFormat(pub String);
pub struct AfterDate(pub String, pub String);
pub struct BeforeDate(pub String, pub String);
pub struct BetweenDates(pub String, pub String, pub String);

impl Rule for NotNull {
    fn name(&self) -> String { "NotNull".to_string() }
    fn validate(&self, value: &str) -> Option<String> {
        if value.trim().is_empty() {
            Some("Value is null or empty".to_string())
        } else {
            None
        }
    }
}

impl Rule for NotEmpty {
    fn name(&self) -> String { "NotEmpty".to_string() }
    fn validate(&self, value: &str) -> Option<String> {
        if value.trim().is_empty() {
            Some("Value is empty".to_string())
        } else {
            None
        }
    }
}

impl Rule for GreaterThan {
    fn name(&self) -> String { format!("GreaterThan({})", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n > self.0 => None,
            Ok(_) => Some(format!("value must be greater than {}", self.0)),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for GreaterEqualThan {
    fn name(&self) -> String { format!("GreaterEqualThan({})", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n >= self.0 => None,
            Ok(_) => Some(format!("value must be greater or equal than {}", self.0)),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for LessThan {
    fn name(&self) -> String { format!("LessThan({})", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n < self.0 => None,
            Ok(_) => Some(format!("value must be less than {}", self.0)),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for LessEqualThan {
    fn name(&self) -> String { format!("LessEqualThan({})", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n <= self.0 => None,
            Ok(_) => Some(format!("value must be less or equal than {}", self.0)),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for Equal {
    fn name(&self) -> String { format!("Equal({})", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n == self.0 => None,
            Ok(_) => Some(format!("value must be equal to {}", self.0)),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for IsPositive {
    fn name(&self) -> String { "IsPositive".to_string() }
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n > 0.0 => None,
            Ok(_) => Some("value must be positive".to_string()),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for IsNegative {
    fn name(&self) -> String { "IsNegative".to_string() }
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n < 0.0 => None,
            Ok(_) => Some("value must be negative".to_string()),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for Between {
    fn name(&self) -> String { format!("Between({}, {})", self.0, self.1) }
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n >= self.0 && n <= self.1 => None,
            Ok(_) => Some(format!("value must be between {} and {}", self.0, self.1)),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for IsFinite {
    fn name(&self) -> String { "IsFinite".to_string() }
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n.is_finite() => None,
            Ok(_) => Some("found inf or NaN values".to_string()),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for IsIn {
    fn name(&self) -> String { format!("IsIn({:?})", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        if self.0.iter().any(|v| v == value) {
            None
        } else {
            Some(format!("value '{}' is not in {:?}", value, self.0))
        }
    }
}

impl Rule for Contains {
    fn name(&self) -> String { format!("Contains('{}')", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        if value.contains(&self.0) {
            None
        } else {
            Some(format!("value '{}' does not contain '{}'", value, self.0))
        }
    }
}

impl Rule for StartsWith {
    fn name(&self) -> String { format!("StartsWith('{}')", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        if value.starts_with(&self.0) {
            None
        } else {
            Some(format!("value '{}' does not start with '{}'", value, self.0))
        }
    }
}

impl Rule for EndsWith {
    fn name(&self) -> String { format!("EndsWith('{}')", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        if value.ends_with(&self.0) {
            None
        } else {
            Some(format!("value '{}' does not end with '{}'", value, self.0))
        }
    }
}

impl Rule for MaxLength {
    fn name(&self) -> String { format!("MaxLength({})", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        if value.len() <= self.0 {
            None
        } else {
            Some(format!(
                "value '{}' has length '{}', expected at most '{}'",
                value, value.len(), self.0
            ))
        }
    }
}

impl Rule for MinLength {
    fn name(&self) -> String { format!("MinLength({})", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        if value.len() >= self.0 {
            None
        } else {
            Some(format!(
                "value '{}' has length '{}', expected minimum length of '{}'",
                value, value.len(), self.0
            ))
        }
    }
}

impl Rule for LengthBetween {
    fn name(&self) -> String { format!("LengthBetween({}, {})", self.0, self.1) }
    fn validate(&self, value: &str) -> Option<String> {
        if value.len() >= self.0 && value.len() <= self.1 {
            None
        } else {
            Some(format!(
                "value '{}' has length '{}', expected minimum length of '{}' and at most '{}'",
                value, value.len(), self.0, self.1
            ))
        }
    }
}

impl Rule for MatchRegex {
    fn name(&self) -> String { format!("MatchRegex('{}')", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        match Regex::new(&self.0) {
            Err(_) => Some(format!("invalid regex pattern '{}'", self.0)),
            Ok(r) if r.is_match(value) => None,
            Ok(_) => Some(format!(
                "value '{}' does not match the pattern '{}'",
                value, self.0
            )),
        }
    }
}

impl Rule for DateFormat {
    fn name(&self) -> String { format!("DateFormat('{}')", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        match NaiveDate::parse_from_str(value, &self.0) {
            Err(_) => Some(format!(
                "value '{}' does not match date format '{}'",
                value, self.0
            )),
            Ok(_) => None,
        }
    }
}

impl Rule for AfterDate {
    fn name(&self) -> String { format!("AfterDate('{}')", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        let reference = match NaiveDate::parse_from_str(&self.0, &self.1) {
            Ok(d) => d,
            Err(_) => return Some(format!("invalid reference date '{}'", self.0)),
        };
        match NaiveDate::parse_from_str(value, &self.1) {
            Err(_) => Some(format!("value '{}' does not match date format '{}'", value, self.1)),
            Ok(n) if n > reference => None,
            Ok(_) => Some(format!("date '{}' must be after '{}'", value, self.0)),
        }
    }
}

impl Rule for BeforeDate {
    fn name(&self) -> String { format!("BeforeDate('{}')", self.0) }
    fn validate(&self, value: &str) -> Option<String> {
        let reference = match NaiveDate::parse_from_str(&self.0, &self.1) {
            Ok(d) => d,
            Err(_) => return Some(format!("invalid reference date '{}'", self.0)),
        };
        match NaiveDate::parse_from_str(value, &self.1) {
            Err(_) => Some(format!("value '{}' does not match date format '{}'", value, self.1)),
            Ok(n) if n < reference => None,
            Ok(_) => Some(format!("date '{}' must be before '{}'", value, self.0)),
        }
    }
}

impl Rule for BetweenDates {
    fn name(&self) -> String { format!("BetweenDates('{}', '{}')", self.0, self.1) }
    fn validate(&self, value: &str) -> Option<String> {
        let from = match NaiveDate::parse_from_str(&self.0, &self.2) {
            Ok(d) => d,
            Err(_) => return Some(format!("invalid reference date '{}'", self.0)),
        };
        let to = match NaiveDate::parse_from_str(&self.1, &self.2) {
            Ok(d) => d,
            Err(_) => return Some(format!("invalid reference date '{}'", self.1)),
        };
        match NaiveDate::parse_from_str(value, &self.2) {
            Err(_) => Some(format!("value '{}' does not match date format '{}'", value, self.2)),
            Ok(n) if n >= from && n <= to => None,
            Ok(_) => Some(format!("date '{}' must be between '{}' and '{}'", value, self.0, self.1)),
        }
    }
}
