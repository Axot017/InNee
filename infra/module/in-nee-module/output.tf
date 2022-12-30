output "user_pool_client_secret" {
  value     = module.auth.user_pool_client_secret
  sensitive = true
}
