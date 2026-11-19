//! The `factsheet` message: the robot's capabilities, limits and geometry.
//!
//! Fields the schema calls "extensible enums" (kinematics, class, localization and navigation
//! types, wheel type) are kept as `String`, with the known values in the docs, so that a
//! factsheet with a manufacturer-specific value still parses.

use serde::{Deserialize, Serialize};

pub use crate::common::BlockingType;

/// The `factsheet` message (topic `/factsheet`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Factsheet {
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
    /// Class and capabilities of the mobile robot.
    pub type_specification: TypeSpecification,
    /// Basic physical properties.
    pub physical_parameters: PhysicalParameters,
    /// Protocol limitations of the robot.
    pub protocol_limits: ProtocolLimits,
    /// Supported protocol features.
    pub protocol_features: ProtocolFeatures,
    /// Detailed geometry.
    pub mobile_robot_geometry: MobileRobotGeometry,
    /// Abstract specification of load capabilities.
    pub load_specification: LoadSpecification,
    /// Hardware/software versions, network and battery charging parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_robot_configuration: Option<MobileRobotConfiguration>,
}

/// Class and capabilities of the mobile robot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeSpecification {
    /// Free-text series name as specified by the manufacturer.
    pub series_name: String,
    /// Free-text description of the robot type series.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_description: Option<String>,
    /// Kinematics type. Extensible enum: `DIFFERENTIAL`, `OMNIDIRECTIONAL`, `THREE_WHEEL`.
    pub mobile_robot_kinematics: String,
    /// Robot class. Extensible enum: `FORKLIFT`, `CONVEYOR`, `TUGGER`, `CARRIER`.
    pub mobile_robot_class: String,
    /// Maximum loadable mass in kg (at least 0).
    pub maximum_load_mass: f64,
    /// Localization types. Extensible enum: `NATURAL`, `REFLECTOR`, `RFID`, `DMC`, `SPOT`, `GRID`.
    pub localization_types: Vec<String>,
    /// Path planning types, sorted by priority. Extensible enum: `PHYSICAL_LINE_GUIDED`,
    /// `VIRTUAL_LINE_GUIDED`, `FREELY_NAVIGATING`.
    pub navigation_types: Vec<String>,
    /// Zone types supported by the robot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supported_zones: Option<Vec<SupportedZone>>,
}

/// Zone type a robot supports. Same values as the `zoneType` of a zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SupportedZone {
    /// `BLOCKED`
    Blocked,
    /// `LINE_GUIDED`
    LineGuided,
    /// `RELEASE`
    Release,
    /// `COORDINATED_REPLANNING`
    CoordinatedReplanning,
    /// `SPEED_LIMIT`
    SpeedLimit,
    /// `ACTION`
    Action,
    /// `PRIORITY`
    Priority,
    /// `PENALTY`
    Penalty,
    /// `DIRECTED`
    Directed,
    /// `BIDIRECTED`
    Bidirected,
}

/// Basic physical properties of the mobile robot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalParameters {
    /// Minimal controlled continuous speed in m/s.
    pub minimum_speed: f64,
    /// Maximum speed in m/s.
    pub maximum_speed: f64,
    /// Minimal controlled continuous rotation speed in rad/s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_angular_speed: Option<f64>,
    /// Maximum rotation speed in rad/s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_angular_speed: Option<f64>,
    /// Maximum acceleration with maximum load in m/s².
    pub maximum_acceleration: f64,
    /// Maximum deceleration with maximum load in m/s².
    pub maximum_deceleration: f64,
    /// Minimum height of the robot in m.
    pub minimum_height: f64,
    /// Maximum height of the robot in m.
    pub maximum_height: f64,
    /// Width in m.
    pub width: f64,
    /// Length in m.
    pub length: f64,
}

/// Protocol limitations. A parameter that is absent or 0 means no explicit limit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolLimits {
    /// Maximum lengths of strings.
    pub maximum_string_lengths: MaximumStringLengths,
    /// Maximum lengths of arrays.
    pub maximum_array_lengths: MaximumArrayLengths,
    /// Timing information.
    pub timing: Timing,
}

/// Maximum lengths of strings. All fields optional.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaximumStringLengths {
    /// Maximum MQTT message length.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_message_length: Option<u32>,
    /// Maximum length of the serial-number part in MQTT topics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_topic_serial_length: Option<u32>,
    /// Maximum length of all other parts in MQTT topics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_topic_element_length: Option<u32>,
    /// Maximum length of ID strings (orderId, nodeId, actionId, edgeId, mapId, zoneId, …).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_id_length: Option<u32>,
    /// If `true`, ID strings must contain numerical values only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_numerical_only: Option<bool>,
    /// Maximum length of loadId strings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_load_id_length: Option<u32>,
}

