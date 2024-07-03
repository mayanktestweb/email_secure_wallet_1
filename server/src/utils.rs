

pub fn bytes_to_string(bytes: &Vec<u8>) -> Option<String> {
    if bytes.len() == 0 {return None;}

    let hex : String = bytes.iter()
    .map(|b| format!("{:02x}", b).to_string())
    .collect::<Vec<String>>()
    .join("");

    Some(hex)
}