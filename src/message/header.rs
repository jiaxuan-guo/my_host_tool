#[derive(Debug)]
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
    id: u16,  // 16 bit identifier
    qr: Qr,   // 1 bit
    opcode: Opcode, // 4 bit
    authoritative_answer: bool, //1 bit
    truncation: bool, // 1 bit
    recursive_desired: bool, // 1bit
    recursive_available: bool, // 1 bit
    reserved0: i32, // 3 bits, must be 0
    resp_code: ResponseCode, // 4 bits
    qd_count: u16, // the number of entries in the question section
    an_count: u16, // the number of resource records in the answer section.
    ns_count: u16,  // the number of name server resource records in the authority records section.
    ar_count: u16, // the number of resource records in the additional records section.
}

impl Header {
    pub fn new(id: u16) -> Self{
        Header {
            id,
            qr: Qr::Query,
            opcode: Opcode::Query, //todo: hard code for now
            authoritative_answer: false, // false since it is a query
            truncation: false,
            recursive_desired: true, //todo: hard code for now
            recursive_available: false, //doesn't matter since it is a query
            reserved0: 0,
            resp_code: ResponseCode::NoError, //doesn't matter since it is a query
            qd_count: 1, //todo: hard code for now
            an_count: 0, //doesn't matter since it is a query
            ns_count: 0, //doesn't matter since it is a query
            ar_count: 0 //doesn't matter since it is a query
        }
    }
}


// A one bit field that specifies whether this message is a query (0), or a response (1).
#[derive(Debug)]
enum Qr {
    Query = 0,
    Response = 1,
}

// A four bit field that specifies kind of query in this message.  This value is set by the originator of a query and copied into the response.
#[derive(Debug)]
enum Opcode {
    Query = 0,
    Iquery = 1,
    Status = 2,
}

#[derive(Debug)]
enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameErrpr = 3,
    NotImplemented = 4,
    Refused = 5,
}