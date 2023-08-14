use lambda_runtime::{run, service_fn, Error, LambdaEvent};
//using lib.rs here
use s3l::{client, get_bucket_sizes};

use humansize::{format_size, DECIMAL};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
    size: String,
}

//making bucket size human readable
async fn human_readable() -> String {
    let client = client().await.unwrap();
    let verbose = Some(false);
    let bucket_sizes = get_bucket_sizes(&client, verbose).await.unwrap();
    let mut total_size = 0;
    for size in bucket_sizes {
        total_size += size;
    }
    let size = format_size(total_size as u64, DECIMAL);
    let result = format!("Total Size of all buckets {}", size);
    result
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let name = event.payload.name;
    let result: String = human_readable().await;

    let response = Response {
        req_id: event.context.request_id,
        msg: format!(r#"Event Payload {}"#, name),
        size: result,
    };
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        //disable printing the log every time function invoked
        .with_target(false)
        //disablign time bcz cloud watch will handle it for us
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
