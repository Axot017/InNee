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

resource "aws_apigatewayv2_api" "default" {
  name          = "${var.app_name}-${var.env}"
  protocol_type = "HTTP"

  tags = {
    Service     = var.app_name
    Environment = var.env
  }
}

resource "aws_apigatewayv2_stage" "default" {
  # deployment_id = aws_apigatewayv2_deployment.default.id
  api_id      = aws_apigatewayv2_api.default.id
  name        = "api"
  auto_deploy = true

  access_log_settings {
    destination_arn = aws_cloudwatch_log_group.api_gw.arn

    format = jsonencode({
      requestId               = "$context.requestId"
      sourceIp                = "$context.identity.sourceIp"
      requestTime             = "$context.requestTime"
      protocol                = "$context.protocol"
      httpMethod              = "$context.httpMethod"
      resourcePath            = "$context.resourcePath"
      routeKey                = "$context.routeKey"
      status                  = "$context.status"
      responseLength          = "$context.responseLength"
      integrationErrorMessage = "$context.integrationErrorMessage"
      }
    )
  }
}

resource "aws_apigatewayv2_integration" "lambdas" {
  count = length(var.lambda_integrations)

  api_id             = aws_apigatewayv2_api.default.id
  integration_type   = "AWS_PROXY"
  integration_method = "POST"
  integration_uri    = var.lambda_integrations[count.index].lambda_invoke_arn
}

resource "aws_apigatewayv2_route" "lambdas" {
  count = length(var.lambda_integrations)

  api_id    = aws_apigatewayv2_api.default.id
  route_key = "${var.lambda_integrations[count.index].method} ${var.lambda_integrations[count.index].route}"

  target = "integrations/${aws_apigatewayv2_integration.lambdas[count.index].id}"
}

resource "aws_cloudwatch_log_group" "api_gw" {
  name = "/aws/api_gw/${aws_apigatewayv2_api.default.name}"

  retention_in_days = 30
}
