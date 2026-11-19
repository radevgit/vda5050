//! The `state` message: the state of the mobile robot.

use serde::{Deserialize, Serialize};

pub use crate::common::{
    BoundingBoxReference, LoadDimensions, ControlPoint, IntermediatePath, MobileRobotPosition, PlannedPath, PolylinePoint, Trajectory,
    Velocity,
};

/// The `state` message (topic `/state`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
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
    /// Maps currently stored on the mobile robot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maps: Option<Vec<MapInfo>>,
    /// Zone sets currently stored on the mobile robot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone_sets: Option<Vec<ZoneSetInfo>>,
    /// Order ID of the current or previous finished order. Empty string if none.
    pub order_id: String,
    /// Order update ID of the accepted order update. 0 if none.
    pub order_update_id: u32,
    /// Node ID of the last reached (or current) node. Empty string if none.
    pub last_node_id: String,
    /// Sequence ID of the last reached node. Only valid if `last_node_id` is not empty.
    pub last_node_sequence_id: u32,
    /// Nodes still to be traversed for the order. Empty if idle.
    pub node_states: Vec<NodeState>,
    /// Edges still to be traversed for the order. Empty if idle.
    pub edge_states: Vec<EdgeState>,
    /// Path within the active order as a NURBS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planned_path: Option<PlannedPath>,
    /// Estimated times of arrival at closer waypoints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intermediate_path: Option<IntermediatePath>,
    /// Current position of the mobile robot on the map.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_robot_position: Option<MobileRobotPosition>,
    /// Velocity in the robot's coordinates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Velocity>,
    /// Loads currently handled by the robot. Absent if the robot cannot determine its load
    /// state; an empty array means unloaded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loads: Option<Vec<Load>>,
    /// `true`: the robot is driving. Other movements (e.g. lift) are not included.
    pub driving: bool,
    /// `true`: the robot is paused (button or instant action) and can resume the order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// `true`: almost at the end of the base; fleet control should send a new base.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_base_request: Option<bool>,
    /// Currently active zone requests. Empty if none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone_requests: Option<Vec<ZoneRequest>>,
    /// Currently active edge requests. Empty if none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edge_requests: Option<Vec<EdgeRequest>>,
    /// Line-guided robots: distance driven past `last_node_id`, in m.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distance_since_last_node: Option<f64>,
    /// Current actions and actions yet to be finished.
    pub action_states: Vec<ActionState>,
    /// All instant action states the robot received.
    pub instant_action_states: Vec<ActionState>,
    /// Zone actions that are running or in an end state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone_action_states: Option<Vec<ActionState>>,
    /// Battery and power information.
    pub power_supply: PowerSupply,
    /// Current operating mode.
    pub operating_mode: OperatingMode,
    /// All active errors. Empty if none.
    pub errors: Vec<StateError>,
    /// Information for visualization or debugging; not for logic in fleet control.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub information: Option<Vec<StateInfo>>,
    /// Safety-related information.
    pub safety_state: SafetyState,
}

/// A map stored on the mobile robot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapInfo {
    /// ID of the map describing a defined area of the robot's workspace.
    pub map_id: String,
    /// Version of the map.
    pub map_version: String,
    /// User-defined, human-readable name. Not for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub map_descriptor: Option<String>,
    /// Whether this map version is currently used. At most one per `map_id` is enabled.
    pub map_status: EnabledStatus,
}

/// A zone set stored on the mobile robot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneSetInfo {
    /// Unique identifier of the zone set.
    pub zone_set_id: String,
    /// Identifier of the corresponding map.
    pub map_id: String,
    /// Whether this zone set is currently used. At most one per map is enabled.
    pub zone_set_status: EnabledStatus,
}

/// Whether a map or zone set is in use on the robot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnabledStatus {
    /// Currently active / used.
    Enabled,
    /// Not in use; can be enabled or deleted on request.
    Disabled,
}

/// A node still to be traversed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeState {
    /// Node identifier. The same node can appear several times; see `sequence_id`.
    pub node_id: String,
    /// Sequence ID to discern multiple nodes with the same `node_id`.
    pub sequence_id: u32,
    /// User-defined, human-readable name. Not for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_descriptor: Option<String>,
    /// `true`: part of the base. `false`: part of the horizon.
    pub released: bool,
    /// Node position (optional, e.g. for debugging).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_position: Option<NodeStatePosition>,
}

/// Position of a node as reported in the state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatePosition {
    /// X-coordinate in m.
    pub x: f64,
    /// Y-coordinate in m.
    pub y: f64,
    /// Orientation in rad (`-π..=π`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// Map the position refers to.
    pub map_id: String,
}

/// An edge still to be traversed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeState {
    /// Edge identifier. The same edge can appear several times; see `sequence_id`.
    pub edge_id: String,
    /// Sequence ID to differentiate edges with the same `edge_id`.
    pub sequence_id: u32,
    /// User-defined, human-readable name. Not for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edge_descriptor: Option<String>,
    /// `true`: part of the base. `false`: part of the horizon.
    pub released: bool,
    /// Trajectory defined in the layout or sent with the order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<Trajectory>,
}

/// A load handled by the mobile robot. All fields are optional.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Load {
    /// Identification of the load (barcode, RFID). Empty if not yet identified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_id: Option<String>,
    /// Type of load.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_type: Option<String>,
    /// Which load handling/carrying unit holds the load.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_position: Option<String>,
    /// Center of the bottom surface of the load's bounding box, in robot coordinates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounding_box_reference: Option<BoundingBoxReference>,
    /// Dimensions of the load's bounding box.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_dimensions: Option<LoadDimensions>,
    /// Absolute weight of the load in kg (at least 0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

