use tokio::signal::unix::{signal, SignalKind};

use std::pin::Pin;
use std::future::Future;
use std::task::{Context, Poll};
use tokio::signal::unix::Signal;

struct MyFuture {
    signal: Signal,
}

impl Future for MyFuture {
    type Output = Option<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        print!("polling MyFuture... ");
        let poll = self.signal.poll_recv(cx);
        match poll {
            Poll::Pending => println!("Pending"),
            Poll::Ready(_) => println!("Ready")
        }
        poll
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let future = MyFuture { signal: signal(SignalKind::interrupt())? }; // use ctrl-c to send interrupt signal
    let future2 = MyFuture { signal: signal(SignalKind::hangup())? };   // use command `$ kill -HUP <PID>` to send hangup signal
    let f1 = tokio::spawn(future);
    let f2 = tokio::spawn(future2);
    let (_o1, _o2) = tokio::join!(f1, f2);

    // // An infinite stream of hangup signals.
    // let mut stream = signal(SignalKind::interrupt())?;
    //
    // // Print whenever a HUP signal is received
    // loop {
    //     stream.recv().await;
    //     println!("got signal interrupt");
    // }

    Ok(())
}