# Report: exercise-hashmap

## 任务

在 `axstd` 中启用 `collections::HashMap`，使其能在 ArceOS 的 `no_std` 环境下正常工作。

## 修改内容

### `axstd/src/lib.rs`

新增了 `collections` 模块，将 `hashbrown` crate 中的 `HashMap` 和 `HashSet` re-export 出来：

```rust
#[cfg(feature = "alloc")]
pub mod collections {
    pub use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
    pub use hashbrown::{HashMap, HashSet};
}

#[cfg(feature = "alloc")]
#[doc(no_inline)]
pub use collections::{HashMap, HashSet};
```

由于 `axstd` 默认使用 `no_std` 环境，无法直接依赖 Rust 标准库的 `std::collections`，因此通过 `hashbrown` crate（纯 Rust 实现的 HashMap，不依赖 std）来提供 `HashMap` 支持。

### `axstd/Cargo.toml`

添加了 `hashbrown` 依赖：

```toml
hashbrown = { version = "0.15", features = ["raw"] }
```

### `Cargo.toml`

将 `axstd` 从 crates.io 版本切换为本地路径，以便本地修改生效：

```toml
[dependencies]
axstd = { path = "./axstd", features = ["defplat", "alloc"], optional = true }
```
