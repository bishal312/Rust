use std::time::Duration;
use std::thread::sleep;
fn main() {
    one();
}

fn slow(name: &str, ms: u64) {
    sleep(Duration::from_millis(50));
    println!("{name} ran for {ms}ms");
}

fn one() {
    trpl::block_on(
        async {
            let a = async {
                println!("'a' started");
                slow("a", 30);
                slow("a", 10);
                slow("a", 20);
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'a' finished.");
            };

            let b = async {
                println!("'b' started");
                slow("b", 75);
                slow("b", 5);
                slow("b", 20);
                slow("b", 350);
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'b' finished.");
            };

            trpl::select(a, b).await;
        }
    )
}