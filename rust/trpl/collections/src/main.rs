fn main() {

    // Vector of int 32
    let mut v: Vec<i32> = Vec::new();
    v.insert(0, 12);
    v.push(2);

    let mut v1 = vec![1, 2, 3];

    // Access via reference (ownership does not move)
    let mut third: &i32 = &v1[2];
    println!("Third = {}.", third);
    third = &v1[1];
    println!("Third is now {third} and the vector[2] is {}.", v1[2]);
    v1[2] = 10;
    println!("Vector[2] is {}.", v1[2]);

    // This panics 
    // println!("Vector[2] is {}.", v1[20]);

    let first: i32 = v1[0];
    println!("First = {first}.");

    let third: Option<&i32> = v1.get(20);
    match third {
        Some(third) => println!("The third element is {third}."),
        None => println!("Outside boundaries"),
    }

    let mut v2 = Vec::new();
    v2.push(11);
    v2.push(12);

    let mut v3 = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // This results in error
    //  v.push(6);
    //     ^^^^^^^^^ mutable borrow occurs here
    v3.push(6);

    println!("The first element is: {first}");

    let row = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Text(String::from("String value"))
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}