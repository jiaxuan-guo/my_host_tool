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
    qname: String,
    qtype: String,
    qclass: Qclass
}

impl Question {
    pub fn new() -> Self {
        Question {
            qname: (),
            qtype: (),
            qclass: Qclass::IN, //todo: hard code for now
        }
    }
}

enum Qclass {
    IN = 1, // the Internet
    CH = 3, // the CHAOS class
    HS = 4, // Hesiod [Dyer 87]
    Any = 255 //any class
}