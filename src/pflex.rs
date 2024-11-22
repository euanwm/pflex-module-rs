use crate::enums::ParamIDs;
use crate::error_codes::RobotError;
use crate::structs::{EndEffectorPosition, MotionProfile, Waypoint};
use crate::tcs_client::{TCSClient, TCSCommand};
use log::{debug, info};

/// Represents the rail on the robot
/// # Fields
/// * `exists` - A boolean that indicates whether the rail exists
#[derive(Debug)]
pub struct Rail {
    exists: bool,
}

/// Containts the TCS client and the rail struct
/// # Fields
/// * `tcs_client` - A TCSClient instance
/// * `rail` - A Rail instance
pub struct PFlexRobot {
    tcs_client: TCSClient,
    rail: Rail,
}

/// Creates a new PFlexRobot instance and panics if it cannot connect to the robot
/// # Arguments
/// * `ip` - A string slice that holds the IP address of the robot
/// * `has_rail` - A boolean that indicates whether the robot has a rail
impl PFlexRobot {
    const DEFAULT_ROBOT_INDEX: i32 = 1;
    pub const DEFAULT_EE_PITCH: f64 = 90.0; // Unless you plan on hitting your robot with a hammer...
    pub const DEFAULT_EE_ROLL: f64 = -180.0; // ...then these should be constant throughout
    const GRIPPER_JOINT_NUMBER: i32 = 5;

    /// Creates a new PFlexRobot instance
    /// # Arguments
    /// * `ip` - A string slice that holds the IP address of the robot
    /// * `has_rail` - A boolean that indicates whether the robot has a rail
    /// # Returns
    /// * A PFlexRobot instance
    pub fn new(ip: &str, has_rail: bool) -> Self {
        let mut tcs_client = TCSClient::new();
        let is_connected = tcs_client.connect(ip, None);
        if is_connected.is_err() {
            // todo: change this to be a return error
            panic!("Could not connect to TCSClient");
        }

        PFlexRobot {
            tcs_client,
            rail: Rail { exists: has_rail },
        }
    }

    /// Polls the robot with a NoOp call to check the connection status
    /// # Returns
    /// * A boolean that indicates whether the connection is alive
    pub fn is_connection_alive(&mut self) -> bool {
        info!("is_connection_alive called");
        let res = self
            .tcs_client
            .send_command(TCSCommand::NoOp, None, true, None);
        match res {
            Ok(_) => true,
            // it'll throw a timeout error as it'll be waiting for a response that never comes
            Err(_) => false,
        }
    }

