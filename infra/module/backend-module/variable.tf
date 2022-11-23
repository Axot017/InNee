variable "region" {
  description = "Default region for provider"
  type        = string
  default     = "eu-central-1"
}

variable "bucket_name" {
  description = "Name of the backend bucket"
  type        = string
}

variable "table_name" {
  description = "Name of the backend lock table"
  type        = string
}
