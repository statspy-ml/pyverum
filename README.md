# pyverum

A fast, Rust-powered data validation library for Python — inspired by [Pandera](https://pandera.readthedocs.io/) and [Great Expectations](https://greatexpectations.io/).

Define your data quality rules in a TOML schema, run validation against your datasets, and get a detailed report showing exactly what passed and what failed.

## Stack

- **Rust** — core validation engine
- **PyO3** — Python bindings
- **serde + toml** — schema deserialization
- **chrono** — date validation
- **regex** — regex pattern validation
- **maturin** — build and publish

## Installation

```bash
pip install pyverum
# or
uv add pyverum
```

## Quick Start

```python
import pyverum

schema = """
[columns.age]
dtype = "float"
gt = 0.0
lt = 120.0

[columns.name]
dtype = "string"
not_null = true
min_length = 2
"""

data = {
    "age":  ["25", "-5", "200"],
    "name": ["Alice", "", "B"],
}

report = pyverum.validate(schema, data)
print(report)
```

Output:
```
Validation: FAILED
Summary: 4 rules | 1 passed | 3 failed
----------------------------------------
[age] GreaterThan(0) — ✗ FAIL
    failed values: ["-5"]
[age] LessThan(120) — ✗ FAIL
    failed values: ["200"]
[name] NotNull — ✓ PASS
[name] MinLength(2) — ✗ FAIL
    failed values: ["", "B"]
```

## Schema Reference

All rules are defined per column inside a TOML schema. Each column must have a `dtype` field.

### Presence

| Rule | Type | Description |
|---|---|---|
| `not_null` | `bool` | Value must not be null or empty |
| `not_empty` | `bool` | Value must not be an empty string |
| `unique` | `bool` | All values in the column must be unique |

```toml
[columns.name]
dtype = "string"
not_null = true
not_empty = true
unique = true
```

### Numeric

| Rule | Type | Description |
|---|---|---|
| `gt` | `float` | Greater than |
| `ge` | `float` | Greater than or equal |
| `lt` | `float` | Less than |
| `le` | `float` | Less than or equal |
| `equal` | `float` | Equal to |
| `between` | `[float, float]` | Inclusive range |
| `is_in` | `[string]` | Value must be in the list |
| `is_positive` | `bool` | Must be > 0 |
| `is_negative` | `bool` | Must be < 0 |
| `is_finite` | `bool` | Must not be inf or NaN |

```toml
[columns.age]
dtype = "float"
gt = 0.0
lt = 120.0
is_positive = true
is_finite = true

[columns.score]
dtype = "float"
between = [0.0, 10.0]

[columns.temperature]
dtype = "float"
ge = -273.15

[columns.category]
dtype = "string"
is_in = ["A", "B", "C"]

[columns.balance]
dtype = "float"
is_negative = true
```

### String

| Rule | Type | Description |
|---|---|---|
| `min_length` | `int` | Minimum string length |
| `max_length` | `int` | Maximum string length |
| `length_between` | `[int, int]` | Inclusive length range |
| `contains` | `string` | Must contain substring |
| `starts_with` | `string` | Must start with prefix |
| `ends_with` | `string` | Must end with suffix |
| `matches_regex` | `string` | Must match regex pattern |

```toml
[columns.name]
dtype = "string"
min_length = 2
max_length = 50

[columns.email]
dtype = "string"
contains = "@"
ends_with = ".com"

[columns.code]
dtype = "string"
matches_regex = "^[A-Z]{2}[0-9]{3}$"
length_between = [5, 5]

[columns.prefix]
dtype = "string"
starts_with = "ID_"
```

### Date

Date rules require `date_format` to be set. The format follows [strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) syntax.

| Rule | Type | Description |
|---|---|---|
| `date_format` | `string` | Value must parse as a date in this format |
| `after` | `string` | Date must be after this value |
| `before` | `string` | Date must be before this value |
| `between_dates` | `[string, string]` | Date must be within this range (inclusive) |

```toml
[columns.birthdate]
dtype = "string"
date_format = "%Y-%m-%d"
after = "1900-01-01"
before = "2010-01-01"

[columns.created_at]
dtype = "string"
date_format = "%Y-%m-%d"
between_dates = ["2020-01-01", "2025-12-31"]
```

## Complete Example

```python
import pyverum

schema = """
[columns.age]
dtype = "float"
gt = 0.0
lt = 120.0
is_positive = true
is_finite = true
not_null = true

[columns.salary]
dtype = "float"
between = [1000.0, 50000.0]
is_positive = true

[columns.score]
dtype = "float"
ge = 0.0
le = 10.0

[columns.category]
dtype = "string"
is_in = ["A", "B", "C"]
not_empty = true

[columns.name]
dtype = "string"
not_null = true
min_length = 2
max_length = 50

[columns.code]
dtype = "string"
matches_regex = "^[A-Z]{2}[0-9]{3}$"
length_between = [5, 5]

[columns.birthdate]
dtype = "string"
date_format = "%Y-%m-%d"
after = "1900-01-01"
before = "2010-01-01"
"""

data = {
    "age":       ["25", "200", "-5", "30"],
    "salary":    ["5000.0", "999.0", "60000.0", "20000.0"],
    "score":     ["10.0", "5.0", "8.0", "10.0"],
    "category":  ["A", "D", "B", "C"],
    "name":      ["Alice", "Bob", "Anne", "Al"],
    "code":      ["AB123", "A1234", "AB123", "CD999"],
    "birthdate": ["1990-05-01", "2020-01-01", "1985-12-31", "1800-01-01"],
}

report = pyverum.validate(schema, data)
print(report)
```

## Using with pandas

```python
import pandas as pd
import pyverum

df = pd.DataFrame({
    "age":  [25, -5, 200],
    "name": ["Alice", "", "Bob"],
})

# convert DataFrame columns to dict[str, list[str]]
data = {col: df[col].astype(str).tolist() for col in df.columns}

report = pyverum.validate(schema, data)
print(report)
```

## Using in CI/CD

```python
import sys
import pyverum

report = pyverum.validate(schema, data)

if not report.checks_pass:
    print(report)
    sys.exit(1)
```

## Report Structure

The `ValidationReport` object has:

- `report.checks_pass` — `True` if all rules passed
- `report.results` — list of `RuleResult` objects

Each `RuleResult` has:

- `result.column_name` — column being validated
- `result.rule_name` — rule name with parameters (e.g. `GreaterThan(0)`)
- `result.pass` — `True` if the rule passed
- `result.failed_values` — list of values that failed

```python
for result in report.results:
    if not result.pass:
        print(f"{result.column_name} / {result.rule_name}: {result.failed_values}")
```

## License

MIT
