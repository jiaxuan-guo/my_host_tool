mod parse_args;
mod message;
mod sender;
mod receiver;
use rand::Rng;

fn main() {
    let matches = parse_args::parse_args();
    let args = parse_args::Args::collect_args(matches);
    let query = message::Message::new_query(generate_id(), args);
    println!("query: {:?}", query);

}

pub fn generate_id() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

    // match args.get_class(){
    //     "IN" => (),
    //     "HS" => (),
    //     "CH" => (),
    //     _ => println!("No supported! Use IN/GS/CH.")
    // }
