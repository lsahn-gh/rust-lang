fn main() {
    //_string_slices();    
    _array_slices();
}

// ---------------------------
fn _array_slices() {
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];
}

// ---------------------------
fn _string_slices() {
    let s = String::from("hello world");
    let s2 = "bye world";

    let word = first_word(&s);
    let word2 = first_word(s2);

    println!("{}, {}", word, word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
