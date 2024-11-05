// yeah i'll add all the precise codes when I can be bothered
// watch this, imma do something heckin lazy instead

use crate::error_codes::ResponseCodes::{RobotPowerNotEnabled, Success, Warning};
use std::fmt;

pub type RobotError = String;
pub type RobotOK = String;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ResponseCodes {
    Success = 0,
    Warning = 1,
    RobotPowerNotEnabled = -1046,
}

impl ResponseCodes {
    pub fn check_code(code: String) -> Result<RobotOK, RobotError> {
        match code.parse().unwrap() {
            0 => Ok(Success.to_string()),
            1 => Ok(Warning.to_string()),
            -1046 => Err(RobotPowerNotEnabled.to_string()),
            _ => Err(code),
        }
    }

    pub fn value(&self) -> i32 {
        *self as i32
    }
}

impl fmt::Display for ResponseCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code_description = match self {
            Success => "Operation completed successfully without an error.",
            Warning => "Operation completed with a warning.",
            _ => "An error occurred during the operation.",
        };
        write!(f, "PFError {}: {}", self.value(), code_description)
    }
}
