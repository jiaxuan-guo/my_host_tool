use crate::message::header::Header;
use crate::message::question::Question;
use crate::message::record::Record;
use crate::parse_args::Args;

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
    header: Header,
    question: Vec<Question>,
    answer: Vec<Record>,
    authority: Vec<Record>,
    additional: Vec<Record>,
}

impl Message {
    pub fn new_query(id: u16, args: Args) -> Self {
        Message{
            header: Header::new(id),
            question: vec![],
            answer: vec![],
            authority: vec![],
            additional: vec![],
        }
    }

    //
    // args: Args { hostname: "baidu.com", class: "", query_type: "", recursive: false, verbose: false,
    // waitforever: false, wait: 5, ipv4: false, ipv6: false, tcp: false, udp: false }
}