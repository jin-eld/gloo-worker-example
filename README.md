# Example of how to use gloo-workers

Despite existing examples in the [gloo-worker](https://github.com/rustwasm/gloo)
repo I still had a hard time figuring out how to put all the bits and pieces
together and get everything running in a browser.

I am new to Rut, let me know if anythin presented here can be done in a better 
way, my findings are mostly based on studying gloo exampls and some trial and 
error.

The part that was evading me seems to be the fact that one has to share
the `#[reactor]` code between the controller and the worker, so that the
controller can call `spawn()` on it, while the worker has to call `register()`.

# Building

To build manually run the following commands:
```
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/controller.wasm --out-dir ./www --target web
wasm-bindgen target/wasm32-unknown-unknown/release/worker.wasm --out-dir ./www --target no-modules
```

I did not find a way to install post-build hooks in cargo, I did not want
to use a shell script for building as it would only work on Linux, so
a small rust program combined with a cargo alias will handle the above
steps. Again, if there are better way to do this - please let me know.

```
cargo web-build
```

The output will be written to `./www` which already contains an `index.html`
to get things started. The index page will load the controller, which
in turn will spawn a worker, send it some data and receive the result.

Expose the `./www` directory via your favorite browser nad navigate the
browser there in order to test the application.

As the goal was to figure out how web workers can be spawned, there is no UI,
the output can only be seen in the browser console.


# Notes
It is important to specify rlib in addition to cdylib in Cargo.toml, otherwise 
importing local libraries will not work:
```
crate-type = ["cdylib", "rlib"]
```

The worker has to be built with `--target no-modules`, the controller which
is launched from a webpage can be either `--target web` or `--target no-modules`
depending on the integration.

