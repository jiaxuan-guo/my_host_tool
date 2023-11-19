use deku::prelude::*;

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
    num_items: u8,
    #[deku(count = "num_items", endian = "big")]
    qname: Vec<u8>,
    qtype: u16,
    qclass: u16
}


// fn read_from_string(
//     qname: String
// ) -> Result<(), DekuError> {
//     // Access to previously written fields
//     println!("field_a = 0x{:X}", field_a);

//     // value of field_b
//     println!("field_b = 0x{:X}", field_b);

//     // Size of the current field
//     println!("bit_size: {:?}", bit_size);

//     // flip the bits on value if field_a is 0x01
//     let value = if field_a == 0x01 { !field_b } else { field_b };

//     value.write(output, bit_size)
// }

impl Question {
    fn name_to_qname (name: String) -> Vec<u8> {
        let name:String = name.to_lowercase();
        let labels: Vec<String> = name.split(".").map(String::from).collect(); //["www","baidu","com"]
        let mut labels_u8: Vec<Vec<u8>> = labels
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect();
        // prepend length
        for label in &mut labels_u8 {
            label.insert(0, label.len() as u8);
        }
        let qname = labels_u8.concat();
        qname
    }

    pub fn new (name: String) -> Self {
        let qname: Vec<u8> = Self::name_to_qname(name);
        Question {
            num_items: qname.len() as u8,
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
fn test_name_to_qname () {
    let name: String = "www.baidu.com".to_string();
    let qname: Vec<u8> = Question::name_to_qname(name);
    assert_eq!(vec![3, 0x77, 0x77, 0x77, 0x5, 0x62, 0x61, 0x69, 0x64, 0x75, 3, 0x63, 0x6F, 0x6D,],qname);

    let name2: String = "WWW.baIdu.com".to_string();
    let qname: Vec<u8> = Question::name_to_qname(name2);
    assert_eq!(vec![3, 0x77, 0x77, 0x77, 0x5, 0x62, 0x61, 0x69, 0x64, 0x75, 3, 0x63, 0x6F, 0x6D,],qname);

}
// fn test_question_serde() {
//     let test_data: &[u8] = [
//         0x00, 0x64, //id
//         0x01, // |QR|   Opcode  |AA|TC|RD|
//         0x00, // |RA|   Z    |   RCODE   |
//         0x00, 0x01, // qd_count
//         0x00, 0x02, // an_count
//         0x00, 0x03, // ns_count
//         0x00, 0x04  // ar_count
//     ]
//     .as_ref();

//     let question: Question = Question::try_from(test_data).unwrap();
//     println!("{:#?}", question);

//     assert_eq!(
//         Question {
//             qname: "www.bing.com".to_string(),
//             qtype: 1,
//             qclass: 1, //todo: hard code for now
//         },
//         question
//     );

//     let question_data: Vec<u8> = question.try_into().unwrap();

//     assert_eq!(test_data, question_data);
// }