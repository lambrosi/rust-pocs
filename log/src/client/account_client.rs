pub fn get() -> String {
    debug!("Making GET request to external API");
    request();
    String::from("GET response.")
}

pub fn post() -> String {
    debug!("Making POST request to external API");
    request();
    String::from("POST response.")
}

fn request() {
    debug!("Request took 2s to be performed");
}