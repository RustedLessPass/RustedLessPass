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
    "fa-graduation-cap"
];

fn get_icon(sha256: &str) -> &'static str {
    // convert sha256 to number base 16
    // TODO: check if u32 is base 16
    let mut sum = 0;
    for c in sha256.chars() {
        sum += c as u32;
    }
    let index = sum % ICONS.len() as u32;
    return ICONS[index as usize];
}

fn calculate_sha256(input: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let result_str = format!("{:x}", result);
    return result_str;
}

pub fn fingerprint_calculate(input: &str, size: usize, separator: &str) -> Vec<String> {
    // let input = "random_text";
    let hashed_input = calculate_sha256(&input);

    // split the sha256 in 3 segments
    let mut hashed_input_slice = &hashed_input[..]; // create a slice of the string
    let segment_size = hashed_input_slice.len() / size; // determine the size of each segment

    let mut hashed_input_icons: Vec<String> = Vec::new();
    for _i in 0..size {
        let (segment, remaining) = hashed_input_slice.split_at(segment_size);

        let hashed_segment = calculate_sha256(segment);
        let hashed_segment_icon = get_icon(&hashed_segment);
        hashed_input_icons.push(hashed_segment_icon.to_string());

        hashed_input_slice = remaining;
    }

    return hashed_input_icons;
}