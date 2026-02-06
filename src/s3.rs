#[cfg(feature = "ssr")]
use {
    aws_config::BehaviorVersion,
    aws_sdk_s3::{Client, config::Credentials},
    std::env::var,
};

#[cfg(feature = "ssr")]
pub async fn create_s3_client() -> Client {
    let username = var("S3_USERNAME").expect("S3_USERNAME must be set in .env file");
    let password = var("S3_PASSWORD").expect("S3_PASSWORD must be set in .env file");
    let region = var("S3_REGION").expect("S3_REGION must be set in .env file");
    let endpoint_url = var("S3_ENDPOINT_URL").expect("S3_ENDPOINT_URL must be set in .env file");
    let creds = Credentials::new(username, password, None, None, "static");
    println!("configuring s3 ...");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(creds)
        .region(aws_config::Region::new(region))
        .endpoint_url(endpoint_url)
        .load()
        .await;
    println!("s3 setup complete!");
    Client::new(&config)
}
