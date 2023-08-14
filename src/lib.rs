use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error};

//create a client
pub async fn client() -> Result<Client, Error> {
    let region_provider = RegionProviderChain::first_try(None)
        .or_default_provider()
        .or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    Ok(client)
}

//get list of all bucket
async fn get_buckets(client: &Client) -> Result<Vec<String>, Error> {
    let mut buckets_name = Vec::new();
    let response = client.list_buckets().send().await.unwrap();
    let buckets = response.buckets().unwrap_or_default();
    for bucket in buckets {
        buckets_name.push(bucket.name().unwrap().to_string());
    }

    Ok(buckets_name)
}

//get size of a single bucket by adding all the objects in it
async fn get_bucket_size(client: &Client, bucket: &str) -> Result<i64, Error> {
    let response = client.list_objects_v2().bucket(bucket).send().await?;
    let contents = response.contents().unwrap_or_default();

    //store in vector
    let mut sizes = Vec::new();
    for object in contents {
        sizes.push(object.size());
    }
    let total_size = sizes.iter().sum();
    println!("Bucket:{},Size:{}", bucket, total_size);
    Ok(total_size)
}

//get each bucket and size of each
pub async fn get_bucket_sizes(client: &Client, verbose: Option<bool>) -> Result<Vec<i64>, Error> {
    let verbose = verbose.unwrap_or(false);

    // Handle errors from get_buckets
    let buckets = match get_buckets(client).await {
        Ok(buckets) => buckets,
        Err(e) => {
            println!("Failed to get buckets: {}", e);
            return Err(e);
        }
    };

    // Handle errors from get_bucket_size
    let mut bucket_sizes = Vec::new();
    for bucket in buckets {
        match get_bucket_size(client, &bucket).await {
            Ok(size) => bucket_sizes.push(size),
            Err(e) => {
                if verbose {
                    println!("Error: {}", e);
                }
            }
        }
    }

    

    Ok(bucket_sizes)
}

