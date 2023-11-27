use deku::prelude::*;
use my_host_tool::utils;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Question {
//  RFC 1035 section 4.1.2
//                                   1  1  1  1  1  1
//     0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
//   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//   |                                               |
//   /                     QNAME                     /
//   /                                               /
//   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//   |                     QTYPE                     |
//   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//   |                     QCLASS                    |
//   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    qname_length: u8,
    #[deku(count = "qname_length", endian = "big")]
    qname: Vec<u8>,
    qtype: u16,
    qclass: u16
}


impl Question {
    pub fn new (name: String) -> Self {
        let qname: Vec<u8> = utils::str_to_vec_u8(name);
        Question {
            qname_length: qname.len() as u8,
            qname,
            qtype: 1,
            qclass: 1, //todo: hard code for now
        }
    }

}

#[derive(Debug)]
enum Qtype {
    A = 1, // a host address
    //todo: add more later
}

#[derive(Debug)]
enum Qclass {
    IN = 1, // the Internet
    CH = 3, // the CHAOS class
    HS = 4, // Hesiod [Dyer 87]
    Any = 255 //any class
}

#[test]
fn test_str_to_vec_u8 () {
    let name: String = "www.baidu.com".to_string();
    let qname: Vec<u8> = utils::str_to_vec_u8(name);
    assert_eq!(vec![0x03, 0x77, 0x77, 0x77, 0x5, 0x62, 0x61, 0x69, 0x64, 0x75, 0x03, 0x63, 0x6F, 0x6D,],qname);

    let name2: String = "WWW.baIdu.com".to_string();
    let qname: Vec<u8> = utils::str_to_vec_u8(name2);
    assert_eq!(vec![0x03, 0x77, 0x77, 0x77, 0x5, 0x62, 0x61, 0x69, 0x64, 0x75, 0x03, 0x63, 0x6F, 0x6D,],qname);

}

#[test]
fn test_question_serde() {
    let test_data: &[u8] = [
        0x0e, //QNAME_LENGTH
        0x03, 0x77, 0x77, 0x77, 0x5, 0x62, 0x61, 0x69, 0x64, 0x75, 0x03, 0x63, 0x6F, 0x6D, // QNAME
        0x00, 0x01, // QTYPE
        0x00, 0x01, // QCLASS
    ]
    .as_ref();

    let question: Question = Question::try_from(test_data).unwrap();
    println!("{:#?}", question);

    let qname: Vec<u8> = utils::str_to_vec_u8("www.baidu.com".to_string());
    assert_eq!(
        Question {
            qname_length: qname.len() as u8, //todo: not safe converter!
            qname,
            qtype: 1,
            qclass: 1, //todo: hard code for now
        },
        question
    );

    let question_data: Vec<u8> = question.try_into().unwrap();

    assert_eq!(test_data, question_data);
}
