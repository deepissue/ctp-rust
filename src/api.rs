//! 高级API模块
//!
//! 提供安全、易用的Rust API接口

pub mod async_md_api;
pub mod async_trader_api;
pub mod md_api;
pub mod trader_api;
pub mod utils;

pub use async_md_api::AsyncMdApi;
pub use async_trader_api::AsyncTraderApi;
pub use md_api::{MdApi, MdSpiHandler};
pub use trader_api::{TraderApi, TraderSpiHandler};

use crate::error::{CtpError, CtpResult};
use std::ffi::CString;

// CTP API基础功能特质
pub trait CtpApi {
    // 获取API版本
    fn get_version() -> CtpResult<String>;

    // 初始化API
    fn init(&mut self) -> CtpResult<()>;

    // 释放API资源
    fn release(&mut self);

    // 获取当前交易日
    fn get_trading_day(&self) -> CtpResult<String>;

    // 注册前置机地址
    fn register_front(&mut self, front_address: &str) -> CtpResult<()>;

    // 等待API线程结束
    fn join(&self) -> CtpResult<i32>;
}

// 将Rust字符串转换为C字符串
pub(crate) fn to_cstring(s: &str) -> CtpResult<CString> {
    CString::new(s).map_err(|e| CtpError::InvalidParameterError(format!("字符串转换失败: {}", e)))
}

// 检查指针是否为空
#[allow(dead_code)]
pub(crate) fn check_null_ptr<T>(ptr: *const T, name: &str) -> CtpResult<()> {
    if ptr.is_null() {
        return Err(CtpError::FfiError(format!("{} 指针为空", name)));
    }
    Ok(())
}

// 安全地从C字符串获取UTF-8字符串
pub(crate) fn safe_cstr_to_string(ptr: *const i8) -> CtpResult<String> {
    if ptr.is_null() {
        return Ok(String::new());
    }

    unsafe { crate::encoding::GbkConverter::cstring_to_utf8(ptr) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_cstring() {
        let result = to_cstring("test");
        assert!(result.is_ok());

        let result = to_cstring("测试");
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_null_ptr() {
        let ptr: *const i32 = std::ptr::null();
        assert!(check_null_ptr(ptr, "test").is_err());

        let value = 42i32;
        let ptr = &value as *const i32;
        assert!(check_null_ptr(ptr, "test").is_ok());
    }
}
