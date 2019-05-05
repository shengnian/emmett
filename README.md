# About

Emmett is an ETL tool with the goal of supporting drop-in Logstash functionality through a config setting.

Note: This project is currently under heavy development, and contributors are welcome! Feedback is also greatly appreciated.

Emmett is written in Rust and uses [Tokio](https://tokio.rs/), which means:

- Blazing fast
- Very small resource requirements (currently needs less than 5 MB RAM for simple pipelines)
- No garbage collector == better and more consistent performance
- Completely asynchronous - plugins operate independantly of one another
- No need to install Java