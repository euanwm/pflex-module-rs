use std::fmt;

/// Commands enumerator for the robot API
#[derive(Debug, PartialEq, Clone)]
pub enum TCSCommand {
    Mode,
    Exit,
    Power,
    Select,
    Attach,
    Home,
    Halt,
    Loc,
    LocXyz,
    Profile,
    Move,
    MoveToCart,
    MoveToJoints,
    MotionState,
    MoveOneAxis,
    MoveRail,
    GetParam,
    GetLocJoints,
    GetLocCart,
    FreeMode,
    NoOp,
    SystemSpeed,
    Payload,
    WaitForEOM,
}

impl fmt::Display for TCSCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TCSCommand::Mode => "mode",
            TCSCommand::Exit => "exit",
            TCSCommand::Power => "hp",
            TCSCommand::Select => "selectRobot",
            TCSCommand::Attach => "attach",
            TCSCommand::Home => "home",
            TCSCommand::Halt => "halt",
            TCSCommand::Loc => "loc",
            TCSCommand::LocXyz => "locXYZ",
            TCSCommand::Profile => "profile",
            TCSCommand::Move => "move",
            TCSCommand::MoveToCart => "movec",
            TCSCommand::MoveToJoints => "movej",
            TCSCommand::MotionState => "state",
            TCSCommand::MoveOneAxis => "moveoneaxis",
            TCSCommand::MoveRail => "moveRail",
            TCSCommand::GetParam => "pd",
            TCSCommand::GetLocJoints => "wherej",
            TCSCommand::GetLocCart => "wherec",
            TCSCommand::FreeMode => "freemode",
            TCSCommand::NoOp => "nop",
            TCSCommand::SystemSpeed => "mspeed",
            TCSCommand::Payload => "payload",
            TCSCommand::WaitForEOM => "waitForEOM",
        };
        write!(f, "{}", s)
    }
}
