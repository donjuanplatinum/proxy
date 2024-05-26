use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::env::{Args, self};
/// 服务器实例
pub struct Server {
    config: Config,
    state: Mutex<State>,
}




pub struct Config{
    listen_address: String,
    proxy_address: String,
}
impl Config{
    fn new()->Self{
	Config{
	    listen_address: String::new(),
	    proxy_address: String::new(),
	}
    }
    fn set_listen_address(&mut self,addr: String) ->() {
	self.listen_address = addr;
    }
    fn set_proxy_adrdress(&mut self,addr: String) ->() {
	self.proxy_address = addr;
    }
}



struct State{
    running: bool,
    connections: usize,
    threads: usize,
}

impl State{
    fn new() -> State {
	return State{running: false, connections: 0, threads: 0};
    }
    
}

impl Server{
    /// 处理cli
    pub fn config(&mut self) -> Result<(),()>{
	let args: Args = env::args();
	let mut vec: Vec<String> = vec!();
	for args in args {
	    vec.push(args);
	}
	if vec.len() > 1 && vec.contains(&"--help".to_string()) {
	    help(&vec);
	}

	if vec.len() < 2 {
	    help(&vec);
	}
	self.config.set_listen_address(vec[1].clone());
	self.config.set_proxy_adrdress(vec[2].clone());
	println!("proxy {} listen {}",self.config.listen_address,self.config.proxy_address);
	Ok(())
    }
    pub fn new() -> Self {
	return Server{state: Mutex::new(State::new()),config: Config::new() };
    }

    pub async fn run(&self) -> (){
	// proxy监听
	let listener = TcpListener::bind(&self.config.listen_address).await.expect(&format!("Failed To Listen {}",self.config.listen_address));
	loop {
	    let (mut stream,peer_addr) = listener.accept().await.expect("Failed To accept");
	    println!("Connection From {}",peer_addr);
	    let proxy_address = self.config.proxy_address.clone();
	    tokio::task::spawn(send(stream,proxy_address));

	}
    }
}



async fn send(mut stream:  TcpStream,dest: String) ->() {
    // 监听地址来消息
    stream.readable().await.expect(&format!("Failed To Get Data From {}",stream.peer_addr().unwrap()));
    println!("Get Message!");
    let mut buf = [0;10000];
    // 获取监听地址消息
    let result = stream.try_read(&mut buf).expect(&format!("failed to read stream from {}", stream.peer_addr().unwrap()));
    println!("get {}",result);
    match result {
	0 => {},
	n => {
	    println!("get proxy messge{}",std::str::from_utf8(&buf).unwrap_or("None"));
	    // 打开链接proxy地址
	    let mut stream1 = TcpStream::connect(&dest).await.expect(&format!("Failed To Open Connection To {}",dest));
	    // 发送给proxy
	    stream1.write_all(&buf).await.expect(&format!("Failed To Send To {}",&dest));
	    // 是否可写
	    stream1.readable().await.expect(&format!("Failed To Get Response From {}",dest));
	    println!("对方来信息了");
	    stream1.try_read(&mut buf);
	    println!("get dest messge{}",std::str::from_utf8(&buf).unwrap_or("0"));
	    stream.write_all(&buf).await.expect("没传回去");

	    println!("写回去了{}",std::str::from_utf8(&buf).unwrap_or("0"));
	    
	},
	_=> {},
    }
}

fn help(vec: &Vec<String>) ->(){
    println!("Usage: {} listen_address proxy_address", vec[0]);
    println!("Example: {} 0.0.0.0:3000 192.168.101.1:8000", vec[0]);
    std::process::exit(1);
}

