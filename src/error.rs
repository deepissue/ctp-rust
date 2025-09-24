//! 错误处理模块
//!
//! 定义了CTP SDK中使用的错误类型和结果类型

use std::fmt;

/// CTP错误类型
#[derive(Debug, Clone)]
pub enum CtpError {
    /// FFI调用错误
    FfiError(String),
    /// 编码转换错误
    EncodingError(String),
    /// 网络连接错误
    ConnectionError(String),
    /// 登录认证错误
    AuthenticationError(String),
    /// 业务逻辑错误
    BusinessError(i32, String),
    /// 初始化错误
    InitializationError(String),
    /// 超时错误
    TimeoutError(String),
    /// 无效参数错误
    InvalidParameterError(String),
    /// 内存错误
    MemoryError(String),
    /// 其他错误
    Other(String),
}

impl fmt::Display for CtpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CtpError::FfiError(msg) => write!(f, "FFI错误: {}", msg),
            CtpError::EncodingError(msg) => write!(f, "编码转换错误: {}", msg),
            CtpError::ConnectionError(msg) => write!(f, "网络连接错误: {}", msg),
            CtpError::AuthenticationError(msg) => write!(f, "登录认证错误: {}", msg),
            CtpError::BusinessError(code, msg) => write!(f, "业务错误 [{}]: {}", code, msg),
            CtpError::InitializationError(msg) => write!(f, "初始化错误: {}", msg),
            CtpError::TimeoutError(msg) => write!(f, "超时错误: {}", msg),
            CtpError::InvalidParameterError(msg) => write!(f, "无效参数错误: {}", msg),
            CtpError::MemoryError(msg) => write!(f, "内存错误: {}", msg),
            CtpError::Other(msg) => write!(f, "其他错误: {}", msg),
        }
    }
}

impl std::error::Error for CtpError {}

/// CTP结果类型
pub type CtpResult<T> = Result<T, CtpError>;

/// 将C字符串错误转换为CtpError
pub fn c_string_error(msg: &str) -> CtpError {
    CtpError::FfiError(format!("C字符串处理错误: {}", msg))
}

/// 将编码错误转换为CtpError
pub fn encoding_error(msg: &str) -> CtpError {
    CtpError::EncodingError(msg.to_string())
}

/// 将连接错误转换为CtpError
pub fn connection_error(msg: &str) -> CtpError {
    CtpError::ConnectionError(msg.to_string())
}

/// 将业务错误转换为CtpError
pub fn business_error(error_id: i32, error_msg: &str) -> CtpError {
    CtpError::BusinessError(error_id, error_msg.to_string())
}
