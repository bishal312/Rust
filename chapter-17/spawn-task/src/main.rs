use std::time::Duration;

fn main() {
    // first_fn();
    // second_fn();
    third_fn();
}
// Note: Because all of this async code runs in an async block in a trpl::block_on call,
// everything within it can avoid blocking. However, the code outside it will 
// block on the block_on function returning. That’s the whole point of the trpl::block_on 
// function: it lets you choose where to block on some set of async code, and thus where to
//  transition between sync and async code.

// fn first_fn() {
//     trpl::block_on(async {
//         let (tx, mut rx) = trpl::channel();

//         let val = String::from("hi");
//         tx.send(val).unwrap();

//         let received = rx.recv().await.unwrap();
//         trpl::sleep(Duration::from_millis(500)).await;
//         println!("received: '{received}'");
//     })
// }

// fn second_fn() {
//     trpl::block_on(async {
//         let (tx, mut rx) = trpl::channel();
//         let vals = vec![
//             String::from("Hi"),
//             String::from("I"),
//             String::from("am bsal"),
//         ];
//         for val in vals {
//             tx.send(val).unwrap();
//             trpl::sleep(Duration::from_millis(500)).await;
//         }

//         while let Some(value) = rx.recv().await {
//             println!("received '{value}'");
//         }
//     })
// }

// The code now successfully sends and receives all of the messages.
// Unfortunately, there are still a couple of problems. For one thing,
// the messages do not arrive at half-second intervals. They arrive all
// at once, 2 seconds (2,000 milliseconds) after we start the program.
// For another, this program also never exits! Instead, it waits forever
// for new messages. You will need to shut it down using ctrl-C.

fn third_fn() {
    trpl::block_on(
        async { 
                let (tx,mut rx) = trpl::channel();
    let tx_fut =  async {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
            String::from("good"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let rx_fut = async {
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    };

    trpl::join(tx_fut, rx_fut).await;
        }
    )
}