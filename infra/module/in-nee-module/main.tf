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
    }
  ]
}
