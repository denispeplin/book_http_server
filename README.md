## A tiny HTTP server from The Rust Book

This HTTP server is described in The Rust Book
in its [final chapter](https://rust-book.cs.brown.edu/ch20-00-final-project-a-web-server.html)

### Build and run

Given Rust is installed

```
cargo run
```

### TODO

The list of homework from `The Rust Book` right above
[the summary](https://rust-book.cs.brown.edu/ch20-03-graceful-shutdown-and-cleanup.html#summary)
on the last page:

* Add more documentation to `ThreadPool` and its public methods.
* Add tests of the library’s functionality.
* Change calls to `unwrap` to more robust error handling.
* Use `ThreadPool` to perform some task other than serving web requests.
* Find a thread pool crate on [crates.io](https://crates.io/) and implement a
  similar web server using the crate instead. Then compare its API and
  robustness to the thread pool we implemented.
