# Rust AWS SDK example with hardcoded credentials

> IMPORTANT: DO NOT hard code credentials ever, unless the design doesn't allow
> any of the alternative ways of providing access to the AWS API.

This example takes the next step and uses the
`aws_credential_types::Credentials::from_keys()` function to demonstrate hard
coding credentials in the code. This is for demo purposes only. I'm warning
again not to do this in real life. A mandatory security training will be the least of
your worries.

```rust
// IMPORTANT: demo only. DO NOT hardcode credentials!!!
let credentials: Credentials = Credentials::from_keys("ACCESS_KEY", "SECRET_KEY", Some("SESSION_TOKEN".to_string()));
let shared_config: Config = Config::builder()
    .credentials_provider(credentials)
    .region(Region::new("ap-southeast-2"))
    .build();
let client: Client = Client::from_conf(config);
```

Note that when using approaches that build a `Credentials` struct, we are using
the `aws_sdk_cloudtrail::Config::builder()` function to set the
`credentials_provider`.

