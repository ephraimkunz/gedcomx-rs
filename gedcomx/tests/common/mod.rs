use chrono::{DateTime, NaiveDateTime, ParseResult, Utc};
use gedcomx::Gedcomx;

#[cfg(test)]
use pretty_assertions::assert_eq;

pub fn assert_matching_json(gx: Gedcomx, filename: &str) {
    // Instead of comparing string, which may yield false negatives because of whitespace, etc,
    // we'll compare serde_json::Values, which is loosly typed json.

    let json_value = serde_json::to_value(&gx).unwrap();

    let expected_json = std::fs::read_to_string(format!("../data/{}.json", filename)).unwrap();
    let expected_value: serde_json::Value = serde_json::from_str(&expected_json).unwrap();

    assert_eq!(json_value, expected_value);
    assert_eq!(serde_json::from_str::<Gedcomx>(&expected_json).unwrap(), gx)
}

#[allow(dead_code)]
pub fn parse(s: &str) -> ParseResult<DateTime<Utc>> {
    let date_time = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")?;
    Ok(DateTime::from_utc(date_time, Utc))
}
