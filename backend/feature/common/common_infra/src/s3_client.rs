use aws_sdk_s3::Client;

pub async fn get_s3_client() -> &'static Client {
    static CLIENT: tokio::sync::OnceCell<Client> = tokio::sync::OnceCell::const_new();

    CLIENT
        .get_or_init(|| async {
            log::debug!("S3 client is initializing...");

            let config = aws_config::load_from_env().await;
            let client = Client::new(&config);
            log::debug!("S3 client is initialized.");

            client
        })
        .await
}
