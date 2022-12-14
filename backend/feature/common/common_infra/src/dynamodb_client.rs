use aws_sdk_dynamodb::Client;

pub async fn get_dynamodb_client() -> &'static Client {
    static CLIENT: tokio::sync::OnceCell<Client> = tokio::sync::OnceCell::const_new();

    CLIENT
        .get_or_init(|| async {
            log::info!("DynamoDB is initializing...");

            let config = aws_config::load_from_env().await;
            let client = aws_sdk_dynamodb::Client::new(&config);
            log::info!("DynamoDB is initialized.");

            client
        })
        .await
}
