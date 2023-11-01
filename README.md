# Rust SQS Lambda Reader

### Purpose
Example repository highlighting how to read from an SQS Queue from a Lambda function
that is built in Rust and deployed with SAM

### Key Files

- `src` - Source for the Rust code
- `Cargo.toml` - Project configuration file.
- `template.yaml` - A template that defines the application's AWS resources.

### Requirements

-   [SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html)
-   [Docker](https://hub.docker.com/search/?type=edition&offering=community)
-   [Rust](https://www.rust-lang.org/tools/install)
-   [Cargo Lambda](https://www.cargo-lambda.info/)

### Deploy the sample application

To deploy the application, you need the folllowing tools:

```bash
 sam build
 sam deploy
```


### Cleanup

To delete the sample application that you created, use the AWS CLI. Assuming you used your project name for the stack name, you can run the following:

```bash
sam delete
```
