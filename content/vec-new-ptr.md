# 新创建的空 `Vec<T>` 的指针为何指向`1`、`2`、`4`、`8`等地址？

当 `Vec` 的容量为0时，没有合法的操作会向其指针指向的位置进行读取和写入，进行任何读写之前都必然会有一次内存分配，因此这个初始的指针并不需要是一个有效的指针。这也使得创建 `Vec` 本身没有进行实际内存分配的必要，既省去了内存分配的开销，也让创建容器的操作可以在常量上下文中使用。

而因为 `Vec` 需要能被作为[切片]使用，由于切片对数据指针的[要求][slice-safety]，它的指针的地址需要是非空并且正确对齐的，因而简单起见便选择了类型的对齐的大小作为这个无效指针指向的地址。


[slice-safety]: https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html#safety