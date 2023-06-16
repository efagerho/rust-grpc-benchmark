use echo::echo_service_client::EchoServiceClient;
use echo::HelloRequest;

pub mod echo {
    tonic::include_proto!("echo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EchoServiceClient::connect("http://[::1]:8080").await?;

    println!("Connected to server");

    let request = tonic::Request::new(HelloRequest {
        name: "Ed".into(),
    });

    let response = client.hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