/// Maximum lengths of arrays. All fields optional; JSON keys contain dots, e.g. `order.nodes`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaximumArrayLengths {
    /// Maximum number of nodes per order.
    #[serde(
        rename = "order.nodes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_nodes: Option<u32>,
    /// Maximum number of edges per order.
    #[serde(
        rename = "order.edges",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_edges: Option<u32>,
    /// Maximum number of actions per node.
    #[serde(
        rename = "node.actions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_actions: Option<u32>,
    /// Maximum number of actions per edge.
    #[serde(
        rename = "edge.actions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_actions: Option<u32>,
    /// Maximum number of parameters per action.
    #[serde(
        rename = "action.actionParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_action_parameters: Option<u32>,
    /// Maximum number of instant actions per message.
    #[serde(
        rename = "instantActions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instant_actions: Option<u32>,
    /// Maximum number of knots per trajectory.
    #[serde(
        rename = "trajectory.knotVector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trajectory_knot_vector: Option<u32>,
    /// Maximum number of control points per trajectory.
    #[serde(
        rename = "trajectory.controlPoints",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trajectory_control_points: Option<u32>,
    /// Maximum number of zones per zone set.
    #[serde(
        rename = "zoneSet.zones",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zone_set_zones: Option<u32>,
    /// Maximum number of nodeStates sent (= nodes in the base).
    #[serde(
        rename = "state.nodeStates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_node_states: Option<u32>,
    /// Maximum number of edgeStates sent (= edges in the base).
    #[serde(
        rename = "state.edgeStates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_edge_states: Option<u32>,
    /// Maximum number of load objects sent.
    #[serde(
        rename = "state.loads",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_loads: Option<u32>,
    /// Maximum number of actionStates sent.
    #[serde(
        rename = "state.actionStates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_action_states: Option<u32>,
    /// Maximum number of instantActionStates sent.
    #[serde(
        rename = "state.instantActionStates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_instant_action_states: Option<u32>,
    /// Maximum number of zoneActionStates sent.
    #[serde(
        rename = "state.zoneActionStates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_zone_action_states: Option<u32>,
    /// Maximum number of errors per state message.
    #[serde(
        rename = "state.errors",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_errors: Option<u32>,
    /// Maximum number of information objects per state message.
    #[serde(
        rename = "state.information",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_information: Option<u32>,
    /// Maximum number of error references per error.
    #[serde(
        rename = "error.errorReferences",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_error_references: Option<u32>,
    /// Maximum number of info references per information.
    #[serde(
        rename = "information.infoReferences",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub information_info_references: Option<u32>,
}

/// Timing information, in seconds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    /// Minimum interval for sending order messages to the robot, s.
    pub minimum_order_interval: f64,
    /// Minimum interval for sending state messages, s.
    pub minimum_state_interval: f64,
    /// Default interval for state messages, s. If absent the default of the spec applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_state_interval: Option<f64>,
    /// Default interval for messages on the visualization topic, s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visualization_interval: Option<f64>,
}

/// Supported features of the VDA 5050 protocol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolFeatures {
    /// Supported and/or required optional parameters. Unlisted ones are not supported.
    pub optional_parameters: Vec<OptionalParameter>,
    /// All actions with parameters supported by the robot (standard and manufacturer-specific).
    pub mobile_robot_actions: Vec<MobileRobotAction>,
}

/// Support for one optional parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalParameter {
    /// Full name, e.g. `order.nodes.nodePosition.allowedDeviationTheta`.
    pub parameter: String,
    /// Type of support.
    pub support: Support,
    /// Free-text description, e.g. why the parameter is needed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Type of support for an optional parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Support {
    /// Supported as specified.
    Supported,
    /// Supported and required for the robot to work.
    Required,
}

/// An action supported by the robot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileRobotAction {
    /// Unique action type, corresponding to `action.actionType`.
    pub action_type: String,
    /// Free-text description of the action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_description: Option<String>,
    /// Allowed scopes of the action.
    pub action_scopes: Vec<ActionScope>,
    /// Parameters. If absent the action has no parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<Vec<ActionParameterSpec>>,
    /// Free-text description of the result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_result: Option<String>,
    /// Possible blocking types of the action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocking_types: Option<Vec<BlockingType>>,
    /// `true`: the action can be paused via startPause.
    pub pause_allowed: bool,
    /// `true`: the action can be cancelled via cancelOrder.
    pub cancel_allowed: bool,
}

/// Where an action type may be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActionScope {
    /// As an instant action.
    Instant,
    /// On nodes.
    Node,
    /// On edges.
    Edge,
    /// As a zone action.
    Zone,
}

/// Description of a parameter of an action type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionParameterSpec {
    /// Key of the parameter.
    pub key: String,
    /// Data type of the value.
    pub value_data_type: ValueDataType,
    /// Free-text description of the parameter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// `true`: the parameter is optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<bool>,
}

/// Data type of an action parameter value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValueDataType {
    /// Boolean.
    Bool,
    /// Number.
    Number,
    /// Integer.
    Integer,
    /// String.
    String,
    /// Object.
    Object,
    /// Array.
    Array,
}

/// Detailed definition of the robot's geometry. All fields optional.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileRobotGeometry {
    /// Wheels: arrangement and geometry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wheel_definitions: Option<Vec<WheelDefinition>>,
    /// 2D envelope curves.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envelopes2d: Option<Vec<Envelope2d>>,
    /// 3D envelope curves.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envelopes3d: Option<Vec<Envelope3d>>,
}

