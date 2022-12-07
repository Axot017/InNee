variable "region" {
  type = string
}

variable "env" {
  type = string
}

variable "dynamodb_billing_mode" {
  type    = string
  default = "PROVISIONED"
}

variable "dynamodb_read_capacity" {
  type    = number
  default = 1
}

variable "dynamodb_write_capacity" {
  type    = number
  default = 1
}

variable "dynamodb_gsk_read_capacity" {
  type    = number
  default = 1
}

variable "dynamodb_gsk_write_capacity" {
  type    = number
  default = 1
}
