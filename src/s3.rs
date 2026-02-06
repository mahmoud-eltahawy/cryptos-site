#[cfg(feature = "ssr")]
use {
    aws_config::BehaviorVersion,
    aws_sdk_s3::{Client, config::Credentials},
    std::env::var,
};

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct S3 {
    pub username: String,
    pub password: String,
    pub region: String,
    pub bucket: String,
    pub endpoint_url: String,
    pub client: Client,
}

#[cfg(feature = "ssr")]
impl S3 {
    pub async fn get_from_env() -> Self {
        let username = var("S3_USERNAME").expect("S3_USERNAME must be set in .env file");
        let password = var("S3_PASSWORD").expect("S3_PASSWORD must be set in .env file");
        let region = var("S3_REGION").expect("S3_REGION must be set in .env file");
        let bucket = var("S3_BUCKET").expect("S3_BUCKET must be set in .env file");
        let endpoint_url =
            var("S3_ENDPOINT_URL").expect("S3_ENDPOINT_URL must be set in .env file");
        let client = Self::create_client(
            username.clone(),
            password.clone(),
            region.clone(),
            endpoint_url.clone(),
        )
        .await;
        Self {
            username,
            password,
            region,
            bucket,
            endpoint_url,
            client,
        }
    }

    async fn create_client(
        username: String,
        password: String,
        region: String,
        endpoint_url: String,
    ) -> Client {
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
}
