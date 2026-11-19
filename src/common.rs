//! Types shared by several messages.

use serde::{Deserialize, Serialize};

/// A NURBS trajectory. Segments reach from the point where the robot starts to enter the edge
/// to the point where it reports that the next node was traversed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trajectory {
    /// Degree of the curve (minimum 1). Default if absent: 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degree: Option<u32>,
    /// Knot vector: control points + degree + 1 values, each in `0.0..=1.0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knot_vector: Option<Vec<f64>>,
    /// Control points of the NURBS, including the beginning and end point.
    pub control_points: Vec<ControlPoint>,
}

/// A control point of a NURBS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlPoint {
    /// X-coordinate in m.
    pub x: f64,
    /// Y-coordinate in m.
    pub y: f64,
    /// Weight with which the point pulls on the curve (greater than 0). Default if absent: 1.0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

/// A key/value parameter of an action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionParameter {
    /// The key of the action parameter, e.g. `duration`, `direction`, `signal`.
    pub key: String,
    /// The value: any JSON value (array, boolean, number, string or object).
    pub value: serde_json::Value,
}

/// Blocking type of an action: whether it may run in parallel with other actions and movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockingType {
    /// Can run in parallel with other actions and movement.
    None,
    /// Can run in parallel with other actions, but movement is stopped.
    Soft,
    /// Can run in parallel with movement, but no other action.
    Single,
    /// Neither other actions nor movement may happen at the same time.
    Hard,
}

/// A path within the robot's currently active order, as a NURBS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannedPath {
    /// The trajectory as a NURBS.
    pub trajectory: Trajectory,
    /// `nodeId`s of the active order that are traversed within the shared planned path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traversed_nodes: Option<Vec<String>>,
}

/// Estimated times of arrival at closer waypoints, as a polyline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntermediatePath {
    /// End points of the segments of the polyline.
    pub polyline: Vec<PolylinePoint>,
}

/// End point of a polyline segment with its estimated time of arrival.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolylinePoint {
    /// X-coordinate in m, in the project-specific coordinate system.
    pub x: f64,
    /// Y-coordinate in m, in the project-specific coordinate system.
    pub y: f64,
    /// Absolute orientation in rad (`-π..=π`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// Estimated time of arrival/traversal, ISO 8601 UTC.
    pub eta: String,
}

/// Current position of the mobile robot on the map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileRobotPosition {
    /// X-coordinate in m.
    pub x: f64,
    /// Y-coordinate in m.
    pub y: f64,
    /// Orientation in rad (`-π..=π`).
    pub theta: f64,
    /// Unique identification of the map.
    pub map_id: String,
    /// `true`: localized, x/y/theta can be trusted. `false`: they cannot.
    pub localized: bool,
    /// Localization quality, `0.0` (lowest) to `1.0` (highest). For logging/visualization only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localization_score: Option<f64>,
    /// Position deviation range in m (at least 0). For logging/visualization only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deviation_range: Option<f64>,
}

/// Velocity of the mobile robot in the robot's own coordinates. All fields optional.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Velocity {
    /// Velocity in the robot's X-direction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vx: Option<f64>,
    /// Velocity in the robot's Y-direction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vy: Option<f64>,
    /// Turning speed around the robot's Z-axis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub omega: Option<f64>,
}

/// Reference point of a load's bounding box.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBoxReference {
    /// X in m.
    pub x: f64,
    /// Y in m.
    pub y: f64,
    /// Z in m.
    pub z: f64,
    /// Orientation of the bounding box (important for tuggers, trains).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
}

/// Dimensions of a load's bounding box in m.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadDimensions {
    /// Length along the robot's x-axis (at least 0).
    pub length: f64,
    /// Width along the robot's y-axis (at least 0).
    pub width: f64,
    /// Height (at least 0), only if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}
