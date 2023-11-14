mod parse_args;
use parse_args::*;

fn main() {
    let matches = parse_args();
    let args = Args::collect_args(matches);


    match args.get_class(){
        "IN" => (),
        "HS" => (),
        "CH" => (),
        _ => println!("No supported! Use IN/GS/CH.")
    }
}
