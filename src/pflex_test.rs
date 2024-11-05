#[cfg(test)]
mod pflex_test {
    use crate::pflex::PFlexRobot;
    use crate::structs::EndEffectorPosition;
    use std::{env, thread::sleep, time::Duration};

    #[test]
    fn check_vitals() {
        let robot_ip = env!("ROBOT_IP");
        if robot_ip.contains("0.0.0.0") {
            panic!("ROBOT_IP ENV VAR has not been overridden");
        }
        let mut pflex_robot = PFlexRobot::new(robot_ip, true);
        println!("All joint states: {:?}", pflex_robot.get_all_joints());
    }

    #[test]
    fn enable_freedmode() {
        let robot_ip = env!("ROBOT_IP");
        if !robot_ip.contains("0.0.0.0") {
            let mut flex_robot = PFlexRobot::new(robot_ip, true);
            // error -1046
            // power not enabled
            flex_robot.set_power(true);

            // error -1009
            // no robot attached
            flex_robot.attach_robot();

            flex_robot.set_free_mode(true);
            sleep(Duration::from_secs(5));
            flex_robot.set_free_mode(false);
        }
    }

    #[test]
    fn open_close_gripper() {
        env::set_var("ROBOT_IP", "10.1.4.23");
        let robot_ip = env!("ROBOT_IP");
        if robot_ip.contains("0.0.0.0") {
            panic!("Please set ROBOT_IP env variable...")
        }
        let mut pf_robot = PFlexRobot::new(robot_ip, true);
        let closed_width = 0.0;
        let open_width = 60.0;
        pf_robot.move_gripper(open_width, 0);
        sleep(Duration::from_secs(5));
        pf_robot.move_gripper(closed_width, 0);
    }

    #[test]
    fn move_rail() {
        env::set_var("ROBOT_IP", "10.1.4.23");
        let robot_ip = env!("ROBOT_IP");
        if robot_ip.contains("0.0.0.0") {
            panic!("Please set ROBOT_IP env variable")
        }
        let mut pf_robot = PFlexRobot::new(robot_ip, true);
        let go_to_rail_position = 300.00;

        pf_robot.is_robot_attached();
        let home_robot = pf_robot.get_home();
        if home_robot.is_err() {
            print!("Robot error: {}", home_robot.unwrap_err())
        }
        // error -1021
        // robot not homed
        let movement = pf_robot.move_rail(go_to_rail_position);
        if movement.is_err() {
            print!("Robot error: {}", movement.unwrap_err())
        }
    }

    #[test]
    fn get_position() {
        env::set_var("ROBOT_IP", "10.1.4.23");
        let robot_ip = env!("ROBOT_IP");
        if robot_ip.contains("0.0.0.0") {
            panic!("Please set ROBOT_IP env variable")
        }
        let mut pf_robot = PFlexRobot::new(robot_ip, true);
        let current_position = pf_robot.get_location();
        print!("Current position data: {:?}", current_position)
    }

    #[test]
    fn move_to_position() {
        env::set_var("ROBOT_IP", "10.1.4.23");
        let robot_ip = env!("ROBOT_IP");
        if robot_ip.contains("0.0.0.0") {
            panic!("Please set ROBOT_IP env variable")
        }
        let mut pf_robot = PFlexRobot::new(robot_ip, true);
        pf_robot.set_power(true);
        pf_robot.is_robot_attached();
        let homed = pf_robot.get_home();
        if homed.is_err() {
            print!("Robot error: {}", homed.unwrap_err())
        }
        let position = EndEffectorPosition {
            yaw_mm: -94.612,
            pitch_mm: 90.0,
            roll_mm: 180.0,
            x_mm: 403.49,
            y_mm: -364.189,
            z_mm: 815.161,
        };
        let move_cmd = pf_robot.move_to_cartesian(position, 0);
        if move_cmd.is_err() {
            println!("Oops: {}", move_cmd.unwrap_err())
        }
    }
}
