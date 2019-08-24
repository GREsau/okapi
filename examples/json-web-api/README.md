# JSON Web API

A simple web API written using [Rocket](https://rocket.rs/), with two binaries:
- `main`, a regular application which you can run locally using `cargo run` as normal
- `lambda`, written to be run as an AWS Lambda function, using [Rocket Lamb](https://github.com/GREsau/rocket-lamb)

The openapi document will be hosted at `/openapi/openapi.json`, and the Swagger UI will be at `/swagger-ui`.

## Deploying to AWS Lambda
Deployment can be done using AWS CloudFormation using the [Serverless Application Model](https://docs.aws.amazon.com/lambda/latest/dg/serverless_app.html). The required CloudFormation template is already set up in [template.yaml](template.yaml).

Requirements:
- Docker
- [AWS CLI](https://aws.amazon.com/cli/)
- An existing S3 bucket

```sh
# Builds the lambda binary in a Docker container and outputs the packaged zip file
docker-compose run --rm build

S3_BUCKET=my-s3-bucket-name
# Choose any name you like for the CloudFormation stack
STACK_NAME=my-rocket-api

# Uploads the CloudFormation template and zipped binary to S3
aws cloudformation package --template-file template.yaml --output-template-file packaged.yaml --s3-bucket $S3_BUCKET

# Deploys the CloudFormation stack
aws cloudformation deploy --template-file packaged.yaml --capabilities CAPABILITY_IAM --stack-name $STACK_NAME

# Outputs the API Gateway URL that you can use to call your API
aws cloudformation describe-stacks --query "Stacks[0].Outputs" --stack-name $STACK_NAME
```
