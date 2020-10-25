use serde::Deserialize;
#[derive(Deserialize)]
struct Commands {
    statusCode: u8,
    _error: Option<String>,
    message: String,
    data: Option<Vec<String>>,
}

pub fn make_request(input: &str, access_token: &str) -> Result<Vec<String>, reqwest::Error> {
    const ENDPOINT: &str = "https://api.idkcli.com/?q=";

    let url = format!("{}{}", ENDPOINT, input);
    let json : Commands = reqwest::blocking::Client::new()
        .get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()?
        .json()?;

    // if let Some(err) = json.error {
    //     return Err(reqwest::Error::new::<String>(reqwest::Error::Kind::Status(json.statusCode), Option(err)));
    // }

    return Ok(json.data.unwrap());
}
