# Rust AWS SDK examples

This repository contains some simple examples on using the Rust AWS SDK with a
few edge cases.

The credentials and the config are loaded from the default locations similar to
the other language SDKs. This is demonstrated in the
[`01_simple_client`](./01_simple_client/) example. Variations to this approach
are demonstrated in the other three examples.

This example uses the CloudTrail service as a specific scenario. However the
usage is more or less similar across the other services.
1. create a config
1. create the client using the config
1. make the API calls

Each service has the crate documentation for more details.

```rust
use aws_config::SdkConfig;
use aws_sdk_cloudtrail::operation::list_trails::ListTrailsOutput;
use aws_sdk_cloudtrail::{
    Client, Error,
};

//...

// create a config
let shared_config: SdkConfig = aws_config::load_from_env().await;
// create the client using the config
let client: Client = Client::new(&shared_config);

// make the API calls
let req = client.list_trails();
let resp: ListTrailsOutput= req.send().await?;
```

The SDK needs an async runtime since almost all calls made by the SDK will be
blocking calls waiting on network calls, file reads etc. The example uses
`tokio` however AWS mentions that any runtime with a thread sleep
function can be used, something that I haven't tried yet.

```rust
#[tokio::main]
async fn main() -> Result<(), Error> {
//...
}
```

Additionally, the SDK internally uses `hyper` HTTP framework, which also seems
to be inter-changeable with the advanced client building APIs provided by the
SDK.
