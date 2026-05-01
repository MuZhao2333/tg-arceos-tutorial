# Report: exercise-sysmap

## 任务

在用户态程序 syscall 仿真层中实现 `SYS_MMAP` 系统调用，使 musl 程序能够使用 `mmap(2)` 进行文件映射。

## 修改内容

### `src/syscall.rs`

#### 1. 定义 mmap 相关常量与标志位

新增了 `MmapProt` 和 `MmapFlags` 两个 bitflags，分别对应 Linux 的 `prot` 和 `flags` 参数：

```rust
struct MmapProt: i32 {
    const PROT_READ = 1 << 0;
    const PROT_WRITE = 1 << 1;
    const PROT_EXEC = 1 << 2;
}

struct MmapFlags: i32 {
    const MAP_SHARED = 1 << 0;
    const MAP_PRIVATE = 1 << 1;
    const MAP_FIXED = 1 << 4;
    const MAP_ANONYMOUS = 1 << 5;
    const MAP_NORESERVE = 1 << 14;
    const MAP_STACK = 0x20000;
}
```

并实现了 `From<MmapProt> for MappingFlags` 将 Linux 的保护标志转换为 ArceOS 的页面映射标志。

#### 2. 匿名映射（`MAP_ANONYMOUS`）

当 `flags` 包含 `MAP_ANONYMOUS` 时：
- 通过 `allocate_vaddr()` 从用户地址空间顶端向下分配虚拟地址
- 遍历分配所有页面，映射到物理页
- 返回分配的虚拟地址

#### 3. 文件映射（非匿名）

当需要将文件映射到内存时：
- 同样通过 `allocate_vaddr()` 分配虚拟地址（或使用 `MAP_FIXED` 指定的地址）
- 将虚拟页面映射到用户空间
- 读取文件内容到缓冲区
- 将数据写入用户页面

#### 4. 辅助函数

- `allocate_vaddr(num_pages)`：从 `MMAP_BASE`（初始 `0x3_0000_0000`）向下增长分配虚拟地址
- `read_file_at(fd, offset, max_len)`：从指定 fd 和偏移处读取文件内容
