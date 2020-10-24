const ENDPOINT: &str = "https://api.idkcli.com/?q=";

pub fn make_request(input: &str) -> String
{
    let url = format!("{}{}", ENDPOINT, input);
    let body = reqwest::blocking::get(&url).unwrap().text().unwrap();

    return body;
}
