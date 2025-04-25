pub fn join_from(vec: &Vec<&str>, start_index: usize) -> String {
    if start_index >= vec.len() {
        return String::new(); // Return empty if index is out of bounds
    }
    vec[start_index..].join(" ")
}