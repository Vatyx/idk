pub fn make_request(input: &str, access_token: &str) -> Result<String, reqwest::Error>
{
    const ENDPOINT: &str = "https://api.idkcli.com/?q=";

    let url = format!("{}{}", ENDPOINT, input);
    return reqwest::blocking::Client::new().get(&url).header("Authorization", format!("Bearer {}", access_token)).send()?.text();
}
