use common::ports::BULK_MSG_HANDLER_DISPATCHER_PORT;
/*  Writte by Rowan Rasmusson
    Summer 2024
    This program is meant to take serialized Msg Struct and determine
    whether its msg_body is larger than one packet size (128 bytes).
    It will break it into multiple packets if this condition is true and
    will assign the packets a sequence number at msg_body[0]
 */
use interfaces::*;
use message_structure::*;
use std::sync::mpsc;
const MAX_BODY_SIZE: usize = 123;
fn main() {
    let large_msg: Msg = Msg::new(2, 5, 1, 5, vec![0; 300]);
    let deconst = handle_large_msg(large_msg).unwrap();
    let _ = reconstruct_msg(deconst);
}
// let ip = "127.0.0.1".to_string();
    // let port = BULK_MSG_HANDLER_DISPATCHER_PORT;
    // let tcp_interface = interfaces::TcpInterface::new_server(ip, port).unwrap();

    // let (bulk_reader_tx, bulk_reader_rx) = mpsc::channel();
    // // let (bulk_writer_tx, bulk_writer_rx) = mpsc::channel();

    // interfaces::async_read(tcp_interface.clone(), bulk_reader_tx, 2048);
fn handle_large_msg(large_msg: Msg) -> Result<Vec<Msg>, &'static str> {
    let body_len: usize = large_msg.msg_body.len();
    let mut messages: Vec<Msg> = Vec::new();

    if body_len <= MAX_BODY_SIZE {
        messages.push(large_msg);
    } else {
        let number_of_packets: usize = (body_len + MAX_BODY_SIZE - 1) / MAX_BODY_SIZE;
        let number_of_packets_u8: u8 = number_of_packets as u8;

        // First message with the number of packets
        let first_msg = deconstruct_msg(large_msg.clone(), 0, Some(number_of_packets_u8))?;
        messages.push(first_msg.clone());

        // Subsequent messages with chunks of the body
        println!("Deconstruction Commencing");
        for i in 0..number_of_packets {
            let start: usize = if i == 0 {
                0
            } else {
                i * MAX_BODY_SIZE - i // To account for offset of adding i number of sequence bytes to beginning of each message
            };
            let end: usize = ((i + 1) * MAX_BODY_SIZE).min(body_len);

            let mut msg_part: Msg = large_msg.clone();

            msg_part.msg_body = msg_part.msg_body[start..end].to_vec();
            let chunk_msg: Msg = deconstruct_msg(msg_part, (i + 1) as u8, None)?;
            messages.push(chunk_msg);
        }
    }
    Ok(messages)
}



// return a Msg structure that has a squence number as its first byte in the body
fn deconstruct_msg(mut msg: Msg, sequence_num: u8, total_packets: Option<u8>) -> Result<Msg, &'static str> {
    if let Some(total) = total_packets {
        msg.msg_body = vec![total];
    } else {
        msg.msg_body.insert(0, sequence_num);
    }

    let body: &[u8] = &msg.msg_body[0..MAX_BODY_SIZE.min(msg.msg_body.len())];
    let sized_msg = Msg {
        header: msg.header,
        msg_body: body.to_vec(),
    };

    if sized_msg.msg_body.len() > MAX_BODY_SIZE {
        return Err("Sized Msg body exceeds max size");
    }
    println!("Sequence #{}", sequence_num);
    println!("{:?}", sized_msg);
    Ok(sized_msg)
}



