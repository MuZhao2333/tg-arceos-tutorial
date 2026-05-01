# Report: exercise-printcolor

## 任务

使用 ANSI 转义序列在 ArceOS 中输出彩色文本。

## 修改内容

### `src/main.rs`

在 `main` 函数中通过 `println!` 打印带有 ANSI 转义序列的彩色字符串：

```rust
println!("[WithColor]: \x1b[1;32mHello, Arceos!\x1b[0m");
```

- `\x1b[1;32m`：设置文本为**粗体**（1）和**绿色**（32）
- `\x1b[0m`：重置文本样式
