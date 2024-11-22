use pflex_module_rs::tcs_client::{TCSClient, TCSCommand};

#[test]
fn make_new() {
    let _tcs = TCSClient::new();
}

#[test]
fn fail_connect() {
    let mut tcs = TCSClient::new();
    let result = tcs.connect("0.0.0.0", None);
    assert!(result.is_err());
}

#[test]
#[ignore = "requires robot"]
fn connect_power_disconnect() {
    let mut tcs = TCSClient::new();
    tcs.connect("0.0.0.0", None)
        .expect("Failed to connect to TCS");
    let _res_1 = tcs.send_command(TCSCommand::NoOp, None, true, None);
    let _res_2 = tcs
        .send_command(TCSCommand::Exit, None, false, None)
        .expect("Goodbye!");
}

#[test]
#[ignore = "requires robot"]
fn connect_home_disconnect() {
    let mut tcs = TCSClient::new();
    tcs.connect("10.1.4.23", None)
        .expect("Failed to connect to TCS");
    //tcs.connect("0.0.0.0", None, None).expect("Failed to connect to TCS");
    // check if connected
    tcs.send_command(TCSCommand::Power, Some(vec!["1"]), true, None)
        .expect("Failed to power on robot");
    tcs.send_command(TCSCommand::NoOp, None, true, None)
        .expect("Failed to connect to TCS");
    // power should be either true or false
    // attach selects a number of robot to connect to (make it zero)
    tcs.send_command(TCSCommand::Attach, Some(vec!["0"]), true, None)
        .expect("Failed to attach robot");
    tcs.send_command(TCSCommand::Exit, None, false, None)
        .expect("Goodbye!");
}
