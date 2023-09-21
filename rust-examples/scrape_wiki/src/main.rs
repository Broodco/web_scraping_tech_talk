fn main() {
    /*
    Reqwest is an high-level HTTP client
     */
    let response = reqwest::blocking::get("https://en.wikipedia.org/robots.txt");
    let content = response.unwrap().text().unwrap();
    println!("{content}");
}

