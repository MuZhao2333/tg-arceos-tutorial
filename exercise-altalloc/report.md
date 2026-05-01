# Report: exercise-altalloc

## 任务

实现一个基于 bump（递增）算法的早期内存分配器 `bump_allocator`，使其同时作为字节分配器和页分配器使用。

## 修改内容

### `modules/bump_allocator/src/lib.rs`

实现了完整的 `EarlyAllocator` 结构体，同时实现了三个 trait：

**数据结构设计：**

- 将可用内存区间 `[start, end]` 从两端使用：
  - 字节分配从前端（`b_pos`）递增
  - 页分配从后端（`p_pos`）递减
  - 中间为可用区域

**实现的 trait：**

1. `BaseAllocator`：初始化和添加内存区域
2. `ByteAllocator`：字节级分配，`alloc()` 将 `b_pos` 按对齐要求向上取整后递增；`dealloc()` 递减计数，仅当计数归零时重置 `b_pos`
3. `PageAllocator`：页级分配，`alloc_pages_at()` 将 `p_pos` 按对齐要求向下取整后递减；`dealloc_pages()` 不做任何操作（bump 特性）

### `modules/axalloc/src/lib.rs`

启用了 `bump` feature，将默认分配器切换为 `bump_allocator`：

```toml
allocator = "bump"
```
