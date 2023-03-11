

#[derive(Debug, Clone)]
struct F {}

impl F {

    async fn run(&self) {

    }
}

pub async fn run() {
    let f = F{};
    let c = f.clone();
    tokio::task::spawn(async move {c.run().await});
    let _f = f;
}
