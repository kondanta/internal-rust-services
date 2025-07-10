pub(crate) fn parse_mac_address(mac_address: &str) -> Result<[u8; 6], &'static str> {
    let mut bytes = [0u8; 6];

    let parts: Vec<&str> = mac_address.split(':').collect();
    if parts.len() != 6 {
        return Err("Invalid MAC address format");
    }

    for (index, part) in parts.iter().enumerate() {
        let byte_value = match u8::from_str_radix(part, 16) {
            Ok(value) => value,
            Err(_) => return Err("Invalid hexadecimal character in MAC address"),
        };

        bytes[index] = byte_value;
    }

    Ok(bytes)
}