trait JsonApi {
    type Error;
    async fn fetch(&self, url: &str) -> Result<serde_json::Value, Self::Error>;
}

struct MyApi {
    client: reqwest::Client
}

impl JsonApi for MyApi {
    type Error = reqwest::Error;
    async fn fetch(&self, url: &str) -> Result<serde_json::Value, Self::Error> {
        self.client.get(url)
        .send()
        .await?
        .json()
        .await
    }
}

#[tokio::main]
async fn main() {

    let api = MyApi{
        client: reqwest::Client::new()
    };

    let response = api.fetch("https://wttr.in/?format=j1").await;

    dbg!(response);
}
