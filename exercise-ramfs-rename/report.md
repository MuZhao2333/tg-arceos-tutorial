# Report: exercise-ramfs-rename

## 任务

在 ramfs 内存文件系统中实现 `rename` 操作，使 `std::fs::rename` 能够正常工作。

## 修改内容

### `axfs_ramfs/src/dir.rs`

#### 1. `DirNode::rename_node()`（第 74-91 行）

在同一目录内重命名子节点：
- 检查源名称是否存在
- 检查目标名称是否已存在
- 从 `BTreeMap` 中移除旧条目并插入新条目

#### 2. `impl VfsNodeOps for DirNode` — `rename()`（第 193-217 行）

实现了 `VfsNodeOps::rename` trait 方法，支持跨目录递归重命名：
- 解析源路径和目标路径（提取目录名和剩余路径）
- 若两者均在同一目录（均无剩余路径），调用 `rename_node()` 直接重命名
- 若涉及子目录，则递归调用子目录的 `rename()`

### `axfs/src/root.rs`

#### 1. `RootDirectory::rename()`（第 171-187 行）

在根目录层面实现了 `VfsOps::rename` trait 方法，处理挂载点逻辑：
- 将旧路径和新路径规范化
- 查找两者所属的挂载文件系统
- 若在同一挂载点内，委托给对应文件系统的 `rename()`
- 若跨文件系统，则先委托给主文件系统的根目录

#### 2. `rename(old, new)`（第 599-607 行）

顶层 API 函数：
- 若目标文件已存在，先删除目标文件
- 获取源文件所在目录节点
- 调用目录的 `rename()` 执行重命名

### `Cargo.toml`

将 `axfs` 和 `axfs_ramfs` 从 crates.io 切换为本地路径：

```toml
[patch.crates-io]
axfs = { path = "./axfs" }
axfs_ramfs = { path = "./axfs_ramfs" }
```