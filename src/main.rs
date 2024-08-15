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

                let mut str_iter = str_to_decode.split(":");
                let res_str = str_iter.nth(1).unwrap_or("Invalid string...");
                println!("{:?}", res_str);
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
