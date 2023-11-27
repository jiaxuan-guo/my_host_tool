use std::vec;

use crate::message::header::Header;
use crate::message::question::{Question, self};
use crate::parse_args::Args;
use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Message {
    // RFC 1035, section 4.1
    // +---------------------+
    // |        Header       |
    // +---------------------+
    // |       Question      | the question for the name server
    // +---------------------+
    // |        Answer       | RRs answering the question
    // +---------------------+
    // |      Authority      | RRs pointing toward an authority
    // +---------------------+
    // |      Additional     | RRs holding additional information
    // +---------------------+
    #[deku(count = "12", endian = "big")]
    header: Vec<u8>,
    question_len: u8,
    #[deku(count = "question_len", endian = "big")]
    question: Vec<u8>,
    answer_len: u8,
    #[deku(count = "answer_len", endian = "big")]
    answer: Vec<u8>,
    authority_len: u8,
    #[deku(count = "authority_len", endian = "big")]
    authority: Vec<u8>,
    additional_len: u8,
    #[deku(count = "additional_len", endian = "big")]
    additional: Vec<u8>,
}

impl Message {
    pub fn new_query(id: u16, args: Args) -> Self {
        Message{
            header: Header::new(id).try_into().unwrap(),
            question_len: 1,
            question: Question::new(args.get_hostname().to_string()).try_into().unwrap(),
            answer_len: 0,
            answer: vec![],
            authority_len: 0,
            authority: vec![],
            additional_len: 0,
            additional: vec![],
        }
    }

    //
    // args: Args { hostname: "baidu.com", class: "", query_type: "", recursive: false, verbose: false,
    // waitforever: false, wait: 5, ipv4: false, ipv6: false, tcp: false, udp: false }

}





#[test]
fn test_question_serde() {
    let test_data: &[u8] = [
        0x00, 0x64, //id
        0x01, // |QR|   Opcode  |AA|TC|RD|
        0x00, // |RA|   Z    |   RCODE   |
        0x00, 0x01, // qd_count
        0x00, 0x00, // an_count
        0x00, 0x00, // ns_count
        0x00, 0x00,  // ar_count ...........................header
        0x0e, //QNAME_LENGTH
        0x03, 0x77, 0x77, 0x77, 0x5, 0x62, 0x61, 0x69, 0x64, 0x75, 0x03, 0x63, 0x6F, 0x6D, // QNAME
        0x00, 0x01, // QTYPE
        0x00, 0x01, // QCLASS ..............................question
        0x00,
        0x00,
        0x00
    ]
    .as_ref();

    let message: Message = Message::try_from(test_data).unwrap();
    println!("{:#?}", message);

    let question: Vec<u8> = Question::new(String::from("www.baidu.com")).try_into().unwrap();
    assert_eq!(
        Message {
            header: Header::new(100).try_into().unwrap(),
            question_len: question.len() as u8,
            question,
            answer_len: 0,
            answer: vec![],
            authority_len: 0,
            authority: vec![],
            additional_len: 0,
            additional: vec![],
        },
        message
    );

    let message_data: Vec<u8> = message.try_into().unwrap();

    assert_eq!(test_data, message_data);
}






