# Rust AWS SDK example with the simplest usage

This example loads the configuration from environment variables that can be
specified with variables defined in the [documentation](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/environment-variables.html).

```rust
let shared_config: SdkConfig = aws_config::load_from_env().await;
let client: Client = Client::new(&shared_config);
```

To test this, the following environment variables should be set at least.
1. `AWS_ACCESS_KEY_ID`
1. `AWS_SECRET_ACCESS_KEY`
1. `AWS_REGION`

Others such as `AWS_SESSION_TOKEN` will be used if defined as necessary.
