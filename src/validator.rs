use crate::rules::*;
use crate::schema::{ColumnSchema, Schema};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RuleResults {
    pub column_name: String,
    pub rule_name: String,
    pub pass: bool,
    pub failed_values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub checks_pass: bool,
    pub results: Vec<RuleResults>,
}

pub fn build_rules(schema: &ColumnSchema) -> Vec<Box<dyn Rule>> {
    let mut rules: Vec<Box<dyn Rule>> = vec![];
    if let Some(true) = schema.not_null {
        rules.push(Box::new(NotNull));
    }
    if let Some(true) = schema.not_empty {
        rules.push(Box::new(NotEmpty));
    }
    if let Some(true) = schema.is_positive {
        rules.push(Box::new(IsPositive));
    }
    if let Some(true) = schema.is_negative {
        rules.push(Box::new(IsNegative));
    }
    if let Some(true) = schema.is_finite {
        rules.push(Box::new(IsFinite));
    }
    if let Some(v) = schema.gt {
        rules.push(Box::new(GreaterThan(v)));
    }
    if let Some(v) = schema.ge {
        rules.push(Box::new(GreaterEqualThan(v)));
    }
    if let Some(v) = schema.le {
        rules.push(Box::new(LessEqualThan(v)));
    }
    if let Some(v) = schema.lt {
        rules.push(Box::new(LessThan(v)));
    }
    if let Some(v) = schema.equal {
        rules.push(Box::new(Equal(v)));
    }
    if let Some(v) = schema.between {
        rules.push(Box::new(Between(v[0], v[1])));
    }
    if let Some(ref v) = schema.is_in {
        rules.push(Box::new(IsIn(v.clone())));
    }
    if let Some(ref v) = schema.contains {
        rules.push(Box::new(Contains(v.clone())));
    }
    if let Some(ref v) = schema.starts_with {
        rules.push(Box::new(StartsWith(v.clone())));
    }
    if let Some(ref v) = schema.ends_with {
        rules.push(Box::new(EndsWith(v.clone())));
    }
    if let Some(v) = schema.min_length {
        rules.push(Box::new(MinLength(v)));
    }
    if let Some(v) = schema.max_length {
        rules.push(Box::new(MaxLength(v)));
    }
    if let Some(v) = schema.length_between {
        rules.push(Box::new(LengthBetween(v[0], v[1])));
    }
    if let Some(ref v) = schema.matches_regex {
        rules.push(Box::new(MatchRegex(v.clone())));
    }
    if let Some(ref fmt) = schema.date_format {
        rules.push(Box::new(DateFormat(fmt.clone())));
    }
    if let (Some(after), Some(fmt)) = (&schema.after, &schema.date_format) {
        rules.push(Box::new(AfterDate(after.clone(), fmt.clone())));
    }
    if let (Some(before), Some(fmt)) = (&schema.before, &schema.date_format) {
        rules.push(Box::new(BeforeDate(before.clone(), fmt.clone())));
    }
    if let (Some(v), Some(fmt)) = (&schema.between_dates, &schema.date_format) {
        rules.push(Box::new(BetweenDates(v[0].clone(), v[1].clone(), fmt.clone())));
    }
    rules
}

pub fn validate_column(
    column_name: &str,
    values: &[&str],
    schema: &ColumnSchema,
) -> Vec<RuleResults> {
    let rules = build_rules(schema);
    let mut results = vec![];

    for rule in &rules {
        let rule_name = rule.name();
        let mut failed_values = vec![];

        for value in values {
            if let Some(_) = rule.validate(value) {
                failed_values.push(value.to_string());
            }
        }

        let pass = failed_values.is_empty();
        results.push(RuleResults {
            column_name: column_name.to_string(),
            rule_name,
            pass,
            failed_values,
        });
    }

    results
}

pub fn validate(
    schema: &Schema,
    data: &HashMap<String, Vec<String>>,
) -> ValidationReport {
    let mut results = vec![];

    for (column_name, column_schema) in &schema.columns {
        if let Some(values) = data.get(column_name) {
            let refs: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
            let mut column_results = validate_column(column_name, &refs, column_schema);

            if column_schema.unique == Some(true) {
                let errors = validate_unique(&refs);
                if !errors.is_empty() {
                    column_results.push(RuleResults {
                        column_name: column_name.clone(),
                        rule_name: "Unique".to_string(),
                        pass: false,
                        failed_values: errors,
                    });
                } else {
                    column_results.push(RuleResults {
                        column_name: column_name.clone(),
                        rule_name: "Unique".to_string(),
                        pass: true,
                        failed_values: vec![],
                    });
                }
            }

            results.extend(column_results);
        }
    }

    let checks_pass = results.iter().all(|r| r.pass);
    ValidationReport { checks_pass, results }
}
