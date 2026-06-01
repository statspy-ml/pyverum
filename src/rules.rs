pub trait Rule {
    fn validate(&self, value: &str) -> Option<String>;
}

pub struct NotNull;
pub struct NotEmpty;
pub struct GreaterThan(f64);
pub struct LessThan(f64);
pub struct GreaterEqualThan(f64);
pub struct LessEqualThan(f64);
pub struct Equal(f64);
pub struct IsPositive;
pub struct IsNegative;
pub struct Between(f64, f64);
pub struct IsFinite;
pub struct IsIn(Vec<String>);

impl Rule for NotNull {
    fn validate(&self, value: &str) -> Option<String> {
        if value.trim().is_empty() {
            Some("Value is null or empty".to_string())
        } else {
            None
        }
    }
}

impl Rule for NotEmpty {
    fn validate(&self, value: &str) -> Option<String> {
        if value.trim().is_empty() {
            Some("Value is empty".to_string())
        } else {
            None
        }
    }
}

impl Rule for GreaterThan {
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n > self.0 => None,
            Ok(_) => Some(format!("value must be greater than {}", self.0)),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for GreaterEqualThan {
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n >= self.0 => None,
            Ok(_) => Some(format!("value must be greater or equal than {}", self.0)),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for LessThan {
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n < self.0 => None,
            Ok(_) => Some(format!("value must be less than {}", self.0)),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for LessEqualThan {
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n <= self.0 => None,
            Ok(_) => Some(format!("value must be less or equal than {}", self.0)),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for Equal {
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n == self.0 => None,
            Ok(_) => Some(format!("value must be equal to {}", self.0)),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for IsPositive {
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n > 0.0 => None,
            Ok(_) => Some("value must be positive".to_string()),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for IsNegative {
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n < 0.0 => None,
            Ok(_) => Some("value must be negative".to_string()),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for Between {
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n >= self.0 && n <= self.1 => None,
            Ok(_) => Some(format!("value must be between {} and {}", self.0, self.1)),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for IsFinite {
    fn validate(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(n) if n.is_finite() => None,
            Ok(_) => Some("found inf or NaN values".to_string()),
            Err(_) => Some(format!("value '{}' is not a number", value)),
        }
    }
}

impl Rule for IsIn {
    fn validate(&self, value: &str) -> Option<String> {
        if self.0.iter().any(|v| v == value) {
            None
        } else {
            Some(format!("value '{}' is not in {:?}", value, self.0))
        }
    }
}
