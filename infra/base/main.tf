terraform {
  backend "s3" {
    bucket         = "in-nee-state"
    key            = "base/terraform.tfstate"
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

module "backend" {
  source = "../module/backend-module"

  table_name  = "in-nee-state-lock"
  bucket_name = "in-nee-state"
}
