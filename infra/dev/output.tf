output "user_pool_client_secret" {
  value     = module.app.user_pool_client_secret
  sensitive = true
}
