# n3rgy_consumer_api_client

A Rust client for accessing electricity and gas smart meter data in the United Kingdom via the [n3rgy consumer data](https://www.n3rgy.com/consumer/) REST API.

**Note:** This library is not affiliated with or endorsed by n3rgy.

## Prerequisites

To access your data, you must enroll in the n3rgy consumer access service. Follow these steps:

1. **Visit the n3rgy website**: Go to the [n3rgy consumer data](https://www.n3rgy.com/consumer/) page.
2. **Retrieve your IHD MAC address**: Get your In Home Device (IHD) MAC address from the settings of your IHD device.
3. **Complete the enrollment process**: Follow the instructions on the website to complete your enrollment.

## Example usage

Set the environment variable `N3RGY__APIKEY` to your In Home Device (IHD) MAC address, formatted without hyphens.

```rust
use chrono::{Days, Local};
use n3rgy_consumer_api_client::{ConsumerApiClient, EnvironmentAuthorizationProvider};

#[tokio::main]
async fn main() {
    let ap = EnvironmentAuthorizationProvider {};
    let client = ConsumerApiClient::new(ap, None);

    let today = Local::now().date_naive();
    let yesterday = today.checked_sub_days(Days::new(1)).unwrap();

    if let Ok(consumption) = client.get_electricity_consumption(yesterday, today).await {
        println!(
            "Retrieved {} electricity consumption records",
            consumption.len()
        );
    }

    if let Ok(consumption) = client.get_electricity_tariff(yesterday, today).await {
        println!("Retrieved {} electricity tariff records", consumption.len());
    }

    if let Ok(consumption) = client.get_gas_consumption(yesterday, today).await {
        println!("Retrieved {} gas consumption records", consumption.len());
    }

    if let Ok(consumption) = client.get_gas_tariff(yesterday, today).await {
        println!("Retrieved {} gas tariff records", consumption.len());
    }
}
```

For interactive user input, consider using `StaticAuthorizationProvider::new(api_key)` rather than `EnvironmentAuthorizationProvider`.

### Important security note

API keys are sensitive information. Ensure they are not stored in source control. Use environment variables or secure vaults to manage them safely.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
