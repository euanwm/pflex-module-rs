// This mock server:

// 1. Listens on port 10100 (the same port your TCS client uses)
// 2. Maintains a shared robot state using an Arc<Mutex<>> to allow concurrent client connections
// 3. Implements responses for all the commands defined in your TCSCommand enum
// 4. Simulates basic behavior, like power requirements for movement commands
// 5. Returns formatted responses in the format the client expects
//
// Notable features:
// - Commands like `hp` (power) will modify the internal state
// - Movement commands check if power is enabled and return error -1046 if not
// - Some commands simulate delays (like waitForEOM)
// - Command parsing properly splits by whitespace and trims as needed
// - Responses follow the "code data\r\n" format expected by your client
//
// You can run this server before executing your robot control code to test your
// integration without a physical robot. This is especially useful for unit
// testing and development.

use std::fmt::Debug;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Mock robot state
#[derive(Debug, Clone)]
struct RobotState {
    power: bool,
    attached: bool,
    homed: bool,
    position: [f64; 6], // x, y, z, yaw, pitch, roll
    joint_positions: [f64; 6],
    free_mode: bool,
    system_speed: i32,
    rail_position: Option<f64>,
    motion_state: String,
    selected_robot: Option<usize>,
}

impl RobotState {
    fn new() -> Self {
        RobotState {
            power: true,
            attached: false,
            homed: false,
            position: [300.0, 0.0, 150.0, 0.0, 90.0, -180.0],
            joint_positions: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            free_mode: false,
            system_speed: 50,
            rail_position: Some(0.0),
            motion_state: "Idle".to_string(),
            selected_robot: None,
        }
    }
}

