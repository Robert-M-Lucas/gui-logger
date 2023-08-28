use std::thread::sleep;
use std::time::Duration;

pub fn gui_log_text(path: String, value: String) {
    let client = reqwest::blocking::Client::new();
    let result = client.put("http://localhost:8000/send")
        .body(format!("{{\
	\"path\": \"{path}\",\
	\"type\": \"text\",\
	\"data\": \"{value}\"\
}}"))
        .send();

    if let Err(e) = result {
        println!("Log failed with error: {}", e);
    }
}

fn main() {
    let mut counter = 0;
    loop {
        gui_log_text("Label/From/Code/LabelName".to_string(), format!("{}", counter));
        counter += 1;
        sleep(Duration::from_millis(50));
    }
}