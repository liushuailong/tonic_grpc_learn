use hello_world::greeter_client::GreeterClient;
use hello_world::{ HelloRequest, AddRequest };

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response.get_ref().message);

    let request =tonic::Request::new(AddRequest{
        num1: 20,
        num2: 30,
    });

    let response = client.add(request).await?;
    println!("RESPONSE={:?}", response.get_ref().sum);
    Ok(())
}