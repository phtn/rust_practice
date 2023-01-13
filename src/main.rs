#![allow(unused)]
#![allow(dead_code)]
use std::io;
use std::fs::File;
use std::cmp::Ordering;
use std::env;

fn main() {
    store_lines()
}

fn store_lines() {
    let mut items: Vec<u32> = vec![];
    let mut lines = io::stdin().lines();
    let mut input: String = String::from("");

    fn push(elem: &String, a: &mut Vec<u32>) {
        a.push(elem.trim().parse().unwrap())
    }

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        // * stop reading
        if last_input.len() == 0 {
            break;
        }

        // * store input
        push(&last_input, &mut items);
    }

    for item in items.iter() {
        let item: &u32 = item;
        if *item % 2 != 0 {
            println!("{}", item * 2);
        } else {
            println!("{}", item);
        }
    }
    println!("{:?}", items);
}

fn collections() {
    #[derive(Debug)]
    struct Student {
        name: String,
    }

    let mut students = vec![Student {
        name: "Elon".to_string(),
    }];
    students.push(Student { name: "Kimbal".to_string() });

    for student in students.iter() {
        println!("{}", student.name);
    }
    println!("{:?}", students);

    use std::collections::HashMap;

    let mut enrollment = HashMap::new();
    enrollment.insert("physics", students);

    let physics_students = enrollment.get("physics");
}

fn strings() {
    // * String -> Capital S string (owned string)

    // * &str -> string slice (not owned)
    let str_slice: &str = "Tesla";
    fn str_len(s: &str) -> usize {
        println!("string: {s} -> length is {:?}", s.len());
        return s.len();
    }
    str_len(&str_slice);
}

fn ownership() {
    // * Ownership Rules:
    // * -> Each value has 1 owner
    // * -> 1 owner at a time
    // * -> owner out of scope, the value is dropped

    // * Stack -> LIFO
    let x: u8 = 0; // * ->> fixed size

    // * Heap
    let mut v: Vec<u8> = vec![1, 2, 3];
    for i in 0..10 {
        v.push(i);
    }
    println!("{:?}", v);

    for i in v {
        println!("{}", i);
    }

    let y: u8 = 1;
    // here y owns the value 1
    let z: u8 = y;
    // here y no longer owns the value one

    // * Borrowing
    let a: u8 = 2;
    log(a);
    // here a no longer owns the value
    let b: u8 = 4;
    log(b.clone());
    // here b still owns the value
    let c: String = String::from("6");
    log_string(&c);
    // here we passed a reference to the c value 'borrowing'
    // and c remains the owner of the value "6"
    let mut d: Vec<u8> = vec![1, 2, 3];
    push_to_vec(&mut d);
    // here d is now [1,2,3,4] and d is still the owner
    // even after mutation since we only passed a reference

    fn log(var: u8) {
        println!("{}", var)
    }
    fn log_string(str: &String) {
        println!("{}", str)
    }
    fn push_to_vec(v: &mut Vec<u8>) {
        v.push(4)
    }
}

fn error_handling() {
    // For Parsing, File access and Data validation
    enum Result<T, E> {
        Ok(T), // unwraps the Ok variant
        Err(E), // returns the Err variant
    }
    enum Option<T> {
        Some(T),
        None,
    }
    let fruits: Vec<&str> = vec!["banana"];

    let first = fruits.get(0);
    println!("{:?}", first); // Some("banana")
    let second = fruits.get(1);
    println!("{:?}", second); // None

    // File Access
    let f = File::open("example.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("No such file or directory: {:?}", error),
    };
}

fn loops() {
    // * LOOP
    let mut n = 5;
    let run: u32 = loop {
        println!("{n}"); // prints 5 -> 1
        n -= 1;
        if n <= 0 {
            break n;
        }
    };
    assert_eq!(run, 0);

    // * WHILE
    let mut count = 10;
    while count >= 0 {
        println!("{count}"); // prints 10 -> 0
        count -= 1;
    }
    assert_eq!(count, -1);

    // * FOR LOOP
    for item in 0..5 {
        println!("{item}"); // prints 0 -> 4
    }

    let arr: [u32; 5] = [5, 4, 3, 2, 1];
    for index in arr {
        println!("{}", index); // prints out elements not index
    }
}

fn matching() {
    let x = 1;

    // * if, else if, else
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("something else!"),
    }

    let boolean: bool = true;

    let binary: u8 = match boolean {
        false => 0,
        true => 1,
    };
}

fn enums() {
    // * Enums are like algebraic data types

    enum WebEvent {
        KeyPress(char),
        Click {
            x: i64,
            y: i64,
        },
    }
    let quit: WebEvent = WebEvent::KeyPress('q');

    // * <T> is a generic type -> like 'any' in TS
    enum Option<T> {
        Some(T), // asserts a non-null value
        None,
    }
    let something = Some(41);
}

fn structs() {
    // * Structs: Classic | Tuple | Unit

    // * Classic Struct
    struct Car {
        make: String,
        model: String,
        price: u32,
    }
    let model_3 = Car {
        make: String::from("Tesla"),
        model: String::from("3"),
        price: 35_000,
    };
    println!("{}", model_3.make);

    // * Tuple Struct
    struct Point2D(u32, u32);
    let origin: Point2D = Point2D(0, 0);
    // println!("{},{}", origin.0, origin.1);

    // * Destructured
    let Point2D(x, y) = origin;
    println!("{},{}", x, y);

    // * Unit Struct
}

fn find_last_char(string: String) -> char {
    if string.is_empty() {
        return 'e';
    }
    string.chars().next_back().unwrap()
}

fn variables() {
    let t: bool = true;
    let f: bool = false;

    // * Arrays are heterogenous
    let arr: [u32; 3] = [1, 2, 3];
    println!("{}", arr[0]);

    // * Tuples are homogenous
    let tup: (bool, u32, &str) = (false, 1, "Elon Mus");

    println!("{}", tup.2);

    let k: char = find_last_char(String::from("Musk"));
    println!("{}", k);
}