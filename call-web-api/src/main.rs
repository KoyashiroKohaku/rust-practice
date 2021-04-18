use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    id: u32,
    login: String,
    url: String,
    name: String,
    bio: String,
    created_at: String,
    updated_at: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let request_url = format!(
        "https://api.github.com/{resource}/{owner}",
        resource = "users",
        owner = "koyashiro",
    );

    println!("{}", request_url);

    let client = reqwest::Client::builder()
        .user_agent("call-web-api")
        .build()?;

    let request = client
        .get(request_url)
        .header("user-agent", "call-web-api")
        .build()?;

    let response = client.execute(request).await?;
    let user: User = response.json().await?;
    println!("{:?}", user);
    Ok(())
}