/// A zone request sent by the robot to fleet control.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneRequest {
    /// Identifier unique per robot within all active requests.
    pub request_id: String,
    /// Type of request.
    pub request_type: ZoneRequestType,
    /// Zone the request relates to (unique within the zone set).
    pub zone_id: String,
    /// Zone set containing the zone.
    pub zone_set_id: String,
    /// Status of the request.
    pub request_status: RequestStatus,
    /// Trajectory (for replanning requests).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<Trajectory>,
}

/// Type of zone request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ZoneRequestType {
    /// Request to enter a zone (e.g. `RELEASE`).
    Access,
    /// Request to replan within a `COORDINATED_REPLANNING` zone.
    Replanning,
}

/// An edge request sent by the robot to fleet control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeRequest {
    /// Identifier unique per robot across all active requests.
    pub request_id: String,
    /// Type of request.
    pub request_type: EdgeRequestType,
    /// Edge the request relates to.
    pub edge_id: String,
    /// Sequence ID of the edge within the order.
    pub sequence_id: u32,
    /// Status of the request.
    pub request_status: RequestStatus,
}

/// Type of edge request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EdgeRequestType {
    /// Request to deviate from the predefined trajectory within the corridor.
    Corridor,
}

/// Status of a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestStatus {
    /// Just stated by the robot.
    Requested,
    /// Acknowledged by fleet control, no permission yet.
    Queued,
    /// Granted by fleet control.
    Granted,
    /// Revoked by fleet control.
    Revoked,
    /// The lease time expired.
    Expired,
}

/// State of an action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionState {
    /// Unique action ID.
    pub action_id: String,
    /// Type of the action (informational).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// User-defined, human-readable name. Not for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_descriptor: Option<String>,
    /// Status of the action.
    pub action_status: ActionStatus,
    /// Description of the result, e.g. of an RFID read. Errors go in `errors`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_result: Option<String>,
}

/// Status of an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActionStatus {
    /// Waiting for the trigger (passing the node, entering the edge).
    Waiting,
    /// Triggered, preparatory measures are initiated.
    Initializing,
    /// Running.
    Running,
    /// Paused by instant action or external trigger.
    Paused,
    /// Failed, but can be retried.
    Retriable,
    /// Finished.
    Finished,
    /// Could not be performed.
    Failed,
}

/// Battery-related information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerSupply {
    /// State of charge in percent (0–100). 100 for permanently powered robots.
    pub state_of_charge: f64,
    /// Battery voltage in V.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub battery_voltage: Option<f64>,
    /// Battery current in A.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub battery_current: Option<f64>,
    /// State of health in percent (0–100).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub battery_health: Option<u8>,
    /// `true`: charging in progress.
    pub charging: bool,
    /// Estimated reach with the current state of charge, in m.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<u32>,
}

/// Operating mode of the mobile robot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OperatingMode {
    /// Starting up.
    Startup,
    /// Automatic, controlled by fleet control.
    Automatic,
    /// Semi-automatic.
    Semiautomatic,
    /// Intervened by a person.
    Intervened,
    /// Manual.
    Manual,
    /// Service.
    Service,
    /// Teach-in.
    TeachIn,
}

/// An error reported by the robot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateError {
    /// Error type, extensible enumeration (free text).
    pub error_type: String,
    /// References (nodeId, edgeId, orderId, actionId, …) giving more information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_references: Option<Vec<Reference>>,
    /// Verbose description with details and possible causes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    /// Translations of the error description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_description_translations: Option<Vec<Translation>>,
    /// Hint on how to solve the error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_hint: Option<String>,
    /// Translations of the error hint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_hint_translations: Option<Vec<Translation>>,
    /// Severity.
    pub error_level: ErrorLevel,
}

/// Severity of an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorLevel {
    /// No immediate attention required; the robot can continue and accept new orders.
    Warning,
    /// Immediate attention required; the robot can continue and accept new orders.
    Urgent,
    /// Immediate attention required; cannot continue the active order, can accept a new one.
    Critical,
    /// User intervention required; cannot continue or accept orders.
    Fatal,
}

/// A key/value reference used by errors and information entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    /// Type of reference (e.g. nodeId, edgeId, orderId, actionId).
    pub reference_key: String,
    /// Value belonging to the key.
    pub reference_value: String,
}

/// Translation of a text for a language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translation {
    /// Language according to ISO 639-1.
    pub translation_key: String,
    /// Translated text.
    pub translation_value: String,
}

/// An information entry (for visualization or debugging only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateInfo {
    /// Type/name of the information.
    pub info_type: String,
    /// References (headerId, orderId, actionId, …).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info_references: Option<Vec<Reference>>,
    /// User-defined, human-readable name. Not for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info_descriptor: Option<String>,
    /// `DEBUG` or `INFO` (visualization).
    pub info_level: InfoLevel,
}

/// Level of an information entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InfoLevel {
    /// For visualization.
    Info,
    /// For debugging.
    Debug,
}

/// Safety-related information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyState {
    /// Active emergency stop, if any.
    pub active_emergency_stop: EmergencyStop,
    /// `true`: a protective field is violated.
    pub field_violation: bool,
}

/// Emergency stop type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmergencyStop {
    /// To be acknowledged manually at the robot.
    Manual,
    /// Facility e-stop, to be acknowledged remotely.
    Remote,
    /// No e-stop activated.
    None,
}
