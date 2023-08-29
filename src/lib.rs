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