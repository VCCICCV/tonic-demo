use tonic::transport::Channel;
use example::example_service_client::ExampleServiceClient;
use example::RequestMessage;
use tokio_stream::StreamExt;

pub mod example {
    // tonic::include_proto!("example");
    include!("./generated/example.rs");
}
// 一元RPC调用
async fn unary_call(
    client: &mut ExampleServiceClient<Channel>
) -> Result<(), Box<dyn std::error::Error>> {
    let request = tonic::Request::new(RequestMessage {
        message: "Hello from unary".into(),
    });
    let response = client.unary_call(request).await?;
    println!("Unary response: {:?}", response.into_inner());
    Ok(())
}
// 服务端流式RPC调用
async fn server_stream(
    client: &mut ExampleServiceClient<Channel>
) -> Result<(), Box<dyn std::error::Error>> {
    let request = tonic::Request::new(RequestMessage {
        message: "Hello from stream".into(),
    });
    let mut response = client.server_stream(request).await?.into_inner();
    while let Some(res) = response.next().await {
        println!("Server stream response: {:?}", res?);
    }
    Ok(())
}
// 客户端流式RPC调用
async fn client_stream(
    client: &mut ExampleServiceClient<Channel>
) -> Result<(), Box<dyn std::error::Error>> {
    let request = tokio_stream::iter(
        vec![
            RequestMessage { message: "Hello".into() },
            RequestMessage { message: "from".into() },
            RequestMessage { message: "client".into() }
        ]
    );
    let response = client.client_stream(request).await?;
    println!("Client stream response: {:?}", response.into_inner());
    Ok(())
}
// 双向流式RPC调用
async fn bidi_stream(
    client: &mut ExampleServiceClient<Channel>
) -> Result<(), Box<dyn std::error::Error>> {
    let request = tokio_stream::iter(
        vec![
            RequestMessage { message: "Hello".into() },
            RequestMessage { message: "from".into() },
            RequestMessage { message: "bidi".into() }
        ]
    );
    let mut response = client.bidi_stream(request).await?.into_inner();
    while let Some(res) = response.next().await {
        println!("Bidi stream response: {:?}", res?);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接到gRPC服务器
    let mut client = ExampleServiceClient::connect("http://[::1]:50051").await?;

    unary_call(&mut client).await?;
    server_stream(&mut client).await?;
    client_stream(&mut client).await?;
    bidi_stream(&mut client).await?;

    Ok(())
}
