# Rust AWS SDK example with a custom credentials provider

There'll be use cases where the logic runs on non-AWS infrastructure with no
way to use functionalities such as AWS IAM Roles Anywhere. In this case, it's
better to integrate credentials retrieval with (say) the internal secret store
so that credentials do not have to be exposed until the last moment they are
used. Doing this involves writing a custom credentials provider.

It's fairly straightforward to write a custom credentials provider. The steps
are
1. Define the struct that denotes the custom credentials provider
1. Implement the logic that retrieves the credentials as an `async` function
   which returns a `aws_credential_types::provider::Result`.
1. Implement the trait `aws_credential_types::provider::ProvideCredentials` for
   the struct, which is ultimately a single function `provide_credentials()`
   which returns a future that returns the credentials.


```rust
// create the type for the custom credentials provider
#[derive(Debug)]
struct CustomCredentialsProvider;

// implement the async logic to generate/retrieve credentials (ex:) from a secret store
impl CustomCredentialsProvider {
    async fn load_credentials(&self) -> provider::Result {
        println!("returning custom hardcoded credentials");
        // IMPORTANT: demo only. DO NOT hardcode credentials!!!
        Ok(Credentials::new("ACCESS_KEY", "SECRET_KEY", Some("SESSION_TOKEN".to_string()), None, "CustomProvider"))
    }
}

// implement ProvideCredentials and return a future for the credentials retrieval logic
impl ProvideCredentials for CustomCredentialsProvider {
    fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials<'a> where Self: 'a {
        future::ProvideCredentials::new(self.load_credentials())
    }
}
```

Then use this as the Credentials Provider when building the config struct.

```rust
let shared_config: Config = Config::builder()
    .credentials_provider(CustomCredentialsProvider)
    .region(Region::new("ap-southeast-2"))
    .build();
let client: Client = Client::from_conf(config);
```
