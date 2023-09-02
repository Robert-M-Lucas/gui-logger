use std::sync::atomic::{AtomicU16, Ordering};

const TEXT_TYPE: &str = "text";
const PROGRESS_TYPE: &str = "progress";
const DELETE_TYPE: &str = "delete";

static PORT: AtomicU16 = AtomicU16::new(8000);

pub fn set_port(port: u16) {
    PORT.swap(port, Ordering::Relaxed);
}


fn send_request(path: &str, label_type: &str, data: &str) {
    let client = reqwest::blocking::Client::new();
    let result = client.put(format!("http://0.0.0.0:{}/send", PORT.load(Ordering::Relaxed)))
        .body(format!("{{\
\"path\": \"{path}\",\
\"type\": \"{label_type}\",\
\"data\": \"{data}\"\
}}"))
        .send();

    if let Err(e) = result {
        println!("Log failed with error: {}", e);
    }
}

pub fn gui_log_text(path: &str, value: &str) {
    send_request(path, TEXT_TYPE, value);
}

pub fn gui_log_progress(path: &str, value: f64) {
    let value = format!("{:.1}", value * 100.0);
    send_request(path, PROGRESS_TYPE, &value);
}

pub fn gui_delete_label(path: &str) {
    send_request(path, DELETE_TYPE, "");
}