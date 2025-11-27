use wreq::Client;
use wreq_util::{Emulation, EmulationOS, EmulationOption};

#[tokio::main]
async fn main() -> Result<(), wreq::Error> {
    // Build a client to emulation random devices
    let emulation = EmulationOption::builder()
        .emulation(Emulation::Firefox128)
        .emulation_os(EmulationOS::Windows)
        .build();

    // Apply the emulation to the client
    let client = Client::builder()
        .emulation(emulation)
        .cert_verification(false)
        .build()?;

    // Use the API you're already familiar with
    let text = client
        // .get("https://tls.peet.ws/api/all")
        // .get("https://fanpa.weneedstudy.cn:8443/complexTest")
        .get("https://47.113.101.23:4442/")
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text);

    Ok(())
}
