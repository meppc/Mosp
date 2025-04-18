# 开发环境配置指南

## 工作目录设置

开发 Mosp 项目时，请确保将工作目录设置为 `~/repos/Mosp`。你可以通过以下方式设置：

```bash
# 在 ~/.bashrc 或 ~/.zshrc 中添加以下别名
alias cdmosp="cd ~/repos/Mosp"
```

## 常用命令别名

为了提高开发效率，建议设置以下别名：

```bash
# 在 ~/.bashrc 或 ~/.zshrc 中添加
alias mb="cargo build"            # 构建项目
alias mr="cargo run"              # 运行项目
alias mt="cargo test"             # 运行测试
alias mf="cargo fmt"              # 格式化代码
alias mc="cargo check"            # 检查代码
alias mw="cargo watch -x run"     # 监视模式运行
```

## 本地开发环境要求

请确保安装以下依赖和工具：

1. Rust 1.75 或更高版本
2. Cargo 包管理器
3. CMake (用于构建 Slint)
4. C++ 编译器

### 安装验证

运行以下命令验证环境配置：

```bash
rustc --version
cargo --version
cmake --version
```

## 常见错误排查

1. 构建失败
   - 检查 Rust 工具链版本是否满足要求
   - 确保所有依赖都已正确安装
   - 尝试清理并重新构建：`cargo clean && cargo build`

2. Slint 相关错误
   - 确保 CMake 已正确安装
   - 检查 C++ 编译器是否可用
   - 参考 [Slint 文档](https://slint.dev/docs/installation) 进行故障排除

3. 依赖冲突
   - 尝试更新依赖：`cargo update`
   - 检查 Cargo.toml 中的版本约束

## 更多资源

- [项目文档](../)
- [架构概览](../architecture-overview.md)
- [技术栈说明](../tech-stack.md)

如遇到其他问题，请查看项目 Issue 或在社区中寻求帮助。