// This is receive large messages from the UHF and be able to put it together to read as one message
fn reconstruct_msg(messages: Vec<Msg>) -> Result<Msg, &'static str> {
    if messages.is_empty() {
        return Err("No messages to reconstruct");
    }

    let first_msg = &messages[0];
    if first_msg.msg_body.is_empty() {
        return Err("First message body empty");
    }

    let total_packets = first_msg.msg_body[0] as usize;
    if total_packets != messages.len() - 1 {
        return Err("Mismatch between number of packets and message count");
    }

    let mut full_body: Vec<u8> = Vec::new();

    for i in 1..messages.len() {
        let expected_seq_num = i as u8;
        let msg = &messages[i];

        if msg.msg_body.is_empty() || msg.msg_body[0] != expected_seq_num {
            return Err("Invalid sequence number or empty message body");
        }
        full_body.extend_from_slice(&msg.msg_body[1..]);
    }
    Ok(Msg {
        header: first_msg.header.clone(),
        msg_body: full_body,
    })
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn large_msg_copying() {
        let large_msg: Msg = Msg::new(2,5,1,5,vec![0; 500]);
        let messages: Vec<Msg> = handle_large_msg(large_msg.clone()).unwrap();
        assert_eq!(messages[1].msg_body[0], 1);
        assert_eq!(messages[2].msg_body[0], 2);
        assert!(messages[0].header.dest_id == messages[1].header.dest_id);
    }

    #[test]
    fn test_msg_vector_len() {
        let large_msg: Msg = Msg::new(2,5,1,5,vec![0; 742]);
        let messages: Vec<Msg> = handle_large_msg(large_msg.clone()).unwrap();
        let number_of_packets: usize = (large_msg.msg_body.len() + MAX_BODY_SIZE - 1) / MAX_BODY_SIZE;
        assert_eq!(messages.len(), number_of_packets + 1);
    }
    #[test]
    fn large_msg_deconstruction_and_reconstruction() {
        let mut msg_body = Vec::new();
        for i in 0..255 {
            msg_body.push(i);
        }
        let large_msg: Msg = Msg::new(2, 5, 1, 5, msg_body);
        let body_len: usize = large_msg.msg_body.len();
        let number_of_packets = (body_len + MAX_BODY_SIZE - 1) / MAX_BODY_SIZE;

        let messages = handle_large_msg(large_msg.clone()).expect("Failed to handle large message");

        assert_eq!(messages.len(), number_of_packets + 1);
        assert_eq!(messages[0].msg_body[0], number_of_packets as u8);

        // Reconstruct the message and verify it matches the original
        let reconstructed_msg = reconstruct_msg(messages).expect("Failed to reconstruct message");
        assert_eq!(reconstructed_msg.msg_body.len(), large_msg.msg_body.len());
        assert_eq!(reconstructed_msg.msg_body, large_msg.msg_body);
    }

    #[test]
    fn empty_msg_reconstruction() {
        let messages: Vec<Msg> = vec![];
        let result = reconstruct_msg(messages);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No messages to reconstruct");
    }

    #[test]
    fn invalid_first_msg_reconstruction() {
        let first_msg = Msg::new(2, 5, 1, 5, vec![]);
        let messages = vec![first_msg];
        let result = reconstruct_msg(messages);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "First message body empty");
    }

    #[test]
    fn mismatch_packet_count_reconstruction() {
        let large_msg: Msg = Msg::new(2, 5, 1, 5, vec![0; 500]);
        let body_len: usize = large_msg.msg_body.len();
        let number_of_packets = (body_len + MAX_BODY_SIZE - 1) / MAX_BODY_SIZE;

        let mut messages = Vec::new();

        let first_msg = deconstruct_msg(large_msg.clone(), 0, Some(number_of_packets as u8)).expect("Failed to deconstruct message");
        messages.push(first_msg);

        for i in 0..number_of_packets - 1 {
            let start = i * MAX_BODY_SIZE;
            let end = ((i + 1) * MAX_BODY_SIZE).min(body_len);
            let mut msg_part = large_msg.clone();
            msg_part.msg_body = msg_part.msg_body[start..end].to_vec();
            let chunk_msg = deconstruct_msg(msg_part, (i + 1) as u8, None).expect("Failed to deconstruct message");
            messages.push(chunk_msg);
        }

        let result = reconstruct_msg(messages);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Mismatch between number of packets and message count");
    }

    #[test]
    fn invalid_sequence_number_reconstruction() {
        let large_msg: Msg = Msg::new(2, 5, 1, 5, vec![0; 500]);
        let body_len: usize = large_msg.msg_body.len();
        let number_of_packets = (body_len + MAX_BODY_SIZE - 1) / MAX_BODY_SIZE;

        let mut messages = Vec::new();

        let first_msg = deconstruct_msg(large_msg.clone(), 0, Some(number_of_packets as u8)).expect("Failed to deconstruct message");
        messages.push(first_msg);

        for i in 0..number_of_packets {
            let start = i * MAX_BODY_SIZE;
            let end = ((i + 1) * MAX_BODY_SIZE).min(body_len);
            let mut msg_part = large_msg.clone();
            msg_part.msg_body = msg_part.msg_body[start..end].to_vec();
            let mut chunk_msg = deconstruct_msg(msg_part, (i + 1) as u8, None).expect("Failed to deconstruct message");
            if i == 1 {
                chunk_msg.msg_body[0] = 255; // Invalid sequence number
            }
            messages.push(chunk_msg);
        }

        let result = reconstruct_msg(messages);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid sequence number or empty message body");
    }

    #[test]
    fn valid_sequence_but_empty_body_reconstruction() {
        let large_msg: Msg = Msg::new(2, 5, 1, 5, vec![0; 500]);
        let body_len: usize = large_msg.msg_body.len();
        let number_of_packets = (body_len + MAX_BODY_SIZE - 1) / MAX_BODY_SIZE;

        let mut messages = Vec::new();

        let first_msg = deconstruct_msg(large_msg.clone(), 0, Some(number_of_packets as u8)).expect("Failed to deconstruct message");
        messages.push(first_msg);

        for i in 0..number_of_packets {
            let start = i * MAX_BODY_SIZE;
            let end = ((i + 1) * MAX_BODY_SIZE).min(body_len);
            let mut msg_part = large_msg.clone();
            msg_part.msg_body = msg_part.msg_body[start..end].to_vec();
            let mut chunk_msg = deconstruct_msg(msg_part, (i + 1) as u8, None).expect("Failed to deconstruct message");
            if i == 2 {
                chunk_msg.msg_body = vec![]; // Empty body for a valid sequence number
            }
            messages.push(chunk_msg);
        }

        let result = reconstruct_msg(messages);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid sequence number or empty message body");
    }
}
