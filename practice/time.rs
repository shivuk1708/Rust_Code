use std::time::{/*Duration,*/ SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};

fn time() {

    let time = SystemTime::now();

    //let time2 = system_time_to_date_time(time);
    let st_now = SystemTime::now();
    println!("{:?}", st_now);
    let dt_now: DateTime<Utc> = st_now.clone().into();
    println!("{:?}", dt_now);

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
}
