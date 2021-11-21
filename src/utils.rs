pub fn handle_whitespace(msg: String) -> String {
    let mut char_vec: Vec<char> = msg.chars().collect();

    if char_vec[0] == ' ' {
        char_vec.remove(0);
    };

    let last_element = char_vec.len() - 1;
    if char_vec[last_element] == ' ' {
        char_vec.remove(last_element);
    };

    return char_vec.iter().cloned().collect::<String>();
}