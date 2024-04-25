use pcap::Capture;

fn main() {
    for device in pcap::Device::list().unwrap() {
        println!("{:?}", device);
    }
    let mut cap = Capture::from_device("eth0").unwrap()
        .promisc(true)
        .snaplen(90789)
        .timeout(1000)
        .open().unwrap();
    for _ in 0..10 { {
        match cap.next_packet() {
            Ok(packet) => { println!("{:?}", packet); },
            Err(e) => { println!("{:?}", e); }
        }
    }
        cap.close().unwrap();
    }
}