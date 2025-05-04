use tokio::runtime;

async fn asyncfn_test() {
    println!("async function");
}
fn main() {
    let async_print_test = "test";
    println!("{}", async_print_test);
    let runtime_val = runtime::Runtime::new().unwrap();
    runtime_val.block_on(asyncfn_test());
}
