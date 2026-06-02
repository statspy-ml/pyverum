use pyverum::schema::{ColumnSchema, Schema};
use pyverum::validator::{build_rules, validate};
use std::collections::HashMap;

#[test]
fn test_not_null_rule() {
    let schema = ColumnSchema {
        not_null: Some(true),
        ..Default::default()
    };

    let rules = build_rules(&schema);
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].validate("hello"), None);
    assert!(rules[0].validate("").is_some());
}

#[test]
fn test_validate_report_passes() {
    let toml = r#"
        [columns.age]
        dtype = "float"
        gt = 0.0
        lt = 120.0

        [columns.name]
        dtype = "string"
        not_null = true
        min_length = 2
    "#;

    let schema = Schema::from_toml(toml).unwrap();

    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    data.insert("age".to_string(), vec!["25".to_string(), "40".to_string(), "70".to_string()]);
    data.insert("name".to_string(), vec!["Alice".to_string(), "Bob".to_string(), "Carol".to_string()]);

    let report = validate(&schema, &data);

    assert!(report.checks_pass);
    assert!(report.results.iter().all(|r| r.pass));
}

#[test]
fn test_validate_report_fails() {
    let toml = r#"
        [columns.age]
        dtype = "float"
        gt = 0.0
        lt = 120.0

        [columns.name]
        dtype = "string"
        not_null = true
        min_length = 2
    "#;

    let schema = Schema::from_toml(toml).unwrap();

    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    data.insert("age".to_string(), vec!["25".to_string(), "-5".to_string(), "200".to_string()]);
    data.insert("name".to_string(), vec!["Alice".to_string(), "".to_string(), "B".to_string()]);

    let report = validate(&schema, &data);

    assert!(!report.checks_pass);

    let age_gt = report.results.iter().find(|r| r.column_name == "age" && r.rule_name == "GreaterThan(0)").unwrap();
    assert!(!age_gt.pass);
    assert_eq!(age_gt.failed_values, vec!["-5"]);

    let age_lt = report.results.iter().find(|r| r.column_name == "age" && r.rule_name == "LessThan(120)").unwrap();
    assert!(!age_lt.pass);
    assert_eq!(age_lt.failed_values, vec!["200"]);

    let name_min = report.results.iter().find(|r| r.column_name == "name" && r.rule_name == "MinLength(2)").unwrap();
    assert!(!name_min.pass);
    assert!(name_min.failed_values.contains(&"B".to_string()));
}
