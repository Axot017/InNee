terraform {
  backend "s3" {
    bucket         = "in-nee-state"
    key            = "dev/terraform.tfstate"
    region         = "eu-central-1"
    dynamodb_table = "in-nee-state-lock"
    encrypt        = true
  }

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

module "app" {
  source = "../module/in-nee-module"

  env = "dev"
}
