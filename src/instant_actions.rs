//! The `instantActions` message: actions the mobile robot executes as soon as they arrive.

use serde::{Deserialize, Serialize};

pub use crate::common::ActionParameter;

/// The `instantActions` message (topic `/instantActions`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstantActions {
    /// Header ID of the message. Defined per topic, incremented by 1 with each sent message.
    pub header_id: u32,
    /// Timestamp in ISO 8601 format (`YYYY-MM-DDTHH:mm:ss.fffZ`).
    pub timestamp: String,
    /// Version of the protocol, `[Major].[Minor].[Patch]`.
    pub version: String,
    /// Manufacturer of the mobile robot.
    pub manufacturer: String,
    /// Serial number of the mobile robot.
    pub serial_number: String,
    /// The instant actions to execute.
    pub actions: Vec<InstantAction>,
}

/// An action that the mobile robot can perform, sent as an instant action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstantAction {
    /// Type of the action. Identifies the function of the action.
    pub action_type: String,
    /// Unique ID to identify the action and map it to the actionState in the state message.
    pub action_id: String,
    /// User-defined, human-readable name. Not to be used for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_descriptor: Option<String>,
    /// Always `NONE` for instant actions: may run in parallel with others, including movement.
    pub blocking_type: InstantBlockingType,
    /// Parameters for the action, e.g. deviceId, loadId, external triggers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<Vec<ActionParameter>>,
    /// `true`: the action can enter RETRIABLE if it fails. `false` (default): it enters FAILED.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retriable: Option<bool>,
}

/// Blocking type of an instant action. The schema allows only `NONE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstantBlockingType {
    /// The action can happen in parallel with others, including movement.
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"{"headerId":3,"timestamp":"2017-04-15T11:40:03.123Z","version":"3.0.1","manufacturer":"ACME","serialNumber":"R-001","actions":[{"actionType":"beep","actionId":"a1","blockingType":"NONE","actionParameters":[{"key":"duration","value":103.2},{"key":"signal","value":true},{"key":"list","value":["a","b"]},{"key":"obj","value":{"x":1}}],"retriable":false},{"actionType":"stopPause","actionId":"a2","actionDescriptor":"resume","blockingType":"NONE"}]}"#;

    #[test]
    fn round_trip() {
        let msg: InstantActions = serde_json::from_str(EXAMPLE).unwrap();
        assert_eq!(msg.actions[1].action_parameters, None);
        let back = serde_json::to_value(&msg).unwrap();
        let orig: serde_json::Value = serde_json::from_str(EXAMPLE).unwrap();
        assert_eq!(back, orig);
    }

    #[test]
    fn rejects_other_blocking_types() {
        let bad = EXAMPLE.replace("\"NONE\"", "\"HARD\"");
        assert!(serde_json::from_str::<InstantActions>(&bad).is_err());
    }
}
