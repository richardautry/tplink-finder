use tokio::time;
use greetings::{start_timer, Timer};

struct SomeTimer {
    seconds: u32,
}

async fn task_that_takes_a_second(some_timer: &SomeTimer) {
    time::sleep(time::Duration::from_secs(1)).await;
    println!("{}", some_timer.seconds);
}

#[tokio::main]
async fn main() {
    // let mut interval = time::interval(time::Duration::from_secs(2));
    // let mut some_timer = SomeTimer {
    //     seconds: 0,
    // };

    // for _i in 0..5 {
    //     interval.tick().await;
    //     task_that_takes_a_second(&some_timer).await;
    //     some_timer.seconds += 1;
    // }
    let length = 5000;
    let mut timer = Timer {
        seconds: 0,
    };

    start_timer(length, &mut timer).await;
}