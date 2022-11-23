variable "env" {
  type = string
}

variable "region" {
  type = string
}

variable "app_name" {
  type = string
}

variable "lambda_integrations" {
  type = list(object({
    lambda_invoke_arn = string
    route             = string
    method            = string
  }))
  default = []
}
