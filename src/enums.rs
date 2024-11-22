/// Parameter IDs that are required when getting or setting parameters on the robot
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ParamIDs {
    HomingStatus = 2800,
    LastError = 320,
}

impl ParamIDs {
    /// Get the value of the parameter ID as an 32-bit integer type
    pub fn value(&self) -> i32 {
        *self as i32
    }

    /// Get the value of the parameter ID as a String type
    pub fn to_string(&self) -> String {
        self.value().to_string()
    }
}
