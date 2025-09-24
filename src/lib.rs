//! # CTP Rust SDK
//!
//! 这是一个为CTP (综合交易平台) 提供的安全Rust绑定库，支持Linux和macOS系统。
//!
//! ## 功能特点
//!
//! - 🔒 类型安全的Rust绑定
//! - 🌍 支持Linux和macOS平台
//! - 📝 自动处理GB18030到UTF-8编码转换
//! - ⚡ 异步支持
//! - 📚 完整的中文文档
//!
//! ## 模块结构
//!
//! - `ffi` - C++库的FFI绑定
//! - `encoding` - 编码转换工具
//! - `api` - 高级API接口
//! - `error` - 错误处理
//! - `types` - 类型定义

pub mod api;
pub mod config;
pub mod encoding;
pub mod error;
pub mod ffi;
pub mod types;
// 重新导出主要类型和函数
pub use api::{AsyncMdApi, MdApi, TraderApi};
pub use config::CtpConfig;
pub use error::{CtpError, CtpResult};
pub use types::*;
/// 库版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