    /// Checks if the robot is attached
    /// # Returns
    /// * A boolean that indicates whether the robot is attached
    /// * A RobotError if the robot is not attached
    pub fn is_robot_attached(&mut self) -> Result<bool, RobotError> {
        info!("is_robot_attached called");
        let res = self
            .tcs_client
            .send_command(TCSCommand::Attach, None, true, None);
        match res {
            Ok(data) => {
                if data[0] == "0" {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Checks if the robot is homed
    /// # Returns
    /// * A boolean that indicates whether the robot is homed
    /// * A RobotError if the robot is not homed
    pub fn is_robot_home(&mut self) -> Result<bool, RobotError> {
        info!("is_robot_home called");
        let res = self
            .tcs_client
            .send_command(TCSCommand::Home, None, true, None);
        match res {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    /// Attaches to the default robot
    /// # Returns
    /// * A RobotError if the robot cannot be attached
    /// * Ok if the robot is attached
    pub fn attach_robot(&mut self) -> Result<(), RobotError> {
        info!("attach_robot called");
        let res = self.tcs_client.send_command(
            TCSCommand::Attach,
            Some(vec![&Self::DEFAULT_ROBOT_INDEX.to_string()]),
            true,
            None,
        );
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Selects the default robot
    /// * A RobotError if the robot cannot be selected
    /// * Ok if the robot is selected
    pub fn select_robot(&mut self) -> Result<(), RobotError> {
        info!("select_robot called");
        let res = self.tcs_client.send_command(
            TCSCommand::Select,
            Some(vec![Self::DEFAULT_ROBOT_INDEX.to_string().as_str()]),
            true,
            None,
        );

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn create_waypoint(&mut self, waypoint: Waypoint) -> Result<(), RobotError> {
        // todo: test this
        info!("create_waypoint called");
        let payload = waypoint.to_payload();
        let payload_ref = payload.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        let res = self
            .tcs_client
            .send_command(TCSCommand::LocXyz, Some(payload_ref), true, None);

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn create_station(self) {
        info!("create_station called");
        unimplemented!()
    }

    pub fn create_motion_profile(&mut self, profile: MotionProfile) {
        // todo: test this
        info!("create_motion_profile called");
        let ordered_args = profile.to_payload();
        let referenced_args = ordered_args
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>(); // todo: this is VERY hacky
        let _res =
            self.tcs_client
                .send_command(TCSCommand::Profile, Some(referenced_args), false, None);
    }

    pub fn get_home(&mut self) -> Result<Vec<String>, RobotError> {
        info!("get_home called");
        let res = self.tcs_client.send_command(
            TCSCommand::GetParam,
            Some(vec![
                &ParamIDs::HomingStatus.to_string(),
                Self::DEFAULT_ROBOT_INDEX.to_string().as_str(),
                "0",
                "1",
            ]),
            true,
            None,
        );

        match res {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }

    pub fn get_location(&mut self) -> Result<Vec<String>, RobotError> {
        info!("get_location called");
        let res = self
            .tcs_client
            .send_command(TCSCommand::Loc, None, true, None);
        match res {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }

    pub fn get_endeffector_position(&mut self) -> Result<EndEffectorPosition, RobotError> {
        info!("get_current_position called");
        let res = self
            .tcs_client
            .send_command(TCSCommand::GetLocCart, None, true, None);

        match res {
            Ok(eepos) => {
                if eepos.len() < 6 {
                    panic!("Issue with fetching current position");
                }
                Ok(EndEffectorPosition {
                    x_mm: eepos[0].to_string().parse::<f64>().unwrap(),
                    y_mm: eepos[1].to_string().parse::<f64>().unwrap(),
                    z_mm: eepos[2].to_string().parse::<f64>().unwrap(),
                    yaw_mm: eepos[3].to_string().parse::<f64>().unwrap(),
                    pitch_mm: eepos[4].to_string().parse::<f64>().unwrap(),
                    roll_mm: eepos[5].to_string().parse::<f64>().unwrap(),
                })
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_system_speed(&mut self) -> i32 {
        info!("get_system_speed called");
        // todo: test this
        let res = self
            .tcs_client
            .send_command(TCSCommand::SystemSpeed, None, true, None);
        match res {
            Ok(speed) => speed[0].to_string().parse::<i32>().unwrap(),
            Err(_) => 0,
        }
    }

    pub fn get_all_joints(&mut self) -> Result<Vec<String>, String> {
        info!("get_all_joints called");
        let res = self
            .tcs_client
            .send_command(TCSCommand::GetLocJoints, None, true, None);
        match res {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    pub fn get_previous_error(&mut self) -> Result<Vec<String>, RobotError> {
        // todo: test this
        info!("get_previous_error called");
        let res = self.tcs_client.send_command(
            TCSCommand::GetParam,
            Some(vec![&ParamIDs::LastError.to_string()]),
            true,
            None,
        );
        match res {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }

    pub fn get_motion_state(&mut self) -> Result<Vec<String>, RobotError> {
        info!("get_motion_state called");
        let res = self
            .tcs_client
            .send_command(TCSCommand::MotionState, None, true, None);
        match res {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }

    pub fn set_system_speed(&mut self, speed: i32) {
        // todo: test this
        info!("set_system_speed called");
        // i'll see myself out...
        let _res = self.tcs_client.send_command(
            TCSCommand::SystemSpeed,
            Some(vec![&speed.to_string()]),
            false,
            None,
        );
    }

    pub fn set_payload(&mut self, payload: i32) -> Result<(), RobotError> {
        // todo: test this
        info!("set_payload called");
        let res = self.tcs_client.send_command(
            TCSCommand::Payload,
            Some(vec![payload.to_string().as_str()]),
            true,
            None,
        );
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn set_free_mode(&mut self, free_on: bool) -> Result<(), RobotError> {
        info!("set_free_mode called");
        // todo: you CAN do this by joint but I'm not sure if that's necessary
        let true_vec = vec!["0"];
        let false_vec = vec!["-1"];
        if free_on {
            let res = self.tcs_client.send_command(
                TCSCommand::FreeMode,
                Some(true_vec),
                false, // I have ZERO idea why this request doesn't give you a response back
                None,
            );
            match res {
                Ok(_) => Ok(()),
                Err(code) => Err(code),
            }
        } else {
            let res =
                self.tcs_client
                    .send_command(TCSCommand::FreeMode, Some(false_vec), true, None);
            match res {
                Ok(_) => Ok(()),
                Err(code) => Err(code),
            }
        }
    }

    pub fn set_power(&mut self, power: bool) {
        info!("set_power called");
        match power {
            true => {
                let result =
                    self.tcs_client
                        .send_command(TCSCommand::Power, Some(vec!["1"]), true, None);
                if let Err(e) = result {
                    // todo: add in error return
                    debug!("Error setting power: {}", e);
                }
            }
            false => {
                let result =
                    self.tcs_client
                        .send_command(TCSCommand::Power, Some(vec!["0"]), true, None);
                if let Err(e) = result {
                    // todo: add in error return
                    debug!("Error setting power: {}", e);
                }
            }
        }
    }

    pub fn set_mode(&mut self, verbose_tcs: bool) -> Result<(), RobotError> {
        // todo: test this
        info!("set_mode called");
        let res = self.tcs_client.send_command(
            TCSCommand::Mode,
            Some(vec![{
                if verbose_tcs {
                    "1"
                } else {
                    "0"
                }
            }]),
            true,
            None,
        );
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn move_gripper(&mut self, target_state: f64, motion_profile_id: i32) {
        // todo: test this
        info!("move_gripper called");
        let payload = vec![
            PFlexRobot::GRIPPER_JOINT_NUMBER.to_string(),
            target_state.to_string(),
            motion_profile_id.to_string(),
        ];
        let payload_ref = payload.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        let res =
            self.tcs_client
                .send_command(TCSCommand::MoveOneAxis, Some(payload_ref), true, None);
        if res.is_err() {
            // todo: add in error return
            debug!("Err on gripper: {}", res.unwrap_err());
        }
    }

    pub fn move_rail(&mut self, position: f64) -> Result<(), RobotError> {
        // todo: test this
        info!("move_rail called");
        if !self.rail.exists {
            panic!("No rail exists");
        }
        let res = self.tcs_client.send_command(
            TCSCommand::MoveRail,
            Some(vec!["1", "1", &position.to_string()]),
            true,
            None,
        );
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn move_to_cartesian(
        &mut self,
        ee_position: EndEffectorPosition,
        motion_profile_id: i32,
    ) -> Result<(), RobotError> {
        // todo: test this
        info!("move_to_position called");
        let mut payload = ee_position.to_payload();
        payload.insert(0, format!("{}", motion_profile_id)); // lazy but it works...
        let payload_ref = payload.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        let res =
            self.tcs_client
                .send_command(TCSCommand::MoveToCart, Some(payload_ref), true, None);
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn move_to_waypoint(&mut self, waypoint_id: i32, motion_profile_id: i32) {
        // todo: test this
        info!("move_to_waypoint called");
        let _res = self.tcs_client.send_command(
            TCSCommand::Move,
            Some(vec![
                &waypoint_id.to_string(),
                &motion_profile_id.to_string(),
            ]),
            false,
            None,
        );
    }

    pub fn move_to_joints(&mut self, joint_config: Vec<&str>) -> Result<(), RobotError> {
        // todo: test this
        info!("move_to_joints called");
        let res =
            self.tcs_client
                .send_command(TCSCommand::MoveToJoints, Some(joint_config), false, None);
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn wait_until_static(&mut self, max_timeout_s: f64) -> Result<(), RobotError> {
        info!("wait_until_static called");
        let res =
            self.tcs_client
                .send_command(TCSCommand::WaitForEOM, None, true, Some(max_timeout_s));
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn halt(&mut self) -> Result<(), RobotError> {
        // todo: test this
        info!("halt called");
        let res = self
            .tcs_client
            .send_command(TCSCommand::Halt, None, false, None);
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Cleanly disconnects from the robot
    pub fn disconnect(&mut self) {
        // this is also in the Drop trait because I really couldn't be bothered to remember...
        info!("disconnect called");
        if self.tcs_client.socket.is_none() {
            debug!("Socket is already dead...");
            return;
        }
        let _res = self
            .tcs_client
            .send_command(TCSCommand::Exit, None, false, None);
        let _kill = self.tcs_client.disconnect();
    }
}

impl Drop for PFlexRobot {
    fn drop(&mut self) {
        if self.tcs_client.socket.is_none() {
            return;
        }
        self.disconnect();
    }
}
