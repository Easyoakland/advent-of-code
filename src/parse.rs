use std::fs;

pub fn read_file_static(file_path: &str) -> Result<&'static str, std::io::Error> {
    let input_str = fs::read_to_string(file_path)?;
    Ok(Box::leak(Box::new(input_str)))
}
