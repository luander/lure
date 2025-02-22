use lure::regex;

fn main() {
    let re = regex!(r"[0-9a-f]+");
    assert!(re.is_match("1234deadbeef"));
}
