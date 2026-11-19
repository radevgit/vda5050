//! The `visualization` message: mobile robot position and/or velocity for visualization.
//! May be published at a higher rate than `state`.

use serde::{Deserialize, Serialize};

pub use crate::common::{
    ControlPoint, IntermediatePath, MobileRobotPosition, PlannedPath, PolylinePoint, Trajectory,
    Velocity,
};

/// The `visualization` message (topic `/visualization`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visualization {
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
    /// Header ID of the state message this visualization message refers to.
    pub reference_state_header_id: u32,
    /// Path within the currently active order, as a NURBS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planned_path: Option<PlannedPath>,
    /// Estimated times of arrival at closer waypoints the robot perceives with its sensors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intermediate_path: Option<IntermediatePath>,
    /// Current position of the mobile robot on the map.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_robot_position: Option<MobileRobotPosition>,
    /// Velocity of the mobile robot in its own coordinates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Velocity>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL: &str = r#"{"headerId":5,"timestamp":"2017-04-15T11:40:03.123Z","version":"3.0.1","manufacturer":"ACME","serialNumber":"R-001","referenceStateHeaderId":42,"plannedPath":{"trajectory":{"degree":2,"knotVector":[0.0,0.0,0.0,1.0,1.0,1.0],"controlPoints":[{"x":0.0,"y":0.0},{"x":1.0,"y":0.5,"weight":2.0},{"x":2.0,"y":0.0}]},"traversedNodes":["n1","n2"]},"intermediatePath":{"polyline":[{"x":0.5,"y":0.1,"theta":1.5,"eta":"2017-04-15T11:40:04.000Z"},{"x":1.0,"y":0.2,"eta":"2017-04-15T11:40:05.000Z"}]},"mobileRobotPosition":{"x":0.1,"y":0.2,"theta":0.3,"mapId":"m1","localized":true,"localizationScore":0.9,"deviationRange":0.05},"velocity":{"vx":1.0,"vy":0.0,"omega":0.1}}"#;

    const MINIMAL: &str = r#"{"headerId":6,"timestamp":"2017-04-15T11:40:03.123Z","version":"3.0.1","manufacturer":"ACME","serialNumber":"R-001","referenceStateHeaderId":42}"#;

    fn round_trip(json: &str) -> Visualization {
        let msg: Visualization = serde_json::from_str(json).unwrap();
        let back = serde_json::to_value(&msg).unwrap();
        let orig: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(back, orig);
        msg
    }

    #[test]
    fn round_trip_full() {
        let msg = round_trip(FULL);
        assert_eq!(msg.planned_path.unwrap().trajectory.control_points.len(), 3);
    }

    #[test]
    fn round_trip_minimal() {
        let msg = round_trip(MINIMAL);
        assert!(msg.planned_path.is_none() && msg.velocity.is_none());
    }

    #[test]
    fn empty_velocity_is_valid() {
        let json = MINIMAL.replace("42}", "42,\"velocity\":{}}");
        round_trip(&json);
    }
}
