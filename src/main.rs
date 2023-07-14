extern crate okosamalunch;

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            okosamalunch::run().await;
        });
}
