mod integer;
mod string;

use integer::decode_integer;
use string::decode_string;

pub fn decode(args: String) {
    let result = if args.starts_with("i") {
        decode_integer(args).map(|n| n.to_string())
    } else {
        decode_string(args).map(|s| format!("{:?}", s))
    };

    match result {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}
