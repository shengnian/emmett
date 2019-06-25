use futures::future::lazy;
use std::path::Path;

use emmett::Pipeline;

fn main() {
    let config = Path::new("./example_configs/full.toml");

    tokio::run(lazy(move || {
        Pipeline::from_toml(config)
            .expect("Couldn't create Pipeline from TOML")
            .run();
        Ok(())
    }));
}
