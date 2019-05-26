> Note: This project is currently under heavy development, and contributors are welcome! Feedback is also greatly appreciated.

# About

emmett is an ETL tool with the goal of serving as a drop-in [Logstash](https://www.elastic.co/products/logstash) replacement as well as
providing added / improved features.

emmett is written in [Rust](https://www.rust-lang.org/) and uses [Tokio](https://tokio.rs/), which means:

- Blazing fast ⚡⚡⚡
- Very small resource requirements (currently needs less than 3 MB RAM for simple pipelines)
- No garbage collector == better and more consistent performance
- Completely asynchronous - plugins operate independently of one another
- Multithreaded and [work-stealing](https://en.wikipedia.org/wiki/Work_stealing)
- No need to install Java

# Usage

emmett uses TOML for configuration, and can also parse [Logstash configuration files](https://www.elastic.co/guide/en/logstash/7.0/configuration-file-structure.html).

The Logstash configuration file parser is currently on hold until the plugins themselves become more stable.
Merge requests are always welcome though!

```toml
# This is an emmett config file.

[input.stdin]

[filter.grok]
match = { message = "%{COMBINEDAPACHELOG}" }
    
[filter.date]
match = [ "timestamp" , "dd/MMM/yyyy:HH:mm:ss Z" ]	

[output.stdout]
codec = "rubydebug"

[output.elasticsearch]
hosts = ["localhost:9200"]
```
