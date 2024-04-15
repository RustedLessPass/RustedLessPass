use lesspass::get_fingerprint;
use std::fmt::Write;
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
    "fa-brands fa-btc",
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
    "fa-futbol",
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
            .fold(String::new(), |mut acc, &byte| {
                write!(acc, "{:02x}", byte).expect("Failed to write to string");
                acc
            });

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_calculate_empty_input() {
        // Arrange
        let input = "";
        let expected_output = vec![
            "fa-heart".to_string(),
            "fa-brands fa-rust".to_string(),
            "fa-rocket".to_string(),
        ];

        // Act
        let result = fingerprint_calculate(input);

        // Assert
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_fingerprint_calculate_non_empty_input_0() {
        // Arrange
        let input = "lorem ipsum";
        // Expected output depends on the actual implementation of get_fingerprint and get_icon functions,
        // You need to provide expected values manually or mock the functions for testing purposes.
        let expected_output = vec![
            "fa-car".to_string(),     // Provide expected icon for the first segment
            "fa-hashtag".to_string(), // Provide expected icon for the second segment
            "fa-bug".to_string(),     // Provide expected icon for the third segment
        ];

        // Act
        let result = fingerprint_calculate(input);

        // Assert
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_fingerprint_calculate_non_empty_input_1() {
        // Arrange
        let input = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Neque sodales ut etiam sit amet nisl purus in mollis.";
        // Expected output depends on the actual implementation of get_fingerprint and get_icon functions,
        // You need to provide expected values manually or mock the functions for testing purposes.
        let expected_output = vec![
            "fa-rocket".to_string(),  // Provide expected icon for the first segment
            "fa-coffee".to_string(),  // Provide expected icon for the second segment
            "fa-cutlery".to_string(), // Provide expected icon for the third segment
        ];

        // Act
        let result = fingerprint_calculate(input);

        // Assert
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_fingerprint_calculate_non_empty_input_2() {
        // Arrange
        let input = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Neque sodales ut etiam sit amet nisl purus in mollis. Eu consequat ac felis donec et odio pellentesque diam volutpat. Mi in nulla posuere sollicitudin. Euismod quis viverra nibh cras. Tristique nulla aliquet enim tortor at auctor urna nunc. Dignissim convallis aenean et tortor at. Turpis egestas pretium aenean pharetra. Sed vulputate odio ut enim. Faucibus et molestie ac feugiat. Donec ultrices tincidunt arcu non sodales neque sodales ut etiam. Donec pretium vulputate sapien nec sagittis aliquam malesuada. Mauris cursus mattis molestie a iaculis. Hendrerit gravida rutrum quisque non. Enim nulla aliquet porttitor lacus luctus accumsan tortor posuere. Et leo duis ut diam quam nulla. Quam lacus suspendisse faucibus interdum posuere lorem. Adipiscing elit ut aliquam purus sit amet. Consectetur adipiscing elit ut aliquam purus sit amet. Erat imperdiet sed euismod nisi porta lorem mollis.";
        // Expected output depends on the actual implementation of get_fingerprint and get_icon functions,
        // You need to provide expected values manually or mock the functions for testing purposes.
        let expected_output = vec![
            "fa-university".to_string(),     // Provide expected icon for the first segment
            "fa-coffee".to_string(), // Provide expected icon for the second segment
            "fa-hotel".to_string(),     // Provide expected icon for the third segment
        ];

        // Act
        let result = fingerprint_calculate(input);

        // Assert
        assert_eq!(result, expected_output);
    }

    // Add more test cases as needed...
}
