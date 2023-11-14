use clap::{Arg, App, ArgMatches};

pub struct Args{
    hostname: String,
    class: String,
    query_type: String,
    recursive: bool,
    verbose: bool,
    waitforever: bool,
    wait: i32,
    ipv4: bool,
    ipv6: bool,
    tcp: bool,
    udp: bool,
}

impl Args {
    //todo: 添加error handling
    pub fn collect_args(matches: ArgMatches) -> Args {
        //retrive arguments
        let hostname = matches.value_of("hostname").unwrap_or("google.com").to_string();
        let class = matches.value_of("class").unwrap_or("").to_string();
        let query_type = matches.value_of("type").unwrap_or("").to_string();
        let recursive = matches.is_present("recursive");
        let verbose = matches.is_present("verbose");
        let waitforever = matches.is_present("waitforever");
        let ipv4 = matches.is_present("ipv4");
        let ipv6 = matches.is_present("ipv6");
        let tcp = matches.is_present("tcp");
        let udp = matches.is_present("udp");
        let wait = matches.value_of("wait").unwrap_or("5").parse().unwrap();
        //create my args
        let args = Args{
            hostname,
            class,
            query_type,
            recursive,
            verbose,
            waitforever,
            wait,
            ipv4,
            ipv6,
            tcp,
            udp,
        };
        args
    }
}

//getter and setter
impl Args {
    pub fn get_hostname(&self) -> &str {
        &self.hostname
    }

    pub fn get_class(&self) -> &str {
        &self.class
    }

    pub fn get_query_type(&self) -> &str {
        &self.query_type
    }

    pub fn get_recursive(&self) -> bool {
        self.recursive
    }

    pub fn get_verbose(&self) -> bool {
        self.verbose
    }

    pub fn get_waitforever(&self) -> bool {
        self.waitforever
    }

    pub fn get_wait(&self) -> i32 {
        self.wait
    }

    pub fn get_ipv4(&self) -> bool {
        self.ipv4
    }

    pub fn get_ipv6(&self) -> bool {
        self.ipv6
    }

    pub fn get_tcp(&self) -> bool {
        self.tcp
    }

    pub fn get_udp(&self) -> bool {
        self.udp
    }
}


pub fn parse_args() -> ArgMatches<'static> {
    App::new("My nslookup tool")
        .author("Guo, Jade <jade.guo@intel.com>")
        .version("0.1.0")
        .about("DNS lookup utility")
        .arg(Arg::with_name("hostname")
                 .takes_value(true)
                 .help("The hostname that you want to look up")
                 .required(true))
        .arg(Arg::with_name("class")
                 .short("c")
                 .long("class")
                 .takes_value(true)
                 .help("This option specifies the query class, which can  be  used  to  lookup HS (Hesiod) or CH (Chaosnet) class resource records. The default class is IN (Internet)"))
        .arg(Arg::with_name("query_type")
                 .short("t")
                 .long("type")
                 .takes_value(true)
                 .help("This option specifies the query type. The type argument can be any recognized query type: CNAME, NS, SOA, TXT, DNSKEY, AXFR, etc."))
        .arg(Arg::with_name("recursive")
                 .short("r")
                 .takes_value(false)
                 .help("This  option  specifies  a non-recursive query"))
        .arg(Arg::with_name("verbose")
                 .short("v")
                 .long("verbose")
                 .takes_value(false)
                 .help("This option sets verbose output"))
        .arg(Arg::with_name("waitforever")
                 .short("w")
                 .takes_value(false)
                 .help("This option sets wait forever"))
        .arg(Arg::with_name("Wait")
                 .short("W")
                 .long("Wait")
                 .takes_value(true)
                 .help("This  options sets the length of the wait timeout, indicating that named should wait for up to wait seconds for a reply. If wait is less than 1, the wait interval is set to 1 second. By default, host waits for 5 seconds for UDP responses and 10 seconds for TCP connections. "))
        .arg(Arg::with_name("IPv4")
                 .short("4")
                 .takes_value(false)
                 .help("This option specifies that only IPv4 should be used for query transport. This is the default way."))
        .arg(Arg::with_name("IPv6")
                 .short("6")
                 .takes_value(false)
                 .help("This  option  specifies that only IPv6 should be used for query transport."))
        .arg(Arg::with_name("TCP")
                 .short("T")
                 .long("TCP")
                 .takes_value(false)
                 .help("This  option  specifies  TCP."))
        .arg(Arg::with_name("UDP")
                 .short("U")
                 .long("UDP")
                 .takes_value(false)
                 .help("This  option  specifies UDP. This is the default way"))
        .get_matches()
}