//! The `responses` message: fleet control's answers to a mobile robot's requests.

use serde::{Deserialize, Serialize};

/// The `responses` message (topic `/responses`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Responses {
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
    /// Array of response objects.
    pub responses: Vec<Response>,
}

/// One response to a request of the mobile robot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Unique per mobile robot identifier within all active requests.
    pub request_id: String,
    /// Fleet control's decision on the request.
    pub grant_type: GrantType,
    /// Timestamp (ISO 8601) at which the lease expires. Only to be sent with responses
    /// granting a request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_expiry: Option<String>,
}

/// Fleet control's decision on a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GrantType {
    /// Fleet control grants the request.
    Granted,
    /// Fleet control acknowledges the request, but no permission is given yet.
    Queued,
    /// Fleet control revokes a previously granted request.
    Revoked,
    /// Fleet control rejects the request.
    Rejected,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"{"headerId":7,"timestamp":"2017-04-15T11:40:03.123Z","version":"3.0.1","manufacturer":"ACME","serialNumber":"R-001","responses":[{"requestId":"r1","grantType":"GRANTED","leaseExpiry":"2017-04-15T11:41:03.123Z"},{"requestId":"r2","grantType":"QUEUED"}]}"#;

    #[test]
    fn round_trip() {
        let msg: Responses = serde_json::from_str(EXAMPLE).unwrap();
        assert_eq!(msg.responses[0].grant_type, GrantType::Granted);
        assert_eq!(msg.responses[1].lease_expiry, None);
        let back = serde_json::to_value(&msg).unwrap();
        let orig: serde_json::Value = serde_json::from_str(EXAMPLE).unwrap();
        assert_eq!(back, orig);
    }
}
