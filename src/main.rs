use std::time::Duration;

use actix::prelude::*;

#[actix_rt::main]
async fn main() {
    let start_time = std::time::SystemTime::now();
    let expected_duration = Duration::from_secs(10 * 10 * 10);

    loop {
        // start new actor
        let addr = MyActor { count: 10 }.start();
        // send message and get future for result
        let _ = addr.send(Ping(10)).await.unwrap(); // if I remove this line, I have a mem leak ... :-?

        let now = std::time::SystemTime::now();
        let duration = now.duration_since(start_time).unwrap();
        if duration > expected_duration {
            break;
        }
    }
    System::current().stop();
}

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        // ctx.stop();
        self.count
    }
}

// impl Drop for MyActor {

// }

fn main_bastion() {
    use bastion::prelude::*;
    use std::time::Duration;

    Bastion::init();
    Bastion::start();

    let start_time = std::time::SystemTime::now();
    let expected_duration = Duration::from_secs(10);

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
