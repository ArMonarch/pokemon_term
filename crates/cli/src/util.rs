/// function to append &str to (&mut String) buffer insted of using macro ->  write!(buffer, formatted_string);
pub fn write(string: &mut String, formatted_string: &str) {
    string.push_str(&formatted_string);
}
