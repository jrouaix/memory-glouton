use std::time::Duration;

use bastion::prelude::*;

fn main() {
    Bastion::init();
    Bastion::start();

    let start_time = std::time::SystemTime::now();
    let expected_duration = Duration::from_secs(2);

    loop {
        let workers = Bastion::children(|children| {
            children.with_exec(|ctx: BastionContext| {
                async move {
                    msg! {
                        ctx.recv().await?,
                        _: _ => (); // nothing
                    }
                    Ok(())
                }
            })
        })
        .expect("Couldn't create the children group.");

        workers.stop().expect("Couldn't stop the children group");
        workers.kill().expect("Couldn't kill the children group");

        let now = std::time::SystemTime::now();
        let duration = now.duration_since(start_time).unwrap();
        if duration > expected_duration {
            break;
        }
    }
    // We are stopping bastion here
    Bastion::stop();
}
