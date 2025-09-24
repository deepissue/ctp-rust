# CTP Rust SDK

[![Crates.io](https://img.shields.io/crates/v/ctp-rust.svg)](https://crates.io/crates/ctp-rust)
[![Documentation](https://docs.rs/ctp-rust/badge.svg)](https://docs.rs/ctp-rust)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/deepissue/ctp-rust)

🚀 **安全、高效、易用的CTP (综合交易平台) Rust绑定库**

为中国金融市场提供完整的期货交易功能，支持同步和异步API，具备类型安全、内存安全和并发安全的特性。

## ✨ 主要特性

- 🔒 **类型安全**: 完整的Rust类型系统支持，编译时错误检查
- 🌍 **跨平台支持**: Linux (x86_64) 和 macOS (x86_64/Apple Silicon) 
- ⚡ **异步支持**: 提供async/await和tokio集成的异步API
- 📝 **编码处理**: 自动GB18030到UTF-8编码转换
- 🧵 **线程安全**: 内置线程安全保护和错误处理
- 🐛 **调试支持**: 集成Debug日志系统，支持文件和控制台输出
- 📚 **完整文档**: 详细的中文文档和使用示例

## 🏗️ 系统要求

### Linux
- Ubuntu 18.04+ / CentOS 7+ / Debian 9+
- GCC 7+ 或 Clang 6+
- glibc 2.17+

### macOS
- macOS 11.0+ (Big Sur)
- Xcode Command Line Tools
- 支持Intel和Apple Silicon架构

## 📦 快速开始

### 1. 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
ctp-rust = "1.0.1"
tokio = { version = "1.42", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
dotenvy = "0.15"
```

### 2. CTP SDK配置

#### 自动下载（推荐）
SDK将在构建时自动配置，无需手动下载。

#### 手动配置
如需手动配置CTP SDK，请将库文件放置在以下目录：

```
libs/ctp/
├── linux/
│   ├── include/          # CTP头文件
│   │   ├── ThostFtdcTraderApi.h
│   │   ├── ThostFtdcMdApi.h
│   │   └── ...
│   └── lib/             # CTP动态库
│       ├── libthosttraderapi_se.so
│       └── libthostmduserapi_se.so
└── mac64/
    ├── include/          # CTP头文件
    └── lib/             # CTP动态库
        ├── libthosttraderapi_se.dylib
        └── libthostmduserapi_se.dylib
```

### 3. 环境变量配置

创建 `.env` 文件：

```env
# CTP服务器配置
CTP_TRADER_FRONT=tcp://180.168.146.187:10201
CTP_MD_FRONT=tcp://180.168.146.187:10211

# 账户配置  
CTP_BROKER_ID=9999
CTP_INVESTOR_ID=your_account
CTP_PASSWORD=your_password

# 可选配置
CTP_APP_ID=simnow_client_test
CTP_AUTH_CODE=0000000000000000
CTP_FLOW_PATH=./flow/
```

### 4. 基础使用示例

#### 同步交易API
```rust
use ctp_rust::{CtpConfig, CtpResult, TraderApi};
use ctp_rust::types::*;
use std::sync::{Arc, Mutex};

fn main() -> CtpResult<()> {
    // 加载配置
    let config = CtpConfig::from_env()?;
    
    // 创建交易API
    let trader_api = TraderApi::new(Some(&config.flow_path), Some(true))?;
    let api_arc = Arc::new(Mutex::new(trader_api));
    
    // 注册事件处理器
    let handler = MyTraderHandler::new();
    api_arc.lock().unwrap().register_spi(handler)?;
    
    // 连接和初始化
    api_arc.lock().unwrap().register_front(&config.trader_front_address)?;
    api_arc.lock().unwrap().init()?;
    
    // 等待连接...
    std::thread::sleep(std::time::Duration::from_secs(30));
    
    Ok(())
}
```

#### 异步交易API
```rust
use ctp_rust::api::AsyncTraderApi;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CtpConfig::from_env()?;
    
    // 创建异步交易API
    let mut async_trader = AsyncTraderApi::new(Some(&config.flow_path))?;
    
    // 连接服务器
    async_trader.connect(&config.trader_front_address).await?;
    
    // 用户登录
    let login_req = ReqUserLoginField::new(
        &config.broker_id,
        &config.investor_id, 
        &config.password
    )?;
    
    async_trader.login(login_req).await?;
    
    // 查询账户信息
    let account_query = QryTradingAccountField::new(&config.broker_id, &config.investor_id)?;
    let account_info = async_trader.query_trading_account(account_query).await?;
    
    println!("可用资金: {:.2}", account_info.available);
    
    Ok(())
}
```

## 📋 完整示例

运行内置示例：

```bash
# 基础交易示例
cargo run --example trader_basic

# 异步交易示例  
cargo run --example async_trader_basic

# 行情订阅示例
cargo run --example md_basic

# 异步行情示例
cargo run --example async_md_basic

# 编码处理示例
cargo run --example encoding_demo

# 错误处理示例
cargo run --example error_handling
```

## 🔧 调试配置

### 启用Debug日志

```rust
use ctp_rust::config::DebugConfig;

// 创建debug配置
let debug_config = DebugConfig {
    enable_debug: true,
    log_file_path: Some("./debug.log".to_string()),
    max_file_size_mb: 100,
    max_backup_files: 5,
};

// 应用配置
debug_config.apply();
```

### 环境变量调试

```bash
# 设置日志级别
export RUST_LOG=debug

# Linux库路径
export LD_LIBRARY_PATH=./libs/ctp/linux/lib:$LD_LIBRARY_PATH

# macOS库路径  
export DYLD_LIBRARY_PATH=./libs/ctp/mac64/lib:$DYLD_LIBRARY_PATH

# 运行示例
cargo run --example trader_basic
```

## 🔄 手动更新SDK方法

当CTP发布新版本时，可以手动更新SDK：

### 1. 下载新版本SDK

从上期技术官网下载最新的CTP SDK：
- Linux: `v6.7.0_20220613_api_traderapi_se_linux64.tar.gz`
- macOS: `v6.7.0_20220613_api_traderapi_se_macos.tar.gz`

### 2. 替换库文件

```bash
# Linux
cp new_sdk/libthosttraderapi_se.so libs/ctp/linux/lib/
cp new_sdk/libthostmduserapi_se.so libs/ctp/linux/lib/
cp new_sdk/*.h libs/ctp/linux/include/

# macOS  
cp new_sdk/libthosttraderapi_se.dylib libs/ctp/mac64/lib/
cp new_sdk/libthostmduserapi_se.dylib libs/ctp/mac64/lib/
cp new_sdk/*.h libs/ctp/mac64/include/
```

### 3. 更新API绑定

如果新版本添加了新的API方法：

```rust
// 1. 在 src/ffi.rs 中添加FFI声明
extern "C" {
    pub fn CThostFtdcTraderApi_ReqNewMethod(
        api: *mut c_void,
        req: *const NewMethodField,
        request_id: c_int,
    ) -> c_int;
}

// 2. 在 src/api/trader_api.rs 中添加Rust包装
impl TraderApi {
    pub fn req_new_method(&mut self, req: &NewMethodField) -> CtpResult<i32> {
        let request_id = self.get_next_request_id();
        let result = unsafe {
            ffi::CThostFtdcTraderApi_ReqNewMethod(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };
        self.handle_api_result(result, request_id)
    }
}

// 3. 在 src/types.rs 中添加新的类型定义
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct NewMethodField {
    pub field1: [c_char; 32],
    pub field2: i32,
    // ... 其他字段
}
```

### 4. 重新编译和测试

```bash
# 清理并重新编译
cargo clean
cargo build

# 运行测试
cargo test

# 测试示例
cargo run --example trader_basic
```

## 🏛️ API架构

### 核心模块

- **`api`** - 高级API接口
  - `TraderApi` - 同步交易API
  - `MdApi` - 同步行情API  
  - `AsyncTraderApi` - 异步交易API
  - `AsyncMdApi` - 异步行情API

- **`types`** - CTP数据类型定义
  - 登录请求/响应类型
  - 查询请求/响应类型
  - 订单和交易类型
  - 行情数据类型

- **`ffi`** - 底层C++绑定
  - 原生CTP API的Rust FFI声明
  - 内存安全的指针操作

- **`encoding`** - 编码转换
  - GB18030 ↔ UTF-8 自动转换
  - 字符串处理工具

- **`error`** - 错误处理
  - 统一的错误类型定义
  - 详细的错误信息

### 异步架构

异步API基于tokio运行时，使用以下模式：
- **事件驱动**: 使用mpsc channels处理回调
- **Future-based**: 所有API调用返回Future
- **超时控制**: 内置请求超时机制
- **线程安全**: 跨线程安全的状态管理

## 🔍 故障排查

### 常见问题

#### 1. 连接失败
```
错误: 连接超时失败
```

**解决方案:**
- 检查网络连接: `ping 180.168.146.187`  
- 验证服务器地址和端口
- 确认防火墙设置
- 检查是否在交易时间段

#### 2. 动态库加载失败
```
错误: cannot open shared object file
```

**解决方案:**
```bash
# Linux
export LD_LIBRARY_PATH=./libs/ctp/linux/lib:$LD_LIBRARY_PATH

# macOS
export DYLD_LIBRARY_PATH=./libs/ctp/mac64/lib:$DYLD_LIBRARY_PATH
```

#### 3. 登录失败
```
错误: 投资者账号错误或密码错误  
```

**解决方案:**
- 验证环境变量配置
- 检查账号是否激活
- 确认密码是否正确
- 检查是否超过登录限制

#### 4. 编译错误
```
错误: linker `cc` not found
```

**解决方案:**
```bash
# Ubuntu/Debian
sudo apt install build-essential

# CentOS/RHEL  
sudo yum groupinstall "Development Tools"

# macOS
xcode-select --install
```

### 调试技巧

#### 启用详细日志
```bash
export RUST_LOG=ctp_rust=debug,tokio=info
export RUST_BACKTRACE=1
```

#### 网络诊断
```bash
# 测试连接
telnet 180.168.146.187 10201

# 检查DNS解析
nslookup 180.168.146.187
```

## 🤝 开发贡献

### 环境准备

```bash
# 克隆项目
git clone https://github.com/deepissue/ctp-rust.git
cd ctp-rust

# 安装依赖
cargo build

# 运行测试
cargo test

# 检查代码格式
cargo fmt
cargo clippy
```

### 提交规范

- 遵循 [Conventional Commits](https://conventionalcommits.org/) 规范
- 提供详细的测试用例
- 更新相关文档

## 📄 许可证

本项目采用 MIT/Apache-2.0 双许可证。

- MIT License: [LICENSE-MIT](LICENSE-MIT)
- Apache License 2.0: [LICENSE-APACHE](LICENSE-APACHE)

## 🔗 相关链接

- [CTP官方文档](http://www.sfit.com.cn/5_2_DocumentDown.htm)
- [SimNow模拟交易](http://www.simnow.com.cn/)
- [Rust官方文档](https://doc.rust-lang.org/)
- [Tokio异步运行时](https://tokio.rs/)

## 📞 技术支持

- 🐛 [Issues](https://github.com/deepissue/ctp-rust/issues) - 问题报告和功能请求
- 💬 [Discussions](https://github.com/deepissue/ctp-rust/discussions) - 讨论和提问
- 📧 Email: 36625090@qq.com

---

**⚠️ 风险提示**: 期货交易具有高风险，可能导致投资损失。请在充分理解风险的基础上进行交易，本SDK仅供技术学习和测试使用。