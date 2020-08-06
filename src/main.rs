/*
	Ping-Pong
	8/5/2020
	A ping client and a pong client each open two sockets and receive on one and send on the other.
*/

use std::process::exit;

fn parse_args() -> Option<String>{
	let args: Vec<String> = std::env::args().collect();
	if args.len() <= 1 {
		print_usage_and_exit();
		None
	}else{
		// print_args(&args);
		match args[1].as_str() {
			"ping" => Some(String::from("ping")),
			"pong" => Some(String::from("pong")),
			_ => {
				print_usage_and_exit();
				None
			}
		}
	}
}

fn print_usage_and_exit(){
	println!("Usage: [ping/pong]");
	exit(0)
}

fn run_ping(){
	println!("[run_ping]");

	std::thread::spawn(move ||{
		let socket_out = std::net::UdpSocket::bind("127.0.0.1:11110").expect("couldn't bind to address");
		// socket_send.connect("0.0.0.0:11111").expect("connect function failed");
		let mut i:u8 = 0;
		loop{
			let mut mesg_vec = vec![];
			//let ping_message:String= format!("ping{}", i);
			let ping_bytes:&[u8] = "ping".as_bytes();
			mesg_vec.extend_from_slice(&ping_bytes);
			mesg_vec.extend_from_slice(&[i]);
			println!("sending ping: {:?}", &mesg_vec);
			let _send_result = &socket_out.send_to(&mesg_vec, "127.0.0.1:22221");
			std::thread::sleep(std::time::Duration::from_millis(500));

			match i {
				0..=254 => i+=1,
				_ => i = 0,
			};
		};
	});

	std::thread::spawn(move ||{

		let socket_in = std::net::UdpSocket::bind("127.0.0.1:11111").expect("couldn't bind to address");
	// 	// socket_send.connect("0.0.0.0:11111").expect("connect function failed");
	//
		let mut buf = [0; 10];
		loop{
			match socket_in.recv(&mut buf) {
				Ok(received) => {
					println!("received {} bytes {:?}", received, &buf[..received]);
				},
				Err(e) => println!("recv function failed: {:?}", e),
			}
			std::thread::sleep(std::time::Duration::from_millis(1000));
		};
	});

	// let the other threads run for a while
	std::thread::sleep(std::time::Duration::from_secs(10));

	println!("[run_ping] Started pinger.");
}

// TODO change the counter size to u64
// UNDER CONSTRUCTION
fn parse_ping(buf: &[u8])-> (bool, u8){

	let ping_candidate:&[u8] = &buf[0..=3];
	let ping_string = String::from_utf8(ping_candidate.to_vec()).unwrap();
	let ping_count:&u8 = &buf[4];
	assert_eq!(ping_string, String::from("ping"));

	(true, ping_count.clone())

}

/// Return a one-for-one pong for each ping received
fn run_pong(){
	println!("[run_pong]");
	let socket_out = std::net::UdpSocket::bind("127.0.0.1:22220").expect("couldn't bind to address");
	let socket_in = std::net::UdpSocket::bind("127.0.0.1:22221").expect("couldn't bind to address");
	// socket.connect("127.0.0.1:22221").expect("connect function failed");

	let mut buf = [0; 10];
	loop{
		match socket_in.recv(&mut buf) {
			Ok(byte_count_rcvd) => {

				println!("received {} bytes {:?}", byte_count_rcvd, &buf[..byte_count_rcvd]);


				match parse_ping(&buf){
					(true, ping_id) => {
						println!("Is it ping? Yes! {}", &ping_id);
						let mut pong_vec = vec![];
						pong_vec.extend_from_slice("pong".as_bytes());
						pong_vec.extend_from_slice(&[ping_id]);
						// let pong_send_result = socket_out.send_to("pong".as_bytes(), "127.0.0.1:11111");
						let pong_send_result = socket_out.send_to(&pong_vec, "127.0.0.1:11111");
						println!("Sent pong {:?}", pong_send_result);
					},
					(false, _) => println!("Is it ping? No."),
					// _ => println!("Should never get here."),
			}



			}
			Err(e) => println!("recv function failed: {:?}", e),
		}
	}
}

fn main() {

	match parse_args().unwrap().as_str() {
		"ping" => run_ping(),
		"pong" => run_pong(),
		_ => (),
	}
}
