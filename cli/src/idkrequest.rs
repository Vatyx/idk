
pub fn make_request(input: &str) -> Result<String, reqwest::Error>
{
    const ENDPOINT: &str = "https://api.idkcli.com/?q=";

    let url = format!("{}{}", ENDPOINT, input);
    return reqwest::blocking::get(&url)?.text();
}
