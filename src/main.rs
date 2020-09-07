use reqwest::blocking as sync_reqwest;

use std::io::Write;
use std::fs::File;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let client = sync_reqwest::Client::new();

    let avatar = client.get("https://avatar.tobi.sh/").send()?.bytes()?;
    File::create("avatar.png")?.write_all(&avatar)?;

    let files = sync_reqwest::multipart::Form::new()
        .text("token", std::env::var("SLACK_OAUTH_TOKEN")?)
        .file("image", "avatar.png")?;

    let api_response = client
        .post("https://slack.com/api/users.setPhoto")
        .multipart(files)
        .send()?;

    println!("{:#?}", api_response.text());
    Ok(())
}
