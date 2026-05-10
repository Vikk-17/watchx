// mod error;

use tokio::net::TcpStream;
use std::error::Error;

#[tokio::main]
pub async fn connect() {

    let stream = match TcpStream::connect("google.com:443").await {
        Ok(_) => println!("Connection succeeded", ),
        Err(e) => println!("Connection failed: {}", e),
    };
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpStream;

    #[tokio::test]
    async fn test_raw_tcp_connections() {
        let target = "google.com:443";
        let stream_result = TcpStream::connect(target).await;

        assert!(stream_result.is_ok(), "Failed to connect to {}", target);
        println!("Successfully opened TCP socket to {}", target);
    }

}

