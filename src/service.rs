
use helloworld::greeter_server::{Greeter, GreeterServer};
use helloworld::{HelloReply,HelloRequest};



use tonic::{transport::Server, Request, Response, Status};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    //打招呼
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;

        println!("client name: {}",name);
        let reply = HelloReply {
            message: format!("Hello {}!", name),
        };
        Ok(Response::new(reply))
    }

    //上传文件
    async fn upload_file(
        &self,
        request: Request<helloworld::FileRequest>,
    ) -> Result<Response<helloworld::FileResponse>, Status> {
        let file_request = request.into_inner();

        let file_name = file_request.file_name;
        let file_content = file_request.file_content;

        let mut file = File::create(file_name).await.map_err(|e| {
            Status::internal(format!("Failed to create file: {}", e))
        })?;

        file.write_all(&file_content).await.map_err(|e| {
            Status::internal(format!("Failed to write file: {}", e))
        })?;

        let reply = helloworld::FileResponse {
            message: "File uploaded successfully".to_string(),
        };

        Ok(Response::new(reply))
    }

    //下载文件
    async fn download_file(
        &self,
        request: Request<helloworld::DownloadRequest>,
    ) -> Result<Response<helloworld::DownloadResponse>, Status> {
        let file_name = &request.into_inner().file_name;

        // 打开文件并读取内容
        let mut file = match File::open(file_name).await {
            Ok(file) => file,
            Err(e) => {
                return Err(Status::not_found(format!("File not found: {}", e)));
            }
        };

        let mut file_content = Vec::new();

        match file.read_to_end(&mut file_content).await {
            Ok(_) => (),
            Err(e) => return Err(Status::internal(format!("Failed to read file: {}", e))),
        };

        let reply = helloworld::DownloadResponse {
            file_name: file_name.to_string(),
            file_content,
        };

        Ok(Response::new(reply))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Greeter server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
