variable "env" {
  type = string
}

variable "app_name" {
  type = string
}

variable "auth_endpoint" {
  type = string
}

variable "auth_client_id" {
  type = string
}

variable "lambda_integrations" {
  type = list(object({
    lambda_invoke_arn = string
    route             = string
    method            = string
    protected         = bool
  }))
  default = []
}
