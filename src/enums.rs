#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ParamIDs {
    HomingStatus = 2800,
    LastError = 320,
}

impl ParamIDs {
    pub fn value(&self) -> i32 {
        *self as i32
    }

    pub fn to_string(&self) -> String {
        self.value().to_string()
    }
}
