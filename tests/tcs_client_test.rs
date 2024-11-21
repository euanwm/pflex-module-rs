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

#[test]
fn loopback_test() {
    // this is done with a mock python script
    let mut tcs = TCSClient::new();
    tcs.connect("0.0.0.0", None)
        .expect("Failed to connect to TCS");
    let res = tcs.send_command(TCSCommand::NoOp, Some(vec!["69", "this"]), true, None);
    match res {
        Ok(response) => {
            println!("Response OK: {:?}", response);
            assert_eq!(response, vec!["nop", "69", "this"]);
        }
        Err(e) => {
            println!("Error in response: {}", e);
            assert!(false);
        }
    }
    // if this one fails, then we have a problem
    let exit = tcs
        .send_command(TCSCommand::Exit, None, false, None)
        .unwrap();
    assert_eq!(exit.len(), 0);
}

#[test]
fn make_it_fail() {
    let mut tcs = TCSClient::new();
    tcs.connect("0.0.0.0", None)
        .expect("Failed to connect to TCS");
    let res = tcs.send_command(TCSCommand::NoOp, Some(vec!["fail_me"]), true, None);
    assert_eq!(res.is_err(), true);
    if res.is_err() {
        println!("Error init brev: {}", res.unwrap_err())
    }
}
