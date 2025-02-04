fn main() {

    let str1: String = String::from("Anton");
    println!("{}", str1);

    let mut str2: String = String::new();
    str2 = "Anton".to_string();

    // Expands the memory used by the string. Still consecutive bytes. 
    str2.push_str(" Antonov");
    println!("Str2 = {str2}.");

    let mut s1 = String::from("foo");
    let s2 = "bar";

    // push_str actually takes a str slice and not the ownership of the s2.
    // This is why the following code works.
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");
    println!("{s2}");

    // This value is borrowed by the method signature of + (a.k.a. add()) -> fn add(mut self, other: &str) -> String {
    // This does not compile
    // println!("{s1}");

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    for c in s.chars() {
        print!("{c}");
    }

}
