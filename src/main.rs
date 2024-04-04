use std::{sync::mpsc, thread};
fn main() {
    let receiver = make_channel();
    loop {
        let recv = receiver.recv();
        // 프로그램이 터지지 않게 recv값이 err이 아닌지 확인하고 unwrap한다
        if recv.is_ok() {
            println!("use send: {}", recv.unwrap());
        }
    }
}

fn make_channel() -> mpsc::Receiver<String> {
    let (sender, receiver) = mpsc::channel();
    // 스레드를 생성해서 reciver(값을 받는 client)를 반환하고
    // sender(값을 보내는 server)를 스레드에 가둬 입력받을때마다 값을 보내게 한다.
    thread::spawn(move || loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        sender.send(s.trim().to_string()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    receiver
}
