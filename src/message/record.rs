use deku::prelude::*;
use my_host_tool::utils;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Record{
//   RFC 1035, section 4.1.3
//                                   1  1  1  1  1  1
//     0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
//   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//   |                                               |
//   /                                               /
//   /                      NAME                     /
//   |                                               |
//   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//   |                      TYPE                     |
//   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//   |                     CLASS                     |
//   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//   |                      TTL                      |
//   |                                               |
//   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//   |                   RDLENGTH                    |
//   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
//   /                     RDATA                     /
//   /                                               /
//   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    name_length: u8,
    #[deku(count = "name_length", endian = "big")]
    name: Vec<u8>,
    // 1    A   a host address
    rr_type: u16,
    // 1    IN  the Internet
    class: u16,
    ttl: i32,
    record_length: u16,
    #[deku(count = "record_length", endian = "big")]
    record_data: Vec<u8>,
}

impl Record {
    pub fn new (name: String) -> Self {
        let name: Vec<u8> = utils::str_to_vec_u8(name);
        Record {
            name_length: name.len() as u8,
            name,
            rr_type: 1,
            class: 1,
            ttl: 0, // prohibit cache
            record_length:0,
            record_data: vec![]
        }
    }

}


#[test]
fn test_question_serde() {
    let test_data: &[u8] = [
        0x0e,//QNAME_LENGTH
        0x03, 0x77, 0x77, 0x77, 0x5, 0x62, 0x61, 0x69, 0x64, 0x75, 0x03, 0x63, 0x6F, 0x6D, // QNAME
        0x00, 0x01, //rr_type
        0x00, 0x01, //class
        0x00, 0x00, //ttl
        0x00, 0x00, //RDLENGTH
    ]
    .as_ref();

    let record: Record = Record::try_from(test_data).unwrap();
    println!("{:#?}", record);

    let name: Vec<u8> = utils::str_to_vec_u8("www.baidu.com".to_string());
    assert_eq!(
        Record::new(String::from("www.baidu.com")),
        record
    );

    let record_data: Vec<u8> = record.try_into().unwrap();

    assert_eq!(test_data, record_data);
}
