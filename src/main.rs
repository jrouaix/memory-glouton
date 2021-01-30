use bastion::prelude::*;

fn main() {
    Bastion::init();
    Bastion::start();

    for _i in 1..10000000 {
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
    }
    // We are stopping bastion here
    Bastion::stop();
}
