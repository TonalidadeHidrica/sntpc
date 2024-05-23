//! Demonstrates how to make a single NTP request to a NTP server of interest
//!
//! Example provides a basic implementation of [`NtpTimestampGenerator`] and [`NtpUdpSocket`]
//! required for the `sntpc` library
#[cfg(feature = "log")]
use simple_logger;
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, Instant};

const POOL_NTP_ADDR: &str = "ntp.nict.jp:123";

fn main() {
    #[cfg(feature = "log")]
    if cfg!(debug_assertions) {
        simple_logger::init_with_level(log::Level::Trace).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Info).unwrap();
    }

    let ntp_time = || {
        let socket =
            UdpSocket::bind("0.0.0.0:0").expect("Unable to crate UDP socket");
        socket
            .set_read_timeout(Some(Duration::from_secs(2)))
            .expect("Unable to set UDP socket read timeout");
        sntpc::simple_get_time(POOL_NTP_ADDR, socket)
    };

    let origin = Instant::now();
    let origin_ntp = ntp_time().unwrap().sec();

    for _ in 0..60 {
        let before = Instant::now();
        let result = ntp_time();
        let after = Instant::now();

        let calc = |t: Instant| {
            let x = (t - origin).as_nanos();
            let base = 10u128.pow(9);
            format!("{}.{:09}", x / base, x % base)
        };

        match result {
            Ok(time) => {
                assert_ne!(time.sec(), 0);
                let seconds = time.sec() - origin_ntp;
                let microseconds =
                    time.sec_fraction() as u64 * 1_000_000 / u32::MAX as u64;
                println!(
                    "{seconds}.{microseconds:06}\t{}\t{}",
                    calc(before),
                    calc(after)
                );
            }
            Err(err) => println!("Err: {:?}", err),
        }

        thread::sleep(Duration::new(1, 0));
    }
}
