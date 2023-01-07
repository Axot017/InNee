variable "env" {
  type = string
}

variable "app_name" {
  type = string
}

variable "name" {
  type = string
}

variable "env_variables" {
  type    = map(string)
  default = null
}

variable "memory_size" {
  type = number
}

variable "zip_path" {
  type = string
}

variable "gateway_execution_arn" {
  type    = string
  default = null
}

variable "s3_arn" {
  type    = string
  default = null
}

variable "arch" {
  type    = string
  default = "arm64"
}

variable "policies" {
  type = list(string)
}
