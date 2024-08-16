/*
* About Bittorrent
* - BitTorrent is a peer protocol operates over TCP or uTP
* - It uses bencoding (a type of encoding used in peer to peer) for storing and transmitting loosely unstructured data.
*/

use std::env;

const DECODE: &str = "decode";

fn process_args(mut args: env::Args) {
    while let Some(arg) = args.next() {
        match arg.as_str() {
            DECODE => {
                let str_to_decode = args
                    .next()
                    .unwrap_or(String::from("No string provided to decode"));

                if str_to_decode.chars().nth(0).unwrap() == 'i' {
                    let numeric_str = str_to_decode.trim_matches(|pat| pat == 'i' || pat == 'e');
                    let size_of_numeric_str = numeric_str.len();

                    if numeric_str == "-0"
                        || numeric_str == "-"
                        || (size_of_numeric_str > 1 && numeric_str.get(0..1).unwrap() == "0")
                    {
                        println!("Invalid integer...");
                        return;
                    }

                    println!("{}", numeric_str.parse::<i64>().unwrap());
                } else {
                    let str_iter = str_to_decode.split_once(":").unwrap();
                    let res_str = str_iter.1;
                    println!("{:?}", res_str);
                }
            }
            _ => {
                println!("Invalid String");
            }
        }
    }
}

fn main() {
    let mut args = env::args().into_iter();
    args.next();
    process_args(args)
}
