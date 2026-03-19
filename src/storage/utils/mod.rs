pub fn prepare_key(ignore_case: bool, k: &str) -> String {
    if ignore_case {
        k.to_uppercase()
    } else {
        k.to_string()
    }
}