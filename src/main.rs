#![windows_subsystem="windows"]
#[macro_use] extern crate sciter;
use sciter::{ Value, HELEMENT, Element };
use std::thread;
struct EventHandler;
impl EventHandler {
    fn sum(&self, a: i32, b: i32) -> i32 { a + b }
    fn sum_async(&self, x: i32, y: i32, callback: Value) -> () {
        thread::spawn(move || {
            callback
            .call(None, &make_args!(x + y), None);
        });
    }
    fn capitalize(&self, a: Value) -> Value {
        Value::from(a.to_string().as_str().to_uppercase())
    }
}

impl sciter::EventHandler for EventHandler {
    fn document_complete(&mut self, root: HELEMENT, target: HELEMENT) {
        &Element::from(root)
        .call_function("set_title", &make_args!("quick maths!"));
    }
    dispatch_script_call! (
        fn sum(i32, i32);
        fn sum_async(i32, i32, Value);
        fn capitalize(Value);
    );
}
fn main() {
    // allows CTRL+SHIFT+I to connect to inspector.exe
    sciter::set_options(sciter::RuntimeOptions::DebugMode(true));
    let archived = include_bytes!("../target/assets.rc");
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO  as u8 |
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO  as u8 |
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_EVAL     as u8 |
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO  as u8 
    ));
    let mut frame = sciter::Window::new();
    frame.event_handler(EventHandler {});
    frame.archive_handler(archived);
    frame.load_file("this://app/main.htm");
    frame.run_app();
}
