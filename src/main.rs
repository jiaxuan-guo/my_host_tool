mod parse_args;
mod message;
use rand::Rng;

fn main() {
    let matches = parse_args::parse_args();
    let args = parse_args::Args::collect_args(matches);
    let id: u16 = generate_id();
    println!("args: {:?}", args);
    message::Message::new_query(id, args);

    // match args.get_class(){
    //     "IN" => (),
    //     "HS" => (),
    //     "CH" => (),
    //     _ => println!("No supported! Use IN/GS/CH.")
    // }
}

pub fn generate_id() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
