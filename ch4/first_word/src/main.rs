fn main() {
    let mut s = String::from("hello world");

    let _word = first_word(&s);
    s.clear();

    let s2 = String::from("hello world");
    let _hello = &s2[0..5];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &val) in bytes.iter().enumerate() {
        if val == b' ' {
            return i;
        }
    }
    s.len()
}

#[allow(dead_code)]
fn first_word_version2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &val) in bytes.iter().enumerate() {
        if val == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
