# multithreaded-web-server
The "Building a Multithreaded Webserver" project from the Rust book.

---

# Usage

Currently this project is not on `crates.io` because it's a 'toy' project built from the [Rust Book](https://doc.rust-lang.org/stable/book/ch20-00-final-project-a-web-server.html). So there are no installation instructions.

However, you can clone this repo and run `cargo run` to run it. Then navigate to `127.0.0.1:7878` to see the HTTP request being served. Additionally, you may also navigate to `127.0.0.1:7878/sleep` (which causes the thread to hang for 5 seconds). Then you can navigate to another instance of `127.0.0.1:7878` which will be served while the other thread is hanging.