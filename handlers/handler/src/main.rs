/* Written by Rowan Rasmusson
    This program will look at the MsgType field of a message and either send it
    to the scheduler or directly to the Command Dispatcher
*/

use interfaces::{self, TcpInterface};
use std::sync::mpsc;

fn main() {
    // This stream will be the message as a whole
    let ip: String = "127.0.0.1".to_string();
    let port: u16 = 8080;
    let tcp_interface: TcpInterface = interfaces::TcpInterface::new(ip,port).unwrap();

    let (coms_handler_tx, coms_handler_rx) = mpsc::channel();

    // should this funciton be private?
    interfaces::async_read(tcp_interface.clone(), coms_handler_tx);

    if let Ok(msg) = coms_handler_rx.recv() {
        // let curr_msg = from_bytes(msg);
    } else {
        // read again?
    }

    assign_msg(curr_msg, tcp_interface.clone());


    // Procedure:
    // Take in TCP byte stream as input
    // Build a message struct from the bytes
    // Look at the type and send it accordingly
    // write the byte stream to a TCP socket
}
// In charge of writing the message as a whole to the correct destination
fn assign_msg(message: Message, tcp: TcpInterface, rx: Receiver<Vec<u8>>) {
    match message.MsgType {
        MsgType::ScheduledMsg(s) => {
            println!("Sent to scheduler!");
        },
        _ => {
            println!("Sent to CmdDispatcher!");
        },
    };
}