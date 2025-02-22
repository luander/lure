use lure::regex;

fn main() {
    let re = regex!("[0-9a-f]+");
    assert!(re.is_match("1234deadbeef"));
}
