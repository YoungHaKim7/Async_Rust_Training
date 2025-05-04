use std::task::Poll;

trait SimpleFuture<Output> {
    fn poll(&mut self) -> Poll<Output>;
}

struct FairDice;

impl SimpleFuture<u8> for FairDice {
    fn poll(&mut self) -> Poll<u8> {
        Poll::Ready(4) // chosen by fair dice roll
    }
}

fn main() {
    println!("Hello, world!");
}
