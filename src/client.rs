use std::time::Duration;

use helloworld::{greeter_client::GreeterClient, DownloadRequest, HelloRequest};
use tokio::io::AsyncWriteExt;


pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // 设置超时时间为 5 秒
    let timeout = Duration::new(30, 0);

    // 创建连接，设置超时
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .timeout(timeout) // 设置超时
        .connect()
        .await?;
    let mut client = GreeterClient::new(channel);
    let request = tonic::Request::new(HelloRequest {
        name: "东亚".into(),
    });

    let response = client.say_hello(request).await;

    match response {
        Ok(resp) => {
            println!("Received: {:?}", resp.into_inner());
        }
        Err(e) => {
            println!("Error occurred: {:?}", e);
        }
    }
  
    

    //上传文件
    let file_name = "guava-32.0.1-jre.jar";
    let file_content = tokio::fs::read("E:\\devtools\\apache-maven-3.9.6\\lib\\guava-32.0.1-jre.jar").await?;

    let request = tonic::Request::new(helloworld::FileRequest {
        file_name: file_name.to_string(),
        file_content,
    });

    let response = client.upload_file(request).await?;
    println!("Response: {:?}", response.into_inner());


    //下载文件
    let file_name = "guava-32.0.1-jre.jar"; // 需要下载的文件名
    // 请求下载文件
    let request = tonic::Request::new(DownloadRequest {
        file_name: file_name.to_string(),
    });

    let response = client.download_file(request).await?;

    let download_response = response.into_inner();


    // 保存下载的文件
    // 拼接文件名，添加 "dwl" 前缀
    let new_file_name = format!("dwl_{}", download_response.file_name);
    // 保存下载的文件
    let mut file = tokio::fs::File::create(new_file_name.clone()).await?;
    file.write_all(&download_response.file_content).await?;

    println!("File downloaded successfully: {}", new_file_name);


    Ok(())
}
