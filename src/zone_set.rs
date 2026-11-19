//! The `zoneSet` message: the zones (areas with special rules) of a map.
//!
//! In the schema, parameters depend on the zone type (`if`/`then`). Here a [`Zone`] holds the
//! common fields and a [`ZoneKind`] enum, tagged by `zoneType`, holds the type-specific ones.

use serde::{Deserialize, Serialize};

pub use crate::common::{ActionParameter, BlockingType};

/// The `zoneSet` message (topic `/zoneSet`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneSetMessage {
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
    /// The zone set.
    pub zone_set: ZoneSet,
}

/// A zone set detailing a dedicated map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneSet {
    /// Globally unique identifier of the map the zone set particularizes.
    pub map_id: String,
    /// Globally unique identifier of the zone set.
    pub zone_set_id: String,
    /// User-defined, human-readable name or descriptor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone_set_descriptor: Option<String>,
    /// The zones.
    pub zones: Vec<Zone>,
}

/// A zone: an area of the map with special rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zone {
    /// Identifier, unique within the zone set.
    pub zone_id: String,
    /// User-defined, human-readable name or descriptor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone_descriptor: Option<String>,
    /// Vertices (x-y-coordinates) defining the shape of the zone. At least 3.
    pub vertices: Vec<Vertex>,
    /// Zone type and its type-specific parameters (`zoneType` plus further fields in the JSON).
    #[serde(flatten)]
    pub kind: ZoneKind,
}

/// Zone category with its type-specific parameters, tagged by `zoneType` in the JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "zoneType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ZoneKind {
    /// The robot must not enter the zone.
    Blocked,
    /// Line-guided robots follow the lines in the zone.
    LineGuided,
    /// Entering requires a release from fleet control.
    #[serde(rename_all = "camelCase")]
    Release {
        /// What the robot does when it loses the release.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        release_loss_behavior: Option<ZoneReleaseLossBehavior>,
    },
    /// Fleet control may replan the robot's trajectory within the zone.
    CoordinatedReplanning,
    /// Speed limit within the zone.
    #[serde(rename_all = "camelCase")]
    SpeedLimit {
        /// Maximum speed in the zone, m/s.
        maximum_speed: f64,
    },
    /// Actions on entering, while crossing and on leaving the zone.
    #[serde(rename_all = "camelCase")]
    Action {
        /// Actions when entering the zone.
        entry_actions: Vec<ZoneAction>,
        /// Actions while crossing the zone (empty if none).
        during_actions: Vec<ZoneAction>,
        /// Actions when leaving the zone (empty if none).
        exit_actions: Vec<ZoneAction>,
    },
    /// Preferred area for path planning.
    #[serde(rename_all = "camelCase")]
    Priority {
        /// Preference over an area with no zone: `0.0` none, `1.0` maximum.
        priority_factor: f64,
    },
    /// Area to avoid in path planning.
    #[serde(rename_all = "camelCase")]
    Penalty {
        /// Penalty over an area with no zone: `0.0` none, `1.0` maximum.
        penalty_factor: f64,
    },
    /// Preferred direction of travel.
    #[serde(rename_all = "camelCase")]
    Directed {
        /// Preferred direction of travel in rad, in the project coordinate system.
        direction: f64,
        /// How strictly the direction is to be kept.
        directed_limitation: DirectedLimitation,
    },
    /// Direction of travel, or its opposite (+π).
    #[serde(rename_all = "camelCase")]
    Bidirected {
        /// Direction in rad; it and its opposite are the allowed directions of travel.
        direction: f64,
        /// How strictly the directions are to be kept.
        bidirected_limitation: BidirectedLimitation,
    },
}

/// Behaviour of the robot when it loses the release of a `RELEASE` zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ZoneReleaseLossBehavior {
    /// Stop.
    Stop,
    /// Continue.
    Continue,
    /// Evacuate the zone.
    Evacuate,
}

/// Limitation of a `DIRECTED` zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DirectedLimitation {
    /// May deviate from the direction, but should avoid it.
    Soft,
    /// May deviate (e.g. for obstacles), but never travel opposite to the direction.
    Restricted,
    /// Keep the direction as precisely as technically possible.
    Strict,
}

/// Limitation of a `BIDIRECTED` zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BidirectedLimitation {
    /// May deviate from the directions, but should avoid it.
    Soft,
    /// No other direction than the given ones, except for obstacle avoidance.
    Restricted,
}

/// A vertex of a zone polygon, in the project-specific coordinate system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    /// X-coordinate in m.
    pub x: f64,
    /// Y-coordinate in m.
    pub y: f64,
}

/// An action executed by the robot on entering, crossing or leaving an `ACTION` zone.
/// Unlike an order action it has no `actionId`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneAction {
    /// Type of the action. Identifies the function of the action.
    pub action_type: String,
    /// User-defined, human-readable name. Not to be used for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_descriptor: Option<String>,
    /// Whether the action may run in parallel with others and with movement.
    pub blocking_type: BlockingType,
    /// Parameters for the action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<Vec<ActionParameter>>,
    /// `true`: the action can enter RETRIABLE if it fails. `false` (default): it enters FAILED.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retriable: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zone(extra: &str) -> String {
        format!(
            r#"{{"zoneId":"z","vertices":[{{"x":0,"y":0}},{{"x":1,"y":0}},{{"x":1,"y":1}}],{extra}}}"#
        )
    }

    #[test]
    fn speed_limit_requires_maximum_speed() {
        assert!(serde_json::from_str::<Zone>(&zone(r#""zoneType":"SPEED_LIMIT""#)).is_err());
        let z: Zone =
            serde_json::from_str(&zone(r#""zoneType":"SPEED_LIMIT","maximumSpeed":0.5"#)).unwrap();
        assert_eq!(z.kind, ZoneKind::SpeedLimit { maximum_speed: 0.5 });
    }

    #[test]
    fn unit_zone_types_serialise_with_tag() {
        let z: Zone = serde_json::from_str(&zone(r#""zoneType":"BLOCKED""#)).unwrap();
        assert_eq!(z.kind, ZoneKind::Blocked);
        let v = serde_json::to_value(&z).unwrap();
        assert_eq!(v["zoneType"], "BLOCKED");
    }

    #[test]
    fn unknown_zone_type_is_rejected() {
        assert!(serde_json::from_str::<Zone>(&zone(r#""zoneType":"MAGIC""#)).is_err());
    }
}
