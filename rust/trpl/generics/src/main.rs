fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_int(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!('a', 'b', 'c', 'd', 'e');
    let result = largetst_char(&char_list);
    println!("The largest char is {}", result);

    let f1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = largest(&f1);
    println!("The largest float is {:.2}", result);
}

fn largest_int(list: &[i32]) -> i32 {
    // Need to be mut to be able to update it.
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largetst_char(list: &[char]) -> char {
    // Need to be mut to be able to update it.
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T>(list: &[T]) -> T 
where T: PartialOrd + Copy {
    // Need to be mut to be able to update it.
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}