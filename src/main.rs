/*
* About Bittorrent
* - BitTorrent is a peer protocol operates over TCP or uTP
* - It uses bencoding (a type of encoding used in peer to peer) for storing and transmitting loosely unstructured data.
*/

use std::{
    env::{args, Args},
    iter::Skip,
};

use bittorrent_starter_rust::decoder;

const DECODE: &str = "decode";

fn process_args(mut args: Skip<Args>) {
    match args.next() {
        Some(arg) if arg == DECODE => {
            let str_to_decode = args.next().ok_or("Invalid Value").unwrap();
            decoder::decode(str_to_decode);
        }
        _ => {
            println!("Invalid String");
        }
    }
}

fn main() {
    let args = args().skip(1);
    process_args(args)
}
