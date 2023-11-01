mod models;

use aws_lambda_events::dynamodb::EventRecord;
use aws_lambda_events::event::dynamodb::StreamRecord;
use aws_lambda_events::sqs::SqsEventObj;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use crate::models::MainModel;


// function_handler
// Lambda handler code for responding to events read from SQS
async fn function_handler(event: LambdaEvent<SqsEventObj<EventRecord>>) -> Result<(), Error> {
    for r in event.payload.records {
        enrich(r.body.change);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        //.pretty()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()

        .init();

    run(service_fn(function_handler)).await
}

fn enrich(stream: StreamRecord) {
    let mm: MainModel = serde_dynamo::from_item(stream.new_image.into_inner()).expect("(Error) Unwrapping MainModel");
    tracing::info!("{:?}", mm);
}