/// A wheel of the robot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WheelDefinition {
    /// Wheel type. Extensible enum: `DRIVE`, `CASTER`, `FIXED`, `MECANUM`.
    #[serde(rename = "type")]
    pub wheel_type: String,
    /// `true`: the wheel is actively driven.
    pub is_active_driven: bool,
    /// `true`: the wheel is actively steered.
    pub is_active_steered: bool,
    /// Position in the robot coordinate system.
    pub position: WheelPosition,
    /// Nominal diameter in m.
    pub diameter: f64,
    /// Nominal width in m.
    pub width: f64,
    /// Displacement of the wheel's center to the rotation point in m (casters). Default 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub center_displacement: Option<f64>,
    /// Free-text constraints defined by the manufacturer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<String>,
}

/// Position of a wheel in the robot coordinate system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelPosition {
    /// X in m.
    pub x: f64,
    /// Y in m.
    pub y: f64,
    /// Orientation in rad (necessary for fixed wheels).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
}

/// A 2D envelope curve (closed simple polygon).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Envelope2d {
    /// Identifier of the envelope curve set.
    #[serde(rename = "envelope2dId")]
    pub envelope_2d_id: String,
    /// Polygon points.
    pub vertices: Vec<Point2d>,
    /// Free-text description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A 2D point in m.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point2d {
    /// X in m.
    pub x: f64,
    /// Y in m.
    pub y: f64,
}

/// A 3D envelope curve.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Envelope3d {
    /// Identifier of the envelope curve set.
    #[serde(rename = "envelope3dId")]
    pub envelope_3d_id: String,
    /// Format of the data, e.g. `DXF`.
    pub format: String,
    /// The envelope data, in the given format (any JSON object).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Protocol and URL for downloading the envelope data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Free-text description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Abstract specification of load capabilities. All fields optional.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadSpecification {
    /// Valid values for `state.loads[].loadPosition` and the load handling action parameter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_positions: Option<Vec<String>>,
    /// Load sets the robot can handle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_sets: Option<Vec<LoadSet>>,
}

/// A set of loads the robot can handle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadSet {
    /// Unique name of the load set, e.g. `DEFAULT`.
    pub set_name: String,
    /// Type of load, e.g. `EPAL`.
    pub load_type: String,
    /// Load positions this set is valid for. Absent/empty: all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_positions: Option<Vec<String>>,
    /// Bounding box reference, as in `state.loads[]`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounding_box_reference: Option<BoundingBoxReference>,
    /// Dimensions of the load's bounding box.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_dimensions: Option<LoadDimensions>,
    /// Maximum weight in kg.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_weight: Option<f64>,
    /// Minimum allowed handling height in m.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_loadhandling_height: Option<f64>,
    /// Maximum allowed handling height in m.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_loadhandling_height: Option<f64>,
    /// Minimum allowed handling depth in m.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_loadhandling_depth: Option<f64>,
    /// Maximum allowed handling depth in m.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_loadhandling_depth: Option<f64>,
    /// Minimum allowed tilt in rad.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_loadhandling_tilt: Option<f64>,
    /// Maximum allowed tilt in rad.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_loadhandling_tilt: Option<f64>,
    /// Maximum allowed speed with this load, m/s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_speed: Option<f64>,
    /// Maximum allowed acceleration with this load, m/s².
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_acceleration: Option<f64>,
    /// Maximum allowed deceleration with this load, m/s².
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_deceleration: Option<f64>,
    /// Approximate time for picking up the load, s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pick_time: Option<f64>,
    /// Approximate time for dropping the load, s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop_time: Option<f64>,
    /// Free-text description of the load handling set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

pub use crate::common::{BoundingBoxReference, LoadDimensions};

/// Hardware/software versions, network and battery charging parameters. All fields optional.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileRobotConfiguration {
    /// Hardware and software versions running on the robot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<Version>>,
    /// Network connection information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    /// Battery charging parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub battery_charging: Option<BatteryCharging>,
}

/// A hardware or software version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    /// The key of the version.
    pub key: String,
    /// The value of the version.
    pub value: String,
}

/// Network information. Not to be updated while the robot is operating.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    /// DNS servers used by the robot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_servers: Option<Vec<String>>,
    /// NTP servers used by the robot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ntp_servers: Option<Vec<String>>,
    /// IP address assigned to the robot for the MQTT broker.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_ip_address: Option<String>,
    /// Subnet mask.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
    /// Default gateway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_gateway: Option<String>,
}

/// Battery charging parameters, charge levels in percent (0–100).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryCharging {
    /// At or below this level fleet control should only send orders to a charging station.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical_low_charging_level: Option<f64>,
    /// Minimum desired charging level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_desired_charging_level: Option<f64>,
    /// Maximum desired charging level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_desired_charging_level: Option<f64>,
    /// Desired minimum charging time, s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_charging_time: Option<u32>,
}
