use lesspass::get_fingerprint;

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

fn get_icon(sha256: &str) -> &'static str {
    let sum = match u32::from_str_radix(sha256, 16) {
        Ok(parsed_value) => parsed_value,
        Err(_) => return "default_icon",
    };
    let index = sum % ICONS.len() as u32;
    ICONS[index as usize]
}

pub fn fingerprint_calculate(input: &str) -> Vec<String> {
    let hashed_input: String = get_fingerprint(input)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    let mut hashed_input_icons: Vec<String> = Vec::new();
    let mut x = 0;
    let mut y = 6;
    for _i in 0..3 {
        let hashed_segment_icon = get_icon(&hashed_input[x..y]);
        hashed_input_icons.push(hashed_segment_icon.to_string());
        x += 6;
        y += 6;
    }
    hashed_input_icons
}
