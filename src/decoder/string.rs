pub fn decode_string(str_to_decode: String) -> Result<String, &'static str> {
    let str_iter = str_to_decode
        .split_once(":")
        .ok_or("Invalid string format...")
        .unwrap();

    let res_str = str_iter.1;
    Ok(res_str.to_owned())
}
