/* Written by Rowan Rasmusson
    This program will look to see if a Msg has a specific dest_id,opcode combination
    and send it to the scheduler or directly to the Command Dispatcher accordingly.
*/

use interfaces::{self, TcpInterface, TCP_BUFFER_SIZE};
use std::sync::mpsc;
use message_structure::*;
use common::dest_ids::*;

fn main() -> Result<(), &'static str> {
    let tcp_interface = start_handler().unwrap();
    let (coms_handler_reader_tx, coms_handler_reader_rx) = mpsc::channel();
    interfaces::async_read(tcp_interface.clone(), coms_handler_reader_tx, TCP_BUFFER_SIZE);

    let mut curr_msg = Msg::new(0,0,0,0,vec![]);
    if let Ok(buffer) = coms_handler_reader_rx.recv() {
        curr_msg = deserialize_msg(buffer).unwrap();
    } else {
        return Err("Cannot read Message");
    }

    assign_msg(curr_msg, tcp_interface.clone())?;


    // Procedure:
    // Take in TCP byte stream as input
    // Build a message struct from the bytes
    // Look at the opcode and destination combo and send to sched or dispatcher
    // write the byte stream to a TCP socket
    Ok(())
}
// In charge of passing along the message as a whole to the correct destination
fn assign_msg(msg: Msg, tcp_interface: TcpInterface) -> Result<(), &'static str> {
    let msg_opcode = msg.header.op_code;
    let msg_dest = msg.header.dest_id;
    if msg_dest == IRIS && msg_opcode == 0 { //opcde 0 means take image for IRIS
        /* let port = SCHEDULER_DISPATCHER_PORT;
        let tcp_stream = ...
        let serialized_msg = serialize_msg(msg);
        stream.write(serialized_msg);
         */
        Ok(())
    } else {
        /*
        let port = MSG_DISPATCHER_PORT;
        let tcp_stream = ...
        let serialized_msg = serialize_msg(msg);
        stream.write(serialized_msg);
         */
        Ok(())
    }
}

fn start_handler() -> Result<TcpInterface, std::io::Error> {
    // This stream will be the message as a whole
    let ip: String = "127.0.0.1".to_string();
    let port: u16 = 8080;
    interfaces::TcpInterface::new_server(ip,port)
}