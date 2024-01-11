# Rust AWS SDK example with overridden configuration

This example takes a step further from the simple client example by specifying
certain details such as the AWS region in the code. It demonstrates using the
builder pattern employed by the Rust AWS SDK to build the configuration struct.
For more configuration options that can be overridden, refer to [the crate
documentation](https://docs.rs/aws-sdk-cloudtrail/latest/aws_sdk_cloudtrail/struct.Config.html).

More details on overriding the credentials provider is shown in the samples 03
and 04.

```rust
let shared_config: SdkConfig = aws_config::from_env()
    // hardcoding the region
    .region("ap-southeast-2")
    .load()
    .await;
let client: Client = Client::new(&shared_config);
```
In the above example, the region is hardcoded to `ap-southeast-2`.

```rust
let shared_config: SdkConfig = aws_config::from_env()
    // hardcoding if the config isn't found in the default loading chain
    .region(RegionProviderChain::default_provider().or_else("ap-southeast-2"))
    .load()
    .await;
let client: Client = Client::new(&shared_config);
```
In contrast, the value for region is set to `ap-southeast-2` only if the
default provider fails to lookup a proper value from the environment variables
or other default value lookup locations.
