# Rust 的 `Future` 是基于轮询的，这种方式不会有性能问题吗？

`Future` 的轮询是带通知机制的轮询，与传统意义上的轮询不完全一样。

当[执行器](<> "executor")调用 `Future` 的 [`poll`][poll] 方法时会传入一个 [`Waker`][waker]，而 `Future` 可以将这个 `Waker` 保存起来，当自己的状态有所变化时，通过其通知执行器可以再次对自己进行轮询。通过这个机制，执行器可以避免反复轮询一个未准备好的 `Future`，避免了传统轮询带来的性能问题。


[poll]: https://doc.rust-lang.org/std/future/trait.Future.html#tymethod.poll
[waker]: https://doc.rust-lang.org/std/task/struct.Waker.html
