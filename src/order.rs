//! The `order` message: fleet control's order (nodes and edges to traverse) for a mobile robot.

use serde::{Deserialize, Serialize};

pub use crate::common::{ActionParameter, BlockingType, ControlPoint, Trajectory};

/// The `order` message (topic `/order`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
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
    /// Order identification. Unique per mobile robot, used to tell orders apart.
    pub order_id: String,
    /// Order update identification, unique per `order_id`. Starts at 0 for a new order.
    pub order_update_id: u32,
    /// Additional human-readable information on the order. Not for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_description: Option<String>,
    /// The nodes of the order, ordered by `sequence_id`.
    pub nodes: Vec<Node>,
    /// The edges of the order, ordered by `sequence_id`.
    pub edges: Vec<Edge>,
}

/// A node of an order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// Unique node identification.
    pub node_id: String,
    /// Number to track the sequence of nodes and edges in an order and to simplify order updates.
    pub sequence_id: u32,
    /// Additional human-readable information on the node. Not for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_descriptor: Option<String>,
    /// `true`: the node is part of the base (fixed). `false`: part of the horizon (may change).
    pub released: bool,
    /// Position of the node on the map. Optional for vehicles that plan their own path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_position: Option<NodePosition>,
    /// Actions to execute at the node.
    pub actions: Vec<Action>,
}

/// Position of a node on the map, in the project-specific coordinate system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePosition {
    /// X-coordinate in m.
    pub x: f64,
    /// Y-coordinate in m.
    pub y: f64,
    /// Absolute orientation in rad (`-π..=π`) when on the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// Allowed deviation (ellipse) of the position in the XY plane.
    #[serde(rename = "allowedDeviationXY", default, skip_serializing_if = "Option::is_none")]
    pub allowed_deviation_xy: Option<AllowedDeviationXy>,
    /// Allowed deviation of theta in rad (`0..=π`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_deviation_theta: Option<f64>,
    /// Unique identification of the map the position refers to.
    pub map_id: String,
}

/// Ellipse of allowed deviation around a node position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowedDeviationXy {
    /// Semi-major axis in m (at least 0).
    pub a: f64,
    /// Semi-minor axis in m (at least 0).
    pub b: f64,
    /// Orientation of the ellipse in rad (`-π..=π`).
    pub theta: f64,
}

/// An edge of an order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    /// Unique edge identification.
    pub edge_id: String,
    /// Number to track the sequence of nodes and edges in an order.
    pub sequence_id: u32,
    /// Additional human-readable information on the edge. Not for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edge_descriptor: Option<String>,
    /// `true`: the edge is part of the base. `false`: part of the horizon.
    pub released: bool,
    /// Permitted maximum speed on the edge in m/s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_speed: Option<f64>,
    /// Permitted maximum height of the mobile robot including the load, in m.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_mobile_robot_height: Option<f64>,
    /// Minimum height of the load handling device on the edge, in m.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_load_handling_device_height: Option<f64>,
    /// Orientation of the robot on the edge in rad (`-π..=π`), see `orientation_type`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<f64>,
    /// How `orientation` is to be interpreted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation_type: Option<OrientationType>,
    /// Sets the direction at junctions for line-guided robots, e.g. `left`, `right`, `straight`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// `true`: the robot rotates to `orientation` before entering the edge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reach_orientation_before_entering: Option<bool>,
    /// Maximum rotation speed in rad/s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_rotation_speed: Option<f64>,
    /// Trajectory as a NURBS. Defines the path the robot has to follow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<Trajectory>,
    /// Length of the path from start node to end node, in m.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    /// Corridor the robot may leave the trajectory within, for free navigation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corridor: Option<Corridor>,
    /// Actions to execute on the edge.
    pub actions: Vec<Action>,
}

/// How an edge's `orientation` is to be interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrientationType {
    /// Relative to the global project-specific map coordinate system.
    Global,
    /// Tangential to the edge.
    Tangential,
}

/// Corridor boundaries along an edge, for free navigation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Corridor {
    /// Width to the left of the trajectory in m (at least 0).
    pub left_width: f64,
    /// Width to the right of the trajectory in m (at least 0).
    pub right_width: f64,
    /// Which point of the robot is kept within the corridor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corridor_reference_point: Option<CorridorReferencePoint>,
    /// `true`: leaving the corridor requires a release from fleet control.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_required: Option<bool>,
    /// What the robot does when it loses the release.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_loss_behavior: Option<ReleaseLossBehavior>,
}

/// Point of the robot that has to stay within a corridor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CorridorReferencePoint {
    /// The kinematic center of the robot.
    KinematicCenter,
    /// The contour of the robot.
    Contour,
}

/// Behaviour of the robot when a corridor release is lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReleaseLossBehavior {
    /// Stop.
    Stop,
    /// Return into the trajectory.
    Return,
}

/// An action the mobile robot performs at a node or on an edge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    /// Type of the action. Identifies the function of the action.
    pub action_type: String,
    /// Unique ID to identify the action and map it to the actionState in the state message.
    pub action_id: String,
    /// User-defined, human-readable name. Not to be used for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_descriptor: Option<String>,
    /// Whether the action may run in parallel with others and with movement.
    pub blocking_type: BlockingType,
    /// Parameters for the action, e.g. deviceId, loadId, external triggers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<Vec<ActionParameter>>,
    /// `true`: the action can enter RETRIABLE if it fails. `false` (default): it enters FAILED.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retriable: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocking_types() {
        for (s, b) in [("NONE", BlockingType::None), ("SOFT", BlockingType::Soft), ("SINGLE", BlockingType::Single), ("HARD", BlockingType::Hard)] {
            let v: BlockingType = serde_json::from_str(&format!("\"{s}\"")).unwrap();
            assert_eq!(v, b);
            assert_eq!(serde_json::to_string(&b).unwrap(), format!("\"{s}\""));
        }
    }
}
