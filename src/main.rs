use std::{thread, time::Duration};
use doe::clipboard::set_clipboard;
use doe::keyboard::keyboard::key_press;
use doe::keyboard::keyboard::key_release;

fn main() {
use rdev::{listen, Event};
use std::sync::{Arc, Mutex};

//hashmap

// Shared array
let shared_array = Arc::new(Mutex::new(Vec::new()));

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

let handle1 = thread::spawn(move || {
    loop {
        let stringToMatch: &str = ".sm";
        let mut array: std::sync::MutexGuard<'_, Vec<String>> = shared_array.lock().unwrap();
        // println!("ARRAY: {:?}", (*array).join(""));
        if (*array).join("") == stringToMatch {
            set_clipboard("seedhe maut kae binna kia jeevan").unwrap();
            use doe::keyboard::KeyCode;

            println!("{}", stringToMatch.len());
            for i in 0..stringToMatch.len() {
                key_press(KeyCode::LEFT_ARROW);
                key_release(KeyCode::LEFT_ARROW);
                thread::sleep(Duration::from_millis(10));
            } 

            key_press(KeyCode::SHIFT);
            for i in 0..stringToMatch.len() {
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

handle.join().unwrap();
handle1.join().unwrap();

// Now you can access the shared array here

}
