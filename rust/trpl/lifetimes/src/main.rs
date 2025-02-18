use std::str;

fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = crate::longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is: {}", result);

    let first_word = crate::first_word("hello world");
    println!("The first word is: {}", first_word);

    let longest_v2 = crate::longest_with_annotations("hello", "world", "This is an announcement");
    println!("The longest string is: {}", longest_v2);
}

// Just to show that I can return the only one reference with lifetime.
// Implicitly what the compiler does
// Rule 1: Apply lifetimes to all input parameters
// Rule 2: If there is one input parameter, apply its lifetime to the output parameters.
// Rule 3: If there are multiple input parameters, but one of them is self, apply the self lifetime to the output parameter.
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    x
}

// No need of explicit lifetime annotations.
// Here this is called lifetime elision. It is understood by the compiler by applying the 3 rules above.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn longest_with_annotations<'a, T>(
    x: &'a str, 
    y: &'a str, 
    announcement: T) -> &'a str
where 
    T: std::fmt::Display
{
    println!("Announcement: {}", announcement);
    if x > y {
        return x;
    }
    y
}