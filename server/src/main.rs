use tokio::runtime::Runtime;

fn main() {

    let rt = Runtime::new().expect("Runtime error { tokio failed to create async runtime... }");

    rt.block_on( async {
        run().await;
    });
}

async fn  run(){

}
