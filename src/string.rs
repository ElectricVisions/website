
pub fn format_or_empty(label: &str, value: &String) -> String {
  if value.is_empty() { return String::new()}

  format!("{label}{value}")
}

