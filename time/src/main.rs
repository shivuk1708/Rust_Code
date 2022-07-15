use chrono::{DateTime, Utc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn time() {
    //let time2 = system_time_to_date_time(time);
    let st_now = SystemTime::now();
    println!("{:?}\n ", st_now);
    let dt_now: DateTime<Utc> = st_now.clone().into();
    println!("{}\n ", dt_now);
}

use std::mem;
use std::io;

pub fn cpu_time() -> Duration {
    unsafe {
        let mut tp = std::mem::uninitialized();
        if sys::clock_gettime(sys::CLOCK_MONOTONIC, &mut tp) == 0 {
            Duration::new(tp.tv_sec as u64, tp.tv_nsec as u32)
        } else {
            panic!("cpu_time: {}", io::Error::last_os_error());
        }
    }
}

mod sys {
    use libc::{c_int, timespec};
    extern "C" {
        pub fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    }
    pub const CLOCK_PROCESS_CPUTIME_ID: c_int = 2;
}
mod ffi {
    extern {
        pub fn clock() -> ::libc::clock_t;
    }
}
fn main() {
    /*
    let now = SystemTime::now();
    sleep(Duration::new(2, 0));
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("{:?}", elapsed);
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }*/
    time();
    //println!("uptime is {:?}", uptime());
    let start = unsafe { ffi::clock() };

    let mut dummy = 0;
    for i in 0..20000 { dummy += i };

    let end = unsafe { ffi::clock() };
    println!("{}, {}, {}, {}", dummy, start, end, end - start);
    println!("{:?}", cpu_time());
}

