# microservice-rs
Plumbing for Rust microservices. 

## Overview

This crate aims to identify key functionality of good Rust microservices, and provide it with a simple procedural macro, removing the need for boilerplate code.

Currently, the crate focuses on the following functionality:

* **Structured Logging**. A `slog::Logger` is configured for JSON output to stdout by default. The user is encouraged to structure their log messages as much as possible.
* **Configuration**: The user can define a configuration type which must implement `Deserialize`. It is automatically loaded from a YAML file, with overrides from environment variables.
* **Signal Handling**: Proper microservices running in Kubernetes or Docker should be able to handle `SIGINT` or `SIGTERM` to shut down gracefully. 

Here is an example of a basic application which makes use of this crate:

```rust
use slog::{Logger, info, warn, debug};
use serde::Deserialize;

#[derive(Debug)]
struct Error;

#[derive(Deserialize, Debug)]
struct Config {
    iter: usize
}

#[microservice::main]
fn main(logger: Logger, config: Config, signal: Signal) -> Result<(), Error> {
    info!(logger, "Hello World.");
    let mut n = 0;
    loop {
        debug!(logger, "Iteration.";
            "n" => n
        );
        if signal.check() {
            warn!(logger, "Detected signal.");
            break;
        }
        n += 1;
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    Ok(())
}
```

Logging, configuration parsing, and signal handling are all injected right into your main function.


## Todo

 - [ ] Async main support.
 - [ ] Catch panics.
 - [ ] Rewrite return error type to include plumbing errors.
 
 
