# 如何同时等待多个 `Future`？

对于固定数量的 `Future`，可以使用 [`futures`][futures] 所提供的 [`join`][join]、[`join3`][join3]、[`join4`][join4] 等函数，或者 [`tokio`][tokio] 所提供的 [`join!`][join-macro] 宏，将多个 `Future` 合并为一个进行等待。对于不定数量的 `Future`，比如有一个 `Vec`，则可以使用 `futures` 的 [`join_all`][join_all] 函数。

如果想要在数个 `Future` 中第一个错误发生时就返回，也可以使用它们对应的 [`try_join`][try_join]、[`try_join3`][try_join3]、[`try_join4`][try_join4]、[`try_join_all`][try_join_all] 等函数以及 [`try_join!`][try_join-macro] 宏。

需要注意的是，上面这些函数和宏在每次被轮询的时候，都会轮询下面每一个未完成的 `Future`，因此当其所带的 `Future` 很多时可能会有性能问题。使用 `futures` 提供的 [`FuturesOrdered`][FuturesOrdered] 和 [`FuturesUnordered`][FuturesUnordered] 这两个结构可以解决这一问题。它们是为管理大量的 `Future` 而设计的，只会去轮询收到了唤醒通知的 `Future`。


[futures]: https://crates.io/crates/futures
[join]: https://docs.rs/futures/0.3/futures/future/fn.join.html
[join3]: https://docs.rs/futures/0.3/futures/future/fn.join3.html
[join4]: https://docs.rs/futures/0.3/futures/future/fn.join4.html
[join_all]: https://docs.rs/futures/0.3/futures/future/fn.join_all.html
[try_join]: https://docs.rs/futures/0.3/futures/future/fn.try_join.html
[try_join3]: https://docs.rs/futures/0.3/futures/future/fn.try_join3.html
[try_join4]: https://docs.rs/futures/0.3/futures/future/fn.try_join4.html
[try_join_all]: https://docs.rs/futures/0.3/futures/future/fn.try_join_all.html
[tokio]: https://crates.io/crates/tokio
[join-macro]: https://docs.rs/tokio/1/tokio/macro.join.html
[try_join-macro]: https://docs.rs/tokio/1/tokio/macro.try_join.html
[FuturesOrdered]: https://docs.rs/futures/0.3/futures/stream/struct.FuturesOrdered.html
[FuturesUnordered]: https://docs.rs/futures/0.3/futures/stream/struct.FuturesUnordered.html

