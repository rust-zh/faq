# 标准库的 `Future`、`futures` crate、`tokio` 和 `async-std` 等之间的关系是什么？

标准库的 [`Future`][future] trait 以及相关的 [`Context`][context]、[`Pin`][pin]、[`Waker`][waker] 等是核心。由于编译器编译异步函数（`async fn`）需要依赖它们的定义，因而它们必须被包含在标准库里。

[`futures`][futures] 是 `Future` 的扩展，提供了许多虽不必进入标准库但依然重要的基础性的东西，比如 [`FutureExt`][future-ext]、[`StreamExt`][stream-ext] 等扩展 trait 和基础的[通道][channel]、[执行器][executor]实现等。

[`tokio`][tokio] 和 [`async-std`][async-std] 是同一个层次的，主要提供异步运行时的实现，都依赖 `futures` 提供的元语，但因为处理的层次不同，所以可以看到一些自定义的与 `futures` 差不多的模块。

此外，虽然目前 [`Stream`][stream] 是由 `futures` 提供的，但未来如果编译器要实现[异步生成器][generator]（async generator），这个 crate 也很可能会进入标准库，因而对其的扩展也依然独立放进了的 `StreamExt` 里。


[future]: https://doc.rust-lang.org/std/future/trait.Future.html
[context]: https://doc.rust-lang.org/std/task/struct.Context.html
[pin]: https://doc.rust-lang.org/std/pin/struct.Pin.html
[waker]: https://doc.rust-lang.org/std/task/struct.Waker.html

[futures]: https://crates.io/crates/futures
[future-ext]: https://docs.rs/futures/*/futures/future/trait.FutureExt.html
[stream]: https://docs.rs/futures/*/futures/stream/trait.Stream.html
[stream-ext]: https://docs.rs/futures/*/futures/stream/trait.StreamExt.html
[channel]: https://docs.rs/futures/*/futures/channel/index.html
[executor]: https://docs.rs/futures/*/futures/executor/index.html

[tokio]: https://crates.io/crates/tokio
[async-std]: https://crates.io/crates/async-std

[generator]: https://rust-lang.github.io/rfcs/2394-async_await.html#generators-and-streams
