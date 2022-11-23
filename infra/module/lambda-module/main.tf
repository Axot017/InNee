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

locals {
  env_variables = var.env_variables == null ? [] : [var.env_variables]
}

resource "aws_iam_role" "lambda_exec" {
  name = "${var.name}-${var.env}-role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
        Effect = "Allow"
        Sid    = ""
      },
    ]
  })
}

resource "aws_lambda_function" "lambda" {
  filename         = var.zip_path
  function_name    = "${var.name}-${var.env}"
  handler          = "not.required"
  source_code_hash = filebase64sha256(var.zip_path)
  runtime          = "provided.al2"
  memory_size      = var.memory_size
  role             = aws_iam_role.lambda_exec.arn
  architectures    = [var.arch]

  dynamic "environment" {
    for_each = local.env_variables

    content {
      variables = each.value
    }
  }

  tags = {
    Environment = var.env
    Name        = var.name
  }
}

resource "aws_lambda_permission" "gateway" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda.function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${var.gateway_execution_arn}/*/*"
}

resource "aws_iam_role_policy_attachment" "lambda_policy" {
  count = length(var.policies)

  role       = aws_iam_role.lambda_exec.name
  policy_arn = var.policies[count.index]
}

resource "aws_cloudwatch_log_group" "hello_world" {
  name = "/aws/lambda/${aws_lambda_function.lambda.function_name}"

  retention_in_days = 30
}
