// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// use std::thread;

// const NTHREADS: u32 = 10;

// // This is the `main` thread
// fn main() {
//     // Make a vector to hold the children which are spawned.
//     let mut children = vec![];

//     for i in 0..NTHREADS {
//         // Spin up another thread
//         children.push(thread::spawn(move || {
//             println!("this is thread number {}", i);
//         }));
//     }

//     for child in children {
//         // Wait for the thread to finish. Returns a result.
//         let _ = child.join();
//     }
// }


// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

// fn main() {
//     // Create a path to the desired file
//     let path = Path::new("hello.txt");
//     let display = path.display();

//     // Open the path in read-only mode, returns `io::Result<File>`
//     let mut file = match File::open(&path) {
//         Err(why) => panic!("couldn't open {}: {}", display, why),
//         Ok(file) => file,
//     };

//     // Read the file contents into a string, returns `io::Result<usize>`
//     let mut s = String::new();
//     match file.read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read {}: {}", display, why),
//         Ok(_) => print!("{} contains:\n{}", display, s),
//     }

//     // `file` goes out of scope, and the "hello.txt" file gets closed
// }

// static LOREM_IPSUM: &str =
//     "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
// tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
// quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
// consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
// cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
// proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
// ";

// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

// fn main() {
//     let path = Path::new("lorem_ipsum.txt");
//     let display = path.display();

//     // Open a file in write-only mode, returns `io::Result<File>`
//     let mut file = match File::create(&path) {
//         Err(why) => panic!("couldn't create {}: {}", display, why),
//         Ok(file) => file,
//     };

//     // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
//     match file.write_all(LOREM_IPSUM.as_bytes()) {
//         Err(why) => panic!("couldn't write to {}: {}", display, why),
//         Ok(_) => println!("successfully wrote to {}", display),
//     }
// }

// use std::fs::read_to_string;

// fn read_lines(filename: &str) -> Vec<String> {
//     let mut result = Vec::new();

//     for line in read_to_string(filename).unwrap().lines() {
//         result.push(line.to_string())
//     }

//     result
// }


// use std::convert::TryFrom;
// use std::convert::TryInto;

// #[derive(Debug, PartialEq)]
// struct EvenNumber(i32);

// impl TryFrom<i32> for EvenNumber {
//     type Error = ();

//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         if value % 2 == 0 {
//             Ok(EvenNumber(value))
//         } else {
//             Err(())
//         }
//     }
// }

// fn main() {
//     // TryFrom

//     assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
//     assert_eq!(EvenNumber::try_from(5), Err(()));

//     // TryInto

//     let result: Result<EvenNumber, ()> = 8i32.try_into();
//     assert_eq!(result, Ok(EvenNumber(8)));
//     let result: Result<EvenNumber, ()> = 5i32.try_into();
//     assert_eq!(result, Err(()));
// }



// // Unlike C/C++, there's no restriction on the order of function definitions
// fn main() {
//     // We can use this function here, and define it somewhere later
//     fizzbuzz_to(100);
// }

// // Function that returns a boolean value
// fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
//     // Corner case, early return
//     if rhs == 0 {
//         return false;
//     }

//     // This is an expression, the `return` keyword is not necessary here
//     lhs % rhs == 0
// }

// // Functions that "don't" return a value, actually return the unit type `()`
// fn fizzbuzz(n: u32) -> () {
//     if is_divisible_by(n, 15) {
//         println!("fizzbuzz");
//     } else if is_divisible_by(n, 3) {
//         println!("fizz");
//     } else if is_divisible_by(n, 5) {
//         println!("buzz");
//     } else {
//         println!("{}", n);
//     }
// }

// // When a function returns `()`, the return type can be omitted from the
// // signature
// fn fizzbuzz_to(n: u32) {
//     for n in 1..=n {
//         fizzbuzz(n);
//     }
// }