#[allow(unused_variables)]
pub fn main() {
    let string_slice: &str = "howdy";
    let string: String = String::from("heyhey");

    // to get a string from a string slice (malleable heap-allocated)
    let string_from_slice: String = string_slice.to_string();
    let string_from_literal: String = "foobar".to_string();

    let string_from_hardcoding: String = String::from("foo");
    let string_from_str_var = String::from(string_slice);

    let str_from_string: &str = &string_slice;

    // combining strings (concatenation) (strings and not string slices)
    let concatenated_string = ["first", "second"].concat();
    let concatenated_string_format_macro = format!("{} {}", "first", "second");

    // concatenting string slices (can just "add") and works only if the mutable string is first
    // and not the string slice
    let string_and_slice = string + string_slice;

    let mut mut_string = String::new();
    // adding to a (mutable) string
    mut_string.push_str(string_slice); // appending a string slice
    mut_string.push_str("foobar"); // appending a string literal
    mut_string.push('c'); // appending a character (single quotes)

    // more concatenation
    let a = String::from("a");
    let b = String::from("b");
    let a_plus = a + &b + &mut_string;

    // substring using bracket notation using [start..end]
    // NOTE: must used care when using bracket notation, as it is still subject to boundary errors
    // and panic
    let str_from_substring: &str = &string_slice[0..3]; // "how"
    let str_from_substring_inc: &str = &string_slice[0..=2]; // "how"
    let how: &str = &string_slice[..3];
    let owdy: &str = &string_slice[1..];

    // obtaining a character by index
    // we can also used the `chars()` function to iterate character by character in a string
    let nth_char = &string_slice.chars().nth(2); // "w"
    match nth_char {
        Some(c) => println!("found a char {}", c),
        None => {}
    }
    // alternative to the above using `if let`
    if let Some(c) = &string_slice.chars().nth(2) {
        println!("found a char {}", c);
    }
}
