//! The `connection` message: the last-will / connection-state message of a mobile robot.
//!
//! Sent with the retain flag. The robot publishes it with [`ConnectionState::Online`] when it
//! connects; the broker publishes the configured last will with
//! [`ConnectionState::ConnectionBroken`] if the connection ends unexpectedly.

use serde::{Deserialize, Serialize};

/// The `connection` message (topic `/connection`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
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
    /// State of the connection between mobile robot and broker.
    pub connection_state: ConnectionState,
}

/// State of the connection between mobile robot and broker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConnectionState {
    /// Connection between mobile robot and broker is active.
    Online,
    /// Connection has gone offline in a coordinated way.
    Offline,
    /// Connection is active, but the robot does not send state messages (power saving).
    Hibernating,
    /// The connection has unexpectedly ended.
    ConnectionBroken,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"{"headerId":1,"timestamp":"2017-04-15T11:40:03.123Z","version":"3.0.1","manufacturer":"ACME","serialNumber":"R-001","connectionState":"CONNECTION_BROKEN"}"#;

    #[test]
    fn round_trip() {
        let msg: Connection = serde_json::from_str(EXAMPLE).unwrap();
        assert_eq!(msg.connection_state, ConnectionState::ConnectionBroken);
        let back: serde_json::Value = serde_json::to_value(&msg).unwrap();
        let orig: serde_json::Value = serde_json::from_str(EXAMPLE).unwrap();
        assert_eq!(back, orig);
    }
}
