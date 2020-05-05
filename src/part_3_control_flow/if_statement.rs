use systemstat::{System, Platform};
// max uptime in seconds
const MAX_UPTIME: f64 = 8.3  * 60.0 * 60.0;

fn sec_to_hrs(secs: f64) -> f64 {
    secs / 60.0 / 60.0
}

pub fn run() {
    println!("\n====3.13 IF STATEMENT====");
    let sys = System::new();
    let uptime = sys.uptime();
    let uptime = match uptime {
        Ok(uptime) => uptime,
        Err(error) => panic!("Problem get the uptime: {:?}", error),
    };
    let uptime_in_secs = uptime.as_secs_f64();

    if uptime_in_secs > MAX_UPTIME {
        println!("You are {:.2} h with computer. Go rest mfkr!", sec_to_hrs(uptime_in_secs));
    } else {
        let almost = sec_to_hrs(MAX_UPTIME - uptime_in_secs);
        println!("You are {:.2} h with computer. You have to study almost {:.2} h!", sec_to_hrs(uptime_in_secs), almost);
    }
}