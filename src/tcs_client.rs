use crate::commands::TCSCommand;
use log::{debug, info};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;

use crate::error_codes::{ResponseCodes, RobotError};

#[derive(Debug)]
pub struct TCSClient {
    pub socket: Option<TcpStream>,
}

/// Creates a new TCSClient instance without an active socket
impl TCSClient {
    pub(crate) const DEFAULT_TIMEOUT: f64 = 5.0;
    const REQUEST_SEPARATOR: &'static str = "\n";
    const RESPONSE_SEPARATOR: &'static str = "\r\n";
    const SPACEBAR_SEPERATOR: &'static str = " ";
    const TCS_SERVER_PORT: u16 = 10100;

    pub fn new() -> TCSClient {
        TCSClient { socket: None }
    }

    /// Attempts to connect to the specified robot
    /// # Arguments
    /// * `ip` - IP address of the robot
    /// * `timeout` - Optional timeout setting for all socket read/write attempts
    pub fn connect(&mut self, ip: &str, timeout: Option<f64>) -> Result<(), std::io::Error> {
        let timeout = timeout.unwrap_or(TCSClient::DEFAULT_TIMEOUT);
        let addr = format!("{}:{}", ip, Self::TCS_SERVER_PORT);
        info!("tcs_client::connect called");
        let conn_attempt = TcpStream::connect(addr.clone());
        match conn_attempt {
            Ok(stream) => {
                stream.set_read_timeout(Some(Duration::from_secs_f64(timeout)))?;
                stream.set_write_timeout(Some(Duration::from_secs_f64(timeout)))?;
                self.socket = Some(stream);
                debug!("connected to client");
                Ok(())
            }
            Err(e) => {
                debug!("failed to connect to client");
                Err(e)
            }
        }
    }

    /// Generates and sends the command payload to the robot
    /// # Arguments
    /// * `command` - Selected command to run from the TCSCommand enum
    /// * `command_args` - Optional command arguments
    /// * `wait_for_response` - A boolean option should any commands not require waiting for a response
    /// * `read_timeout` - Optional argument to set the read timeout on the socket
    pub fn send_command<'a>(
        &mut self,
        command: TCSCommand,
        command_args: Option<Vec<&str>>,
        wait_for_response: bool,
        read_timeout: Option<f64>,
    ) -> Result<Vec<String>, RobotError> {
        info!("tcs_client::send_command called");
        if read_timeout.is_some() {
            self.socket
                .as_ref()
                .unwrap()
                .set_read_timeout(Some(Duration::from_secs_f64(read_timeout.unwrap())))
                .expect("Failed to set read timeout");
        }
        let payload: String;

        // build the additional arguments if they exist
        if command_args.is_some() {
            let payload_slice = command_args.unwrap().join(TCSClient::SPACEBAR_SEPERATOR);
            payload = format!(
                "{}{}{}{}",
                command,
                TCSClient::SPACEBAR_SEPERATOR,
                payload_slice,
                TCSClient::REQUEST_SEPARATOR
            );
        // build the command without additional arguments
        } else {
            payload = format!("{}{}", command, TCSClient::REQUEST_SEPARATOR);
        }
        debug!("tcs_client::send_command payload: {}", payload);
        // send the command
        self.socket
            .as_ref()
            .unwrap()
            .write_all(payload.as_bytes())
            .expect("Failed to write message");

        // read the response (if needed)
        if wait_for_response {
            let response = self.get_response();
            match response {
                Ok(r) => {
                    let if_error_code = ResponseCodes::check_code(r[0].to_owned());
                    match if_error_code {
                        Ok(_) => Ok(r[1..].to_vec()),
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e.to_string()),
            }
        } else {
            Ok(vec![])
        }
    }

    fn get_response<'a>(&mut self) -> Result<Vec<String>, std::io::Error> {
        info!("tcs_client::get_response called");
        let read_buffer = &mut [0; 1024];
        let mut response: Vec<u8> = Vec::new();
        // todo: remove unwrap
        let read_attempt = self.socket.as_ref().unwrap().read(read_buffer);
        match read_attempt {
            Ok(bytes_read) => {
                response.extend_from_slice(&read_buffer[..bytes_read]);
            }
            Err(e) => {
                return Err(e);
            }
        }
        debug!("tcs_client::get_response payload: {:#?}", read_attempt);
        // todo: make this nicer
        let response_str = std::str::from_utf8(&response).unwrap();
        let response_parts = response_str
            .split(TCSClient::SPACEBAR_SEPERATOR)
            .collect::<Vec<&str>>();
        let response_parts = response_parts
            .iter()
            .map(|part| part.trim_end_matches(TCSClient::RESPONSE_SEPARATOR))
            .collect::<Vec<&str>>();
        let collated_response = response_parts
            .iter()
            .map(|part| part.to_string())
            .collect::<Vec<String>>();
        debug!(
            "tcs_client::get_response return value: {:#?}",
            collated_response
        );
        Ok(collated_response)
    }

    /// Closes the socket only
    pub fn disconnect(&mut self) -> Result<(), std::io::Error> {
        // this only closes the socket, it doesn't tell the robot that you're disconnecting
        // that'll need to be done by calling the exit command
        info!("tcs_client::disconnect called");
        if self.socket.is_some() {
            self.socket
                .as_ref()
                .unwrap()
                .shutdown(Shutdown::Both)
                .expect("Shutdown failed so I'm failing I guess(?)");
            self.socket = None;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Not connected to a TCS",
            ))
        }
    }
}
