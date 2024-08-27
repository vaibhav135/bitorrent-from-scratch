pub fn decode_integer(str_integer: String) -> Result<i64, &'static str> {
    let numeric_str = str_integer.trim_matches(|pat| pat == 'i' || pat == 'e');
    let size_of_numeric_str = numeric_str.len();

    if numeric_str == "-0"
        || numeric_str == "-"
        || (size_of_numeric_str > 1 && numeric_str.get(0..1).unwrap() == "0")
    {
        return Err("Invalid integer...");
    }

    numeric_str.parse().map_err(|_| "Invalid integer")
}
