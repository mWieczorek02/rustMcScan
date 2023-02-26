#[path = "./mod/address_list.rs"]
mod address_list;

use ipnet::Ipv4Net;

use std::net::TcpStream;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use address_list::get_address_list;

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // thread::spawn(|| loop {
    //     println!("number of threads: {:?}", GLOBAL_THREAD_COUNT);

    //     thread::sleep(Duration::from_millis(5000));
    // });

    get_address_list().iter().for_each(|address| {
        let net4: Ipv4Net = address.parse().unwrap();
        net4.hosts().for_each(|host| {
            GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);

            thread::spawn(move || {
                let conn = TcpStream::connect((host, 25565));

                match &conn {
                    Ok(_) => {
                        let ping_result = mcping::get_status(&host.to_string(), None);
                        match &ping_result {
                            Ok(ping) => {
                                let player_count = ping.1.players.online;
                                if player_count > 0 {
                                    println!(
                                        "player count: {:?} - address: {:?}, version: {:?}",
                                        &player_count, &host, &ping.1.version.name
                                    )
                                };
                            }
                            Err(err) => {}
                        }
                    }
                    Err(_) => {}
                }
                GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
            });

            thread::sleep(Duration::from_millis(50));
        });
    });

    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1));
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
