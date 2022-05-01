# 如何同时等待多个 `Future`？

如果想要等待多个 `Future` 都完成后返回，对于固定数量的 `Future` 可以使用 [futures] 所提供的 [`join`][join]、[`join3`][join3]、[`join4`][join4] 等函数，或者 [tokio] 所提供的 [`join!`][join-macro] 宏，将多个 `Future` 合并为一个进行等待。对于不定数量的 `Future`，比如有一个 `Vec`，则可以使用 futures 的 [`join_all`][join_all] 函数。

若要在数个 `Future` 中第一个错误发生时就返回，则可以使用它们对应的 [`try_join`][try_join]、[`try_join3`][try_join3]、[`try_join4`][try_join4]、[`try_join_all`][try_join_all] 等函数以及 [`try_join!`][try_join-macro] 宏。

如果想要在多个 `Future` 中的第一个完成后就返回，可以使用 futures 的 [`select`][select]、[`select_all`][select_all] 和 [`select_ok`][select_ok] 函数或 tokio 的 [`select!`][select-macro] 宏。

需要注意的是，上面这些函数和宏在每次被轮询的时候，无法插入新的 `Future`——使用 futures 提供的 [`FuturesOrdered`][FuturesOrdered] 和 [`FuturesUnordered`][FuturesUnordered] 这两个结构可以解决这一问题。它们将这些 `Future` 聚合成一个 [`Stream`][Stream] 逐个返回里面 `Future` 的结果，其中前者会按照输入的 `Future` 的顺序返回，而后者则是以任意顺序（可以近似看作按照完成顺序）返回。同时，也可以调用 `push` 方法来插入一个 Future 进行轮询。


[futures]: https://crates.io/crates/futures
[join]: https://docs.rs/futures/0.3/futures/future/fn.join.html
[join3]: https://docs.rs/futures/0.3/futures/future/fn.join3.html
[join4]: https://docs.rs/futures/0.3/futures/future/fn.join4.html
[join_all]: https://docs.rs/futures/0.3/futures/future/fn.join_all.html
[try_join]: https://docs.rs/futures/0.3/futures/future/fn.try_join.html
[try_join3]: https://docs.rs/futures/0.3/futures/future/fn.try_join3.html
[try_join4]: https://docs.rs/futures/0.3/futures/future/fn.try_join4.html
[try_join_all]: https://docs.rs/futures/0.3/futures/future/fn.try_join_all.html
[select]: https://docs.rs/futures/0.3/futures/future/fn.select.html
[select_all]: https://docs.rs/futures/0.3/futures/future/fn.select_all.html
[select_ok]: https://docs.rs/futures/0.3/futures/future/fn.select_ok.html
[FuturesOrdered]: https://docs.rs/futures/0.3/futures/stream/struct.FuturesOrdered.html
[FuturesUnordered]: https://docs.rs/futures/0.3/futures/stream/struct.FuturesUnordered.html
[Stream]: https://docs.rs/futures/0.3/futures/stream/trait.Stream.html

[tokio]: https://crates.io/crates/tokio
[join-macro]: https://docs.rs/tokio/1/tokio/macro.join.html
[try_join-macro]: https://docs.rs/tokio/1/tokio/macro.try_join.html
[select-macro]: https://docs.rs/tokio/1/tokio/macro.select.html

