# 为什么 Rust 生成的程序体积比较大？如何最小化程序体积？

有多个因素使得 Rust 在默认情况下有着相对较大的程序体积，包括了[单态化]、调试符号、标准库等。一般来说，Rust 偏向于为性能优化而非更小的体积。

通常使用发布模式编译（`--release`），以及（在 Linux 和 macOS 下）使用 `strip` 删除符号信息可以在一定程度上缩小程序体积。更多方法可以参考 [Minimizing Rust Binary Size](https://github.com/johnthagen/min-sized-rust)，对这一问题有较完整的介绍。
