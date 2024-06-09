use reqwest;

async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    let url = "https://jsonplaceholder.typicode.com/todos/1";

    match fetch_data(url).await {
        Ok(data) => println!("Received data: {:?}", data),
        Err(e) => println!("An error occurred: {}", e),
    }
}
