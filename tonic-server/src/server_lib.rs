use tonic::{ Request, Response, Status, Streaming };
use tokio_stream::{ wrappers::ReceiverStream, StreamExt };
use std::pin::Pin;
use tokio::sync::mpsc;

pub mod example {
    // 1. 导入文件描述符集
    pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("../../proto/example_descriptor.bin");
    // tonic::include_proto!("example");// 你可以ctrl点击查看源码，它是一个宏，添加了env的编译目录，我们这里指定生成的rust目录，所以直接导入即可
    include!("./generated/example.rs");
}

use example::example_service_server::{ ExampleService, ExampleServiceServer };
use example::{ RequestMessage, ResponseMessage };

#[derive(Debug, Default)]
pub struct MyExampleService;

#[tonic::async_trait]
impl ExampleService for MyExampleService {
    // 一元RPC调用
    async fn unary_call(
        &self,
        request: Request<RequestMessage>
    ) -> Result<Response<ResponseMessage>, Status> {
        let response = ResponseMessage {
            message: format!("Unary response to {}", request.into_inner().message),
        };
        Ok(Response::new(response))
    }

    type ServerStreamStream = Pin<
        Box<dyn tokio_stream::Stream<Item = Result<ResponseMessage, Status>> + Send>
    >;
    // 服务端流式RPC调用
    async fn server_stream(
        &self,
        request: Request<RequestMessage>
    ) -> Result<Response<Self::ServerStreamStream>, Status> {
        let message = request.into_inner().message;
        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            for i in 0..3 {
                let response = ResponseMessage {
                    message: format!("Stream response {} to {}", i, message),
                };
                tx.send(Ok(response)).await.unwrap();
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx)) as Self::ServerStreamStream))
    }
    // 客户端流式RPC调用
    async fn client_stream(
        &self,
        request: Request<Streaming<RequestMessage>>
    ) -> Result<Response<ResponseMessage>, Status> {
        let mut stream = request.into_inner();
        let mut messages = Vec::new();

        while let Some(req) = stream.next().await {
            let req = req?;
            messages.push(req.message);
        }

        let response = ResponseMessage {
            message: format!("Received messages: {:?}", messages.join(", ")),
        };
        Ok(Response::new(response))
    }

    type BidiStreamStream = Pin<
        Box<dyn tokio_stream::Stream<Item = Result<ResponseMessage, Status>> + Send>
    >;
    // 双向流式RPC调用
    async fn bidi_stream(
        &self,
        request: Request<Streaming<RequestMessage>>
    ) -> Result<Response<Self::BidiStreamStream>, Status> {
        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            while let Some(req) = stream.next().await {
                let req = req.unwrap();
                let response = ResponseMessage {
                    message: format!("Echo: {}", req.message),
                };
                tx.send(Ok(response)).await.unwrap();
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx)) as Self::BidiStreamStream))
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let example_service = MyExampleService::default();

    println!("Server listening on {}", addr);
    // 2. 创建反射服务
    let reflection_service = tonic_reflection::server::Builder
        ::configure()
        .register_encoded_file_descriptor_set(example::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    tonic::transport::Server
        ::builder()
        .add_service(ExampleServiceServer::new(example_service))
        .add_service(reflection_service) // 3. 添加反射服务
        .serve(addr).await?;

    Ok(())
}
