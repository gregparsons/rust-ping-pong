/*
	Ping-Pong
	8/5/2020
	A ping client and a pong client each open two sockets and receive on one and send on the other.
*/

/// Send "pings" until the timer runs out; receive "pongs" back.
/// TODO: Doesn't need two sockets.
fn run_ping(){
	let thread0 = std::thread::spawn(move ||{
		let socket_out = std::net::UdpSocket::bind("127.0.0.1:11110").expect("couldn't bind to address");
		let mut i:u8 = 0;
		loop{
			let mut mesg_vec = vec![];
			let ping_bytes:&[u8] = "ping".as_bytes();
			mesg_vec.extend_from_slice(&ping_bytes);
			mesg_vec.extend_from_slice(&[i]);
			println!("[ping] sent ping: {:?}", &mesg_vec);
			let _send_result = &socket_out.send_to(&mesg_vec, "127.0.0.1:22221");
			std::thread::sleep(std::time::Duration::from_millis(500));

			// reset the counter
			match i {
				0..=254 => i+=1,
				_ => i = 0,
			};
		};
	});

	let thread1 = std::thread::spawn(move ||{
		let socket_in = std::net::UdpSocket::bind("127.0.0.1:11111").expect("couldn't bind to address");
		let mut buf = [0; 10];
		loop{
			match socket_in.recv(&mut buf) {
				Ok(received) => {
					// TODO parse the buffer to see if it's a "pong:[counter]"
					println!("[ping] got pong: {:?}", &buf[..received])
				},
				Err(e) => println!("[ping] recv function failed: {:?}", e),
			}
			// std::thread::sleep(std::time::Duration::from_millis(1000));
		};
	});

	// Don't totally understand rust threads here. One of these is needed to run the pinger. Has no effect in lazy mode.
	let res = thread0.join();
	// std::thread::sleep(std::time::Duration::from_millis(1000));
	// thread1.join().unwrap();

}

/// Check if the buffer contains a "ping"
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
				// println!("[pong] received {} bytes {:?}", byte_count_rcvd, &buf[..byte_count_rcvd]);
				match parse_ping(&buf){
					(true, ping_id) => {
						println!("[pong] got ping: {}, {:?}", &ping_id, &buf[..byte_count_rcvd]);
						let mut pong_vec = vec![];
						pong_vec.extend_from_slice("pong".as_bytes());
						pong_vec.extend_from_slice(&[ping_id]);
						socket_out.send_to(&pong_vec, "127.0.0.1:11111").unwrap();
						println!("[pong] sent pong {:?}", &pong_vec);
					},
					(false, _) => println!("[pong] Datagram was not a ping."),
				}
			}
			Err(e) => println!("[pong] recv function failed: {:?}", e),
		}
	}
}

fn parse_args(){
	let args: Vec<String> = std::env::args().collect();
	if args.len() <= 1 {
		print_usage_and_exit();
	}else{
		// print_args(&args);
		match args[1].as_str() {
			"ping" => run_ping(), // Some(String::from("ping")),
			"pong" => run_pong(), //Some(String::from("pong")),
			"lazy" => run_lazy(),
			_ => print_usage_and_exit(),
		}
	}
}

fn print_usage_and_exit(){
	println!("Usage: [ping/pong]");
	std::process::exit(0)
}

/// Run the ping and pong sides in different threads.
fn run_lazy(){
	let pong_child = std::thread::spawn(move ||{ run_pong() });
	let ping_child = std::thread::spawn(move || { run_ping() });
	pong_child.join().unwrap();
	ping_child.join().unwrap();
}

fn main() {
	parse_args()
}
