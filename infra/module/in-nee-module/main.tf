terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

provider "aws" {
  region = var.region
}

module "hello_lambda" {
  source = "../lambda-module"


  region                = var.region
  env                   = var.env
  name                  = "hello"
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/hello.zip"
  gateway_execution_arn = module.gateway.execution_arn
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  ]
}

module "create_apartment_v1_lambda" {
  source = "../lambda-module"


  region                = var.region
  env                   = var.env
  name                  = "create-apartment-v1"
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/create_apartment_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  ]
}

module "gateway" {
  source = "../gateway-module"

  region   = var.region
  env      = var.env
  app_name = "in-nee"
  lambda_integrations = [
    {
      lambda_invoke_arn = module.hello_lambda.invoke_arn
      route             = "/hello"
      method            = "GET"
    },
    {
      lambda_invoke_arn = module.create_apartment_v1_lambda.invoke_arn
      route             = "/v1/apartment"
      method            = "POST"
    }
  ]
}

resource "aws_dynamodb_table" "basic-dynamodb-table" {
  name           = "in-nee-${var.env}"
  billing_mode   = var.dynamodb_billing_mode
  read_capacity  = var.dynamodb_read_capacity
  write_capacity = var.dynamodb_write_capacity
  hash_key       = "HK"
  range_key      = "RK"

  attribute {
    name = "HK"
    type = "S"
  }

  attribute {
    name = "RK"
    type = "S"
  }

  attribute {
    name = "GSI_HK"
    type = "S"
  }

  attribute {
    name = "GSI_RK"
    type = "S"
  }

  ttl {
    attribute_name = "TTL"
    enabled        = true
  }

  global_secondary_index {
    name            = "GSI"
    hash_key        = "GSI_HK"
    range_key       = "GSI_RK"
    projection_type = "ALL"
    read_capacity   = var.dynamodb_gsk_read_capacity
    write_capacity  = var.dynamodb_gsk_write_capacity
  }

  tags = {
    Service     = "in-nee"
    Environment = var.env
  }
}
