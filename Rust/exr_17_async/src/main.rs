use futures::executor::block_on;


async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    // let x = ::futures::executor::block_on(async {
    //     ten().await
    // });
    // println!("{}",x);
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}

// връща анонимен тип, който имплементира trait-а `Future<Output = u8>`
async fn five() -> u8 {
    5
}

async fn ten() -> u8 {
    five().await + 5
}



// async fn get_two_sites_async() {
//     // Create two different "futures" which, when run to completion,
//     // will asynchronously download the webpages.
//     let future_one = download_async("https://www.foo.com");
//     let future_two = download_async("https://www.bar.com");
//
//     // Run both futures to completion at the same time.
//     join!(future_one, future_two);
// }

/** Example of good async code
async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
** /