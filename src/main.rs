use futures::future::lazy;
use std::path::Path;

mod pipeline;
use pipeline::Pipeline;

fn main() {

    let example_config = Path::new("./example_configs/full.toml");
    
    tokio::run(lazy(move || {
        Pipeline::from_toml(example_config).run();
        Ok(())
    }));
    
}
