use pflex_module_rs::pflex::PFlexRobot;
use pflex_module_rs::structs::EndEffectorPosition;
use std::{env, thread::sleep, time::Duration};

#[test]
#[ignore = "requires robot"]
fn check_vitals() {
    let robot_ip = env!("ROBOT_IP");
    if robot_ip.contains("0.0.0.0") {
        panic!("ROBOT_IP ENV VAR has not been overridden");
    }
    let mut pflex_robot = PFlexRobot::new(robot_ip, true);
    println!("All joint states: {:?}", pflex_robot.get_all_joints());
}

#[test]
#[ignore = "requires robot"]
fn enable_freedmode() {
    let robot_ip = env!("ROBOT_IP");
    if !robot_ip.contains("0.0.0.0") {
        let mut flex_robot = PFlexRobot::new(robot_ip, true);
        // error -1046
        // power not enabled
        flex_robot.set_power(true);

        // error -1009
        // no robot attached
        flex_robot.attach_robot().expect("Robot not attached");

        flex_robot
            .set_free_mode(true)
            .expect("Failed setting free mode");

        sleep(Duration::from_secs(5));

        flex_robot
            .set_free_mode(false)
            .expect("Failed setting free mode")
    }
}

#[test]
#[ignore = "requires robot"]
fn open_close_gripper() {
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
#[ignore = "requires robot"]
fn move_rail() {
    let robot_ip = env!("ROBOT_IP");
    if robot_ip.contains("0.0.0.0") {
        panic!("Please set ROBOT_IP env variable")
    }
    let mut pf_robot = PFlexRobot::new(robot_ip, true);
    let go_to_rail_position = 300.00;

    pf_robot.is_robot_attached().expect("Failed to attaching");
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
#[ignore = "requires robot"]
fn get_position() {
    let robot_ip = env!("ROBOT_IP");
    if robot_ip.contains("0.0.0.0") {
        panic!("Please set ROBOT_IP env variable")
    }
    let mut pf_robot = PFlexRobot::new(robot_ip, true);
    let current_position = pf_robot.get_location();
    print!("Current position data: {:?}", current_position)
}

#[test]
#[ignore = "requires robot"]
fn move_to_position() {
    let robot_ip = env!("ROBOT_IP");
    if robot_ip.contains("0.0.0.0") {
        panic!("Please set ROBOT_IP env variable")
    }
    let mut pf_robot = PFlexRobot::new(robot_ip, true);
    pf_robot.set_power(true);
    let attached = pf_robot.is_robot_attached();
    if attached.is_err() {
        panic!("Robot error: {}", attached.unwrap_err())
    }
    let homed = pf_robot.get_home();
    if homed.is_err() {
        panic!("Robot error: {}", homed.unwrap_err())
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
