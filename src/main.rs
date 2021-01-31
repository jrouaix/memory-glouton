use std::time::Duration;

use anyhow::{anyhow, Result as AnyResult};
use bastion::prelude::*;
use log::{error, warn};

static RUN_FOR: Duration = Duration::from_secs(60);
static CHILD_LIFE: Duration = Duration::from_millis(200);

// RUST_LOG=warn cargo run
fn main() -> AnyResult<()> {
    println!(
        "make sure you run this program with `RUST_LOG=warn` so you can get the wake up messages"
    );
    env_logger::init();
    Bastion::init();
    Bastion::start();

    Bastion::supervisor(|sp| {
        sp.with_strategy(SupervisionStrategy::OneForOne)
            .children(|children| {
                children.with_exec(|_ctx: BastionContext| async move {
                    warn!("hey I just woke up");
                    std::thread::sleep(CHILD_LIFE);
                    error!("Ok I'm outta here");
                    // We want to exit with an error so the supervisor respawns us
                    Err(())
                })
            })
    })
    .map_err(|_| anyhow!("woops"))?;

    let task = blocking! {
        std::thread::sleep(RUN_FOR);
    };
    run!(task);

    // We are stopping bastion here
    Bastion::stop();
    Ok(())
}
