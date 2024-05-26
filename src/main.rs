use proxy::server::Server;
use std::net::ToSocketAddrs;
fn main() {
    let addr = "192.168.101.1:8000".to_socket_addrs();
    let mut server: Server = Server::new();
    server.config();
    let rt = tokio::runtime::Runtime::new().expect("Tokio Start Failed!");
    let run = async {
	server.run().await;
    };
    rt.block_on(run);
}
