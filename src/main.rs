use std::{thread, time::Duration};
use doe::clipboard::set_clipboard;
use doe::keyboard::keyboard::key_press;
use doe::keyboard::keyboard::key_release;
use std::collections::HashMap;
use doe::keyboard::KeyCode;
use std::io;

fn main() {
use rdev::{listen, Event};
use std::sync::{Arc, Mutex};

//hashmap

// Shared array
let shared_array = Arc::new(Mutex::new(Vec::new()));

// map.insert(String::from("key1"), String::from("value1"));
// map.insert(String::from("key2"), String::from("value2"));
// map.insert(String::from("key3"), String::from("value3"));
let array_clone = Arc::clone(&shared_array);


let callback = move |event: Event| {
    // println!("My callback {:?}", event);
    match event.name {
        Some(string) => {
            let mut array = array_clone.lock().unwrap();

            if &string == "\u{8}"  {
                array.pop();
            }
            else if &string == " " || &string == "\r" {
                array.clear();
            }
            else if &string == ";" {
                array.clear();
                array.push(string);
            }
            else if string.chars().all(char::is_alphanumeric) || &string == "." {
                array.push(string);
            }
        },
        None => (),
    }
};

// This will block.
let handle = thread::spawn(move || {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}); 
let shared_map: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
let map1 = Arc::clone(&shared_map);


let handle1 = thread::spawn(move || {
    let mut current_length = 0;
    loop {
        let mut array: std::sync::MutexGuard<'_, Vec<String>> = shared_array.lock().unwrap();
        let map = map1.lock().unwrap();

        if array.len() == current_length {
            continue;
        }
        current_length = array.len();
        
        if  map.contains_key(&((*array).join("")).to_string()) {
            let snip = map.get(&((*array).join("")).to_string());
            match snip {
                Some(color) => set_clipboard(color).unwrap(),
                None => continue,
            }
            let string_to_match: &str = &(*array).join("");
            // set_clipboard(snip).unwrap();

            println!("{}", string_to_match.len());
            for _i in 0..string_to_match.len() {
                key_press(KeyCode::LEFT_ARROW);
                key_release(KeyCode::LEFT_ARROW);
                thread::sleep(Duration::from_millis(10));
            } 

            key_press(KeyCode::SHIFT);
            for _i in 0..string_to_match.len() {
                key_press(KeyCode::RIGHT_ARROW);
                key_release(KeyCode::RIGHT_ARROW);
                thread::sleep(Duration::from_millis(10));
            }
            key_press(KeyCode::INSERT);
            key_release(KeyCode::SHIFT);
            key_release(KeyCode::INSERT);
            array.clear();
    }
        // thread::sleep(Duration::from_secs(3));
    }
});

let map2 = Arc::clone(&shared_map);

let input_thread = thread::spawn(move || {
    
    loop {
        
        let mut input = String::new();
        println!("Please write a text to expand like .mom==yo mama fat and ugly: ");
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                    let mut map: std::sync::MutexGuard<'_, HashMap<String,String>> = map2.lock().unwrap();
                    // Process the input here
                    let input_array: Vec<&str> = input.split("==").collect();
                    map.insert(input_array[0].to_string(), input_array[1].to_string());
                    println!("You entered: {}", input);
                }
                Err(error) => {
                    println!("Error reading input: {}", error);
                }
            }
             thread::sleep(Duration::from_millis(100));

        }
    });


handle.join().unwrap();
handle1.join().unwrap();
input_thread.join().unwrap();

// Now you can access the shared array here

}
