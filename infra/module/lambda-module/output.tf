output "invoke_arn" {
  value = aws_lambda_function.lambda.invoke_arn
}

output "arn" {
  value = aws_lambda_function.lambda.arn
}

output "permission_id" {
  value = length(aws_lambda_permission.s3) == 0 ? null : aws_lambda_permission.s3[0].id
}
