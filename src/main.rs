fn main() {
    use doe::keyboard::listen_keybord;
    use doe::mouse::listen_mouse_position;
    use doe::mouse::listen_mouse_scroll;
    use doe::mouse::listen_mouse_click;
    let t1 = std::thread::spawn(||{
        listen_keybord();
    });
    let t2 = std::thread::spawn(||{
        listen_mouse_position();
    });
    let t3 = std::thread::spawn(||{
        listen_mouse_scroll();
    });
    let t4 = std::thread::spawn(||{
        listen_mouse_click();
    });
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
}
