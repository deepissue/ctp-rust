# CTP Rust SDK Makefile
# 提供便捷的编译和管理命令

.PHONY: wrapper build test examples clean clean-all install help

# 默认目标
all: wrapper build

# 编译C++包装器
wrapper:
	@echo "🔨 编译CTP C++包装器..."
	@if [ "$(shell uname)" = "Darwin" ]; then \
		cd libs/ctp/mac64/wrapper && $(MAKE); \
	else \
		cd libs/ctp/linux/wrapper && $(MAKE); \
	fi
	@echo "✅ C++包装器编译完成"

# 编译Rust项目
build: wrapper
	@echo "🦀 编译Rust项目..."
	cargo build
	@echo "✅ Rust项目编译完成"

# 编译发布版本
release: wrapper
	@echo "🚀 编译发布版本..."
	cargo build --release
	@echo "✅ 发布版本编译完成"

# 运行测试
test: build
	@echo "🧪 运行测试..."
	cargo test
	@echo "✅ 测试完成"

# 编译示例
examples: build
	@echo "📚 编译示例程序..."
	cargo build --examples
	@echo "✅ 示例程序编译完成"

# 运行特定示例
run-md-basic: examples
	@echo "🔄 运行行情示例..."
	cargo run --example md_basic

run-trader-basic: examples
	@echo "💹 运行交易示例..."
	cargo run --example trader_basic

run-async-md: examples
	@echo "⚡ 运行异步行情示例..."
	cargo run --example async_md_basic

run-encoding-demo: examples
	@echo "🔤 运行编码演示..."
	cargo run --example encoding_demo

run-error-handling: examples
	@echo "🔧 运行错误处理示例..."
	cargo run --example error_handling

# 清理编译产物
clean:
	@echo "🧹 清理Rust编译产物..."
	cargo clean
	@echo "✅ Rust清理完成"

# 清理C++包装器
clean-wrapper:
	@echo "🧹 清理C++包装器..."
	@if [ "$(shell uname)" = "Darwin" ]; then \
		cd libs/ctp/mac64/wrapper && $(MAKE) clean; \
	else \
		cd libs/ctp/linux/wrapper && $(MAKE) clean; \
	fi
	@echo "✅ C++包装器清理完成"

# 完全清理
clean-all: clean clean-wrapper
	@echo "🧹 清理临时文件..."
	@find . -name "*.con" -delete 2>/dev/null || true
	@find . -name "error.*" -delete 2>/dev/null || true
	@echo "✅ 完全清理完成"

# 代码格式化
fmt:
	@echo "✨ 格式化代码..."
	cargo fmt
	@echo "✅ 代码格式化完成"

# 代码检查
clippy: build
	@echo "🔍 运行Clippy检查..."
	cargo clippy -- -D warnings
	@echo "✅ Clippy检查完成"

# 文档生成
doc:
	@echo "📖 生成文档..."
	cargo doc --no-deps --open
	@echo "✅ 文档生成完成"

# 安装C++包装器到系统
install-wrapper: wrapper
	@echo "📦 安装C++包装器..."
	@if [ "$(shell uname)" = "Darwin" ]; then \
		cd libs/ctp/mac64/wrapper && $(MAKE) install; \
	else \
		cd libs/ctp/linux/wrapper && $(MAKE) install; \
	fi
	@echo "✅ C++包装器安装完成"

# 检查依赖更新
update:
	@echo "🔄 检查依赖更新..."
	cargo update
	@echo "✅ 依赖更新完成"

# 基准测试
bench: build
	@echo "⚡ 运行基准测试..."
	cargo bench
	@echo "✅ 基准测试完成"

# 发布准备
prepare-release: clean-all
	@echo "🚀 准备发布..."
	$(MAKE) wrapper
	$(MAKE) build
	$(MAKE) test
	$(MAKE) clippy
	$(MAKE) doc
	@echo "✅ 发布准备完成"

# 帮助信息
help:
	@echo "CTP Rust SDK 编译帮助"
	@echo "====================="
	@echo ""
	@echo "🔨 编译命令:"
	@echo "  make              - 编译包装器和Rust项目 (默认)"
	@echo "  make wrapper      - 只编译C++包装器"
	@echo "  make build        - 编译Rust项目"
	@echo "  make release      - 编译发布版本"
	@echo "  make examples     - 编译示例程序"
	@echo ""
	@echo "🧪 测试命令:"
	@echo "  make test         - 运行测试"
	@echo "  make clippy       - 运行Clippy检查"
	@echo "  make bench        - 运行基准测试"
	@echo ""
	@echo "📚 示例运行:"
	@echo "  make run-md-basic      - 运行行情示例"
	@echo "  make run-trader-basic  - 运行交易示例"
	@echo "  make run-async-md      - 运行异步行情示例"
	@echo "  make run-encoding-demo - 运行编码演示"
	@echo "  make run-error-handling- 运行错误处理示例"
	@echo ""
	@echo "🧹 清理命令:"
	@echo "  make clean        - 清理Rust编译产物"
	@echo "  make clean-wrapper- 清理C++包装器"
	@echo "  make clean-all    - 完全清理"
	@echo ""
	@echo "📖 工具命令:"
	@echo "  make fmt          - 格式化代码"
	@echo "  make doc          - 生成文档"
	@echo "  make update       - 更新依赖"
	@echo "  make install-wrapper - 安装C++包装器到系统"
	@echo "  make prepare-release - 准备发布"
	@echo "  make help         - 显示此帮助"
