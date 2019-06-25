use futures::future::lazy;
use std::path::Path;

use emmett::Pipeline;

fn main() {
    let example_config = Path::new("./example_configs/full.toml");

    tokio::run(lazy(move || {
        Pipeline::from_toml(example_config)
            .expect("Couldn't create Pipeline from TOML")
            .run();
        Ok(())
    }));
}
