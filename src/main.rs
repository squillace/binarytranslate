
fn main() {
    println!("Hello, world!");
    let word = "convert to binary";
    let binary_response = String::from_utf8(word_to_binary(word)).unwrap();
    println!("{}", binary_response);
}




pub fn word_to_binary(word: &str) -> Vec<u8> {
    let mut binary_data = Vec::new();

    for ch in word.chars() {
        let char_code = ch as u8;
        let binary_representation = format!("{:08b}", char_code);

        let padded_binary = binary_representation
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8);

        binary_data.extend(padded_binary);
    }

    binary_data
}