fn handle_client(mut stream: TcpStream, robot_state: Arc<Mutex<RobotState>>) {
    println!("New client connected: {}", stream.peer_addr().unwrap());

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(_) => {
                let response = process_command(&line, robot_state.clone());
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    println!("Failed to send response: {}", e);
                    break;
                } else {
                    println!("Response sent: {:?}", response);
                }
                if line.trim() == "exit" {
                    println!("Client requested exit");
                    break;
                }
            }
            Err(e) => {
                println!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

fn process_command(command: &str, robot_state: Arc<Mutex<RobotState>>) -> String {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    if parts.is_empty() {
        return "0 Invalid command\r\n".to_string();
    }

    let cmd = parts[0];
    println!("Processing command: {:?}", parts);

    match cmd {
        "nop" => "0 \r\n".to_string(),

        "mode" => {
            if parts.len() > 1 {
                // Set verbose mode (not actually implemented in mock)
                "0 \r\n".to_string()
            } else {
                "0 0\r\n".to_string() // Return current mode
            }
        }

        "hp" => {
            let mut state = robot_state.lock().unwrap();
            if parts.len() > 1 {
                state.power = parts[1] == "1";
                "0 \r\n".to_string()
            } else {
                format!("0 {}\r\n", if state.power { "1" } else { "0" })
            }
        }

        "selectRobot" => {
            let mut state = robot_state.lock().unwrap();
            // Always succeed for robot 1
            if parts.len() > 1 {
                if parts[1] == "1" {
                    state.selected_robot = Some(1);
                    "0 \r\n".to_string()
                } else {
                    "-1 Invalid robot index\r\n".to_string()
                }
            } else {
                match state.selected_robot {
                    Some(idx) => format!("0 {}\r\n", idx),
                    None => "-1 No robot selected\r\n".to_string(),
                }
            }
        }

        "attach" => {
            let mut state = robot_state.lock().unwrap();
            if parts.len() > 1 {
                state.attached = true;
                "0 \r\n".to_string()
            } else {
                format!("0 {}\r\n", if state.attached { "1" } else { "0" })
            }
        }

        "home" => {
            let state = robot_state.lock().unwrap();
            if !state.power {
                "-1046 Robot power not enabled\r\n".to_string()
            } else {
                "0 \r\n".to_string()
            }
        }

        "halt" => "0 \r\n".to_string(),

        "loc" => {
            let _state = robot_state.lock().unwrap();
            "0 1 300.0 0.0 150.0 0.0 90.0 -180.0\r\n".to_string()
        }

        "locXYZ" => {
            if parts.len() < 8 {
                "-1 Insufficient parameters\r\n".to_string()
            } else {
                "0 \r\n".to_string()
            }
        }

        "profile" => {
            if parts.len() < 10 {
                "-1 Insufficient parameters\r\n".to_string()
            } else {
                "0 \r\n".to_string()
            }
        }

        "move" => {
            let state = robot_state.lock().unwrap();
            if !state.power {
                "-1046 Robot power not enabled\r\n".to_string()
            } else {
                "0 \r\n".to_string()
            }
        }

        "movec" => {
            let state = robot_state.lock().unwrap();
            if !state.power {
                "-1046 Robot power not enabled\r\n".to_string()
            } else if parts.len() < 7 {
                "-1 Insufficient parameters\r\n".to_string()
            } else {
                "0 \r\n".to_string()
            }
        }

        "movej" => {
            let state = robot_state.lock().unwrap();
            if !state.power {
                "-1046 Robot power not enabled\r\n".to_string()
            } else {
                "0 \r\n".to_string()
            }
        }

        "state" => {
            let state = robot_state.lock().unwrap();
            format!("0 {}\r\n", state.motion_state)
        }

        "moveoneaxis" => {
            let state = robot_state.lock().unwrap();
            if !state.power {
                "-1046 Robot power not enabled\r\n".to_string()
            } else if parts.len() < 4 {
                "-1 Insufficient parameters\r\n".to_string()
            } else {
                "0 \r\n".to_string()
            }
        }

        "moveRail" => {
            let state = robot_state.lock().unwrap();
            if state.rail_position.is_none() {
                "-1 No rail available\r\n".to_string()
            } else if !state.power {
                "-1046 Robot power not enabled\r\n".to_string()
            } else {
                "0 \r\n".to_string()
            }
        }

        "pd" => {
            if parts.len() < 2 {
                "-1 Insufficient parameters\r\n".to_string()
            } else {
                let state = robot_state.lock().unwrap();
                match parts[1] {
                    "2800" => if state.homed { "0 1\r\n" } else { "0 0\r\n" }.to_string(), // HomingStatus
                    "2003" => if state.rail_position.is_some() {
                        "0 111\r\n"
                    } else {
                        "0 15\r\n"
                    }
                    .to_string(), // Axis config
                    "320" => "0 0\r\n".to_string(), // LastError
                    _ => "-1 Unknown parameter\r\n".to_string(),
                }
            }
        }

        "wherej" => {
            let state = robot_state.lock().unwrap();
            format!(
                "0 {} {} {} {} {} {}\r\n",
                state.joint_positions[0],
                state.joint_positions[1],
                state.joint_positions[2],
                state.joint_positions[3],
                state.joint_positions[4],
                state.joint_positions[5]
            )
        }

        "wherec" => {
            let state = robot_state.lock().unwrap();
            format!(
                "0 {} {} {} {} {} {}\r\n",
                state.position[0],
                state.position[1],
                state.position[2],
                state.position[3],
                state.position[4],
                state.position[5]
            )
        }

        "freemode" => {
            let mut state = robot_state.lock().unwrap();
            if parts.len() > 1 {
                if parts[1] == "-1" {
                    state.free_mode = false;
                } else {
                    // state.free_mode = parts[1] == "0"; // 0 enables free mode, -1 disables it
                    state.free_mode = true;
                }
                "0 \r\n".to_string()
            } else {
                "-1 Insufficient parameters\r\n".to_string()
            }
        }

        "mspeed" => {
            let mut state = robot_state.lock().unwrap();
            if parts.len() > 1 {
                if let Ok(speed) = parts[1].parse::<i32>() {
                    state.system_speed = speed;
                }
                "0 \r\n".to_string()
            } else {
                format!("0 {}\r\n", state.system_speed)
            }
        }

        "payload" => {
            let state = robot_state.lock().unwrap();
            if !state.power {
                "-1046 Robot power not enabled\r\n".to_string()
            } else {
                "0 \r\n".to_string()
            }
        }

        "waitForEOM" => {
            // Simulate a short delay before responding
            thread::sleep(Duration::from_millis(500));
            "0 \r\n".to_string()
        }

        "exit" => "0 \r\n".to_string(),

        _ => format!("-1 Unknown command: {}\r\n", cmd),
    }
}

fn robot_loop(robot_state: Arc<Mutex<RobotState>>) {
    let start_time = Instant::now();
    let v = 0.01;
    loop {
        {
            let mut state = robot_state.lock().unwrap();
            if state.free_mode {
                // Update robot position when in free mode
                let delta_time = start_time.elapsed().as_millis() as f64 / 1000.0;
                let mut pos = state.position;
                pos[0] = (pos[0] + delta_time * v) % 2000.;
                pos[1] = (pos[1] + delta_time * v) % 2000.;
                pos[2] = (pos[2] + delta_time * v) % 2000.;
                pos[3] = (pos[3] + delta_time * v) % 360.;
                pos[4] = (pos[4] + delta_time * v) % 360.;
                pos[5] = (pos[5] + delta_time * v) % 360.;
                state.position = pos;
            }
            if !state.power {
                break;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:10100").expect("Failed to bind to address");
    println!("Mock PFlex server running on port 10100");

    let robot_state = Arc::new(Mutex::new(RobotState::new()));
    let robot_state_clone = robot_state.clone();

    thread::spawn(|| {
        robot_loop(robot_state_clone);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let robot_state_clone = robot_state.clone();
                thread::spawn(move || {
                    handle_client(stream, robot_state_clone);
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
