use deku::prelude::*;
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Header {
    //  RFC1035 section 4.1.1 Header section format
    //                                   1  1  1  1  1  1
    //     0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    //   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    //   |                      ID                       |
    //   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    //   |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
    //   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    //   |                    QDCOUNT                    |
    //   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    //   |                    ANCOUNT                    |
    //   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    //   |                    NSCOUNT                    |
    //   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    //   |                    ARCOUNT                    |
    //   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    id: u16,
    #[deku(bits = "1")]
    qr: u8,      // 0 for Query, 1 for Response
    #[deku(bits = "4")]
    opcode: u8, // 0 for Query, 1 for IQuery, 2 for Status
    #[deku(bits = "1")]
    authoritative_answer: bool,
    #[deku(bits = "1")]
    truncation: bool,
    #[deku(bits = "1")]
    recursive_desired: bool,
    #[deku(bits = "1")]
    recursive_available: bool,
    #[deku(bits = "3")]
    reserved0: u32, // reserved, must be 0
    #[deku(bits = "4")]
    resp_code: u8, // 0 for No Error, 1 for Format Error, 2 for Server Failure, 3 for Name Error, 4 for Not Implemented, 5 for Refused
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ar_count: u16,
}

impl Header {
    pub fn new(id: u16) -> Self {
        Header {
            id,
            qr: 0,
            opcode: 0,                   //todo: hard code for now
            authoritative_answer: false, // false since it is a query
            truncation: false,
            recursive_desired: true,    //todo: hard code for now
            recursive_available: false, //doesn't matter since it is a query
            reserved0: 0,
            resp_code: 0, //doesn't matter since it is a query
            qd_count: 1,  //todo: hard code for now
            an_count: 0,  //doesn't matter since it is a query
            ns_count: 0,  //doesn't matter since it is a query
            ar_count: 0,  //doesn't matter since it is a query
        }
    }
}

// A four bit field that specifies kind of query in this message.  This value is set by the originator of a query and copied into the response.
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum Opcode {
    #[deku(id = "0")]
    Query,
    #[deku(id = "1")]
    Iquery,
    #[deku(id = "2")]
    Status,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum ResponseCode {
    #[deku(id = "0")]
    NoError,
    #[deku(id = "1")]
    FormatError,
    #[deku(id = "2")]
    ServerFailure,
    #[deku(id = "3")]
    NameError,
    #[deku(id = "4")]
    NotImplemented,
    #[deku(id = "5")]
    Refused,
}

#[test]
fn test_header() {
    let test_data: &[u8] = [
        0x00, 0x64, //id
        0x01, // |QR|   Opcode  |AA|TC|RD|
        0x00, // |RA|   Z    |   RCODE   |
        0x00, 0x01, // qd_count
        0x00, 0x02, // an_count
        0x00, 0x03, // ns_count
        0x00, 0x04  // ar_count
    ]
    .as_ref();
    // let test_data: Vec<u8> = vec![0; 14];
    let header = Header::try_from(test_data).unwrap();
    println!("{:#?}", header);

    assert_eq!(
        Header {
            id:100,
            qr: 0,
            opcode: 0, //todo: hard code for now
            authoritative_answer: false, // false since it is a query
            truncation: false,
            recursive_desired: true, //todo: hard code for now
            recursive_available: false, //doesn't matter since it is a query
            reserved0: 0,
            resp_code: 0, //doesn't matter since it is a query
            qd_count: 1, //todo: hard code for now
            an_count: 2, //doesn't matter since it is a query
            ns_count: 3, //doesn't matter since it is a query
            ar_count: 4 //doesn't matter since it is a query
        },
        header
    );

    let header_data: Vec<u8> = header.try_into().unwrap();

    assert_eq!(test_data, header_data);
}
