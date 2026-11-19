//! Validates example messages, and what this crate serialises from them, against the
//! vendored official JSON schemas (`spec/schemas`).

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

fn read(path: &str) -> Value {
    let text = std::fs::read_to_string(format!("{}/{path}", env!("CARGO_MANIFEST_DIR")))
        .unwrap_or_else(|e| panic!("{path}: {e}"));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn validator(schema: &str) -> jsonschema::Validator {
    jsonschema::options()
        .should_validate_formats(true)
        .build(&read(&format!("spec/schemas/{schema}.schema")))
        .unwrap_or_else(|e| panic!("schema {schema}: {e}"))
}

fn assert_valid(v: &jsonschema::Validator, what: &str, instance: &Value) {
    let errors: Vec<String> = v
        .iter_errors(instance)
        .map(|e| format!("{e} at {}", e.instance_path()))
        .collect();
    assert!(
        errors.is_empty(),
        "{what} is not valid:\n{}",
        errors.join("\n")
    );
}

/// example file -> valid against schema -> typed -> JSON again -> identical and still valid.
fn check<T: Serialize + DeserializeOwned>(name: &str) {
    let v = validator(name);
    let example = read(&format!("tests/examples/{name}.json"));
    assert_valid(&v, &format!("example {name}"), &example);
    let typed: T = serde_json::from_value(example.clone()).unwrap();
    let back = serde_json::to_value(&typed).unwrap();
    assert_valid(&v, &format!("serialised {name}"), &back);
    assert_eq!(back, example, "{name} round trip differs");
}

#[test]
fn connection() {
    check::<vda5050::connection::Connection>("connection");
}

#[test]
fn responses() {
    check::<vda5050::responses::Responses>("responses");
}

#[test]
fn instant_actions() {
    check::<vda5050::instant_actions::InstantActions>("instantActions");
}

#[test]
fn order() {
    check::<vda5050::order::Order>("order");
}

#[test]
fn zone_set() {
    check::<vda5050::zone_set::ZoneSetMessage>("zoneSet");
}

#[test]
fn state() {
    check::<vda5050::state::State>("state");
}

/// Only the required fields: optional ones must stay absent after a round trip.
#[test]
fn state_minimal() {
    let v = validator("state");
    let example = read("tests/examples/state_minimal.json");
    assert_valid(&v, "state_minimal example", &example);
    let typed: vda5050::state::State = serde_json::from_value(example.clone()).unwrap();
    assert_eq!(serde_json::to_value(&typed).unwrap(), example);
}

#[test]
fn factsheet() {
    check::<vda5050::factsheet::Factsheet>("factsheet");
}

/// Required fields only (empty optional objects stay empty).
#[test]
fn factsheet_minimal() {
    let v = validator("factsheet");
    let example = read("tests/examples/factsheet_minimal.json");
    assert_valid(&v, "factsheet_minimal example", &example);
    let typed: vda5050::factsheet::Factsheet = serde_json::from_value(example.clone()).unwrap();
    assert_eq!(serde_json::to_value(&typed).unwrap(), example);
}

#[test]
fn visualization() {
    check::<vda5050::visualization::Visualization>("visualization");
}

/// The validator must really reject things, otherwise the tests above prove nothing.
#[test]
fn validator_rejects_bad_messages() {
    let v = validator("connection");
    let mut bad = read("tests/examples/connection.json");
    bad["connectionState"] = "SLEEPING".into();
    assert!(!v.is_valid(&bad), "bad enum accepted");
    let mut bad = read("tests/examples/connection.json");
    bad["timestamp"] = "yesterday".into();
    assert!(
        !v.is_valid(&bad),
        "bad date-time accepted (formats not checked?)"
    );
    let mut bad = read("tests/examples/connection.json");
    bad["headerId"] = (-1).into();
    assert!(!v.is_valid(&bad), "negative headerId accepted");
}

/// Same for rules reached through `$ref` into `$defs` (draft 2020-12 resolution).
#[test]
fn validator_follows_defs_refs() {
    let v = validator("visualization");
    let mut bad = read("tests/examples/visualization.json");
    bad["mobileRobotPosition"]["localized"] = "yes".into();
    assert!(!v.is_valid(&bad), "bad value behind $ref accepted");
}
