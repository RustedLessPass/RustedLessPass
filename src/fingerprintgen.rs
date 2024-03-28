use lesspass::get_fingerprint;

// Define a constant array of icon names
const ICONS: [&str; 46] = [
    "fa-hashtag",
    "fa-heart",
    "fa-hotel",
    "fa-university",
    "fa-plug",
    "fa-ambulance",
    "fa-bus",
    "fa-car",
    "fa-plane",
    "fa-rocket",
    "fa-ship",
    "fa-subway",
    "fa-truck",
    "fa-jpy",
    "fa-eur",
    "fa-btc",
    "fa-usd",
    "fa-gbp",
    "fa-archive",
    "fa-area-chart",
    "fa-bed",
    "fa-beer",
    "fa-bell",
    "fa-binoculars",
    "fa-birthday-cake",
    "fa-bomb",
    "fa-briefcase",
    "fa-bug",
    "fa-camera",
    "fa-cart-plus",
    "fa-certificate",
    "fa-coffee",
    "fa-cloud",
    "fa-coffee",
    "fa-comment",
    "fa-cube",
    "fa-cutlery",
    "fa-database",
    "fa-diamond",
    "fa-exclamation-circle",
    "fa-eye",
    "fa-flag",
    "fa-flask",
    "fa-futbol-o",
    "fa-gamepad",
    "fa-graduation-cap",
];

// Function to retrieve an icon based on the provided SHA256 hash
fn get_icon(sha256: &str) -> &'static str {
    // Convert the hexadecimal SHA256 hash to a u32 integer
    let sum = match u32::from_str_radix(sha256, 16) {
        Ok(parsed_value) => parsed_value,
        // Return a default icon if the hash cannot be parsed
        Err(_) => return "default_icon",
    };
    // Calculate the index of the icon based on the hash value
    let index = sum % ICONS.len() as u32;
    // Return the icon name corresponding to the calculated index
    ICONS[index as usize]
}

// Function to calculate and return a fingerprint based on the input string
pub fn fingerprint_calculate(input: &str) -> Vec<String> {
    let mut hashed_input_icons: Vec<String> = Vec::new();

    // Set default icons if input is empty
    if input.is_empty() {
        hashed_input_icons.push("fa-heart".to_string());
        hashed_input_icons.push("fa-brands fa-rust".to_string());
        hashed_input_icons.push("fa-rocket".to_string());
    } else {
        // Calculate the SHA256 fingerprint of the input string
        let hashed_input: String = get_fingerprint(input)
            // Convert the byte array to a hexadecimal string
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();

        // Divide the hashed input into segments and assign icons
        let mut x = 0;
        let mut y = 6;
        for _i in 0..3 {
            // Get an icon corresponding to each segment of the hashed input
            let hashed_segment_icon = get_icon(&hashed_input[x..y]);
            hashed_input_icons.push(hashed_segment_icon.to_string());
            x += 6;
            y += 6;
        }
    }
    // Return the vector of hashed input icons
    hashed_input_icons
}
