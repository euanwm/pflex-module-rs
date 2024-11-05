use crate::pflex::PFlexRobot;

#[derive(Debug, Clone, PartialEq)]
pub struct Waypoint {
    pub id: i32,
    pub x_mm: f64,
    pub y_mm: f64,
    pub z_mm: f64,
    pub orientation_deg: f64,
    pub rail_position_mm: Option<f64>,
}

impl Waypoint {
    pub fn new(
        id: i32,
        x_mm: f64,
        y_mm: f64,
        z_mm: f64,
        orientation_deg: f64,
        rail_position_mm: Option<f64>,
    ) -> Self {
        Waypoint {
            id,
            x_mm,
            y_mm,
            z_mm,
            orientation_deg,
            rail_position_mm,
        }
    }

    pub fn ee_position(&self) {}
    pub fn to_payload(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.x_mm.to_string(),
            self.y_mm.to_string(),
            self.z_mm.to_string(),
            self.orientation_deg.to_string(),
            PFlexRobot::DEFAULT_EE_PITCH.to_string(),
            PFlexRobot::DEFAULT_EE_ROLL.to_string(),
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MotionProfile {
    pub id: i32,
    pub max_speed_percent: f64,  // Default: 50.0
    pub max_accel_percent: f64,  // Default: 50.0
    pub max_decel_percent: f64,  // Default: 50.0
    pub accel_ramp_seconds: f64, // Default: 0.1
    pub decel_ramp_seconds: f64, // Default: 0.1
    pub in_range: f64,           // Default: 10.0
    pub straight_line: i32,      // Default: 0 (False)
}

impl MotionProfile {
    pub fn default(id: i32) -> Self {
        MotionProfile {
            id,
            max_speed_percent: 50.0,
            max_accel_percent: 50.0,
            max_decel_percent: 50.0,
            accel_ramp_seconds: 0.1,
            decel_ramp_seconds: 0.1,
            in_range: 10.0,
            straight_line: 0,
        }
    }

    pub fn to_payload(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.max_speed_percent.to_string(),
            "0".to_string(),
            self.max_accel_percent.to_string(),
            self.max_decel_percent.to_string(),
            self.accel_ramp_seconds.to_string(),
            self.decel_ramp_seconds.to_string(),
            self.in_range.to_string(),
            self.straight_line.to_string(),
        ]
    }
}

#[derive(Debug)]
pub struct EndEffectorPosition {
    pub yaw_mm: f64,
    pub pitch_mm: f64,
    pub roll_mm: f64,
    pub x_mm: f64,
    pub y_mm: f64,
    pub z_mm: f64,
}

impl EndEffectorPosition {
    pub fn to_payload(&self) -> Vec<String> {
        vec![
            self.x_mm.to_string(),
            self.y_mm.to_string(),
            self.z_mm.to_string(),
            self.yaw_mm.to_string(),
            self.pitch_mm.to_string(),
            self.roll_mm.to_string(),
        ]
    }
}
