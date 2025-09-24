//! 编码转换模块
//!
//! 处理GB18030和UTF-8之间的编码转换

use crate::error::{encoding_error, CtpResult};
use encoding::all::GB18030;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use std::ffi::{CStr, CString};

// GB18030编码转换器
pub struct GbkConverter;

impl GbkConverter {
    // 将GB18030编码的字节转换为UTF-8字符串
    //
    // # 参数
    // * `bytes` - GB18030编码的字节数组
    //
    // # 返回值
    // UTF-8字符串
    pub fn gb18030_to_utf8(bytes: &[u8]) -> CtpResult<String> {
        // 移除末尾的空字节
        let trimmed_bytes = Self::trim_null_bytes(bytes);

        if trimmed_bytes.is_empty() {
            return Ok(String::new());
        }

        GB18030
            .decode(trimmed_bytes, DecoderTrap::Replace)
            .map_err(|e| encoding_error(&format!("GB18030解码失败: {}", e)))
    }

    // 将UTF-8字符串转换为GB18030编码的字节
    //
    // # 参数
    // * `utf8_str` - UTF-8字符串
    //
    // # 返回值
    // GB18030编码的字节数组
    pub fn utf8_to_gb18030(utf8_str: &str) -> CtpResult<Vec<u8>> {
        GB18030
            .encode(utf8_str, EncoderTrap::Replace)
            .map_err(|e| encoding_error(&format!("UTF-8编码失败: {}", e)))
    }

    // 将UTF-8字符串转换为以null结尾的GB18030 C字符串
    //
    // # 参数
    // * `utf8_str` - UTF-8字符串
    //
    // # 返回值
    // GB18030编码的CString
    pub fn utf8_to_gb18030_cstring(utf8_str: &str) -> CtpResult<CString> {
        let gb_bytes = Self::utf8_to_gb18030(utf8_str)?;
        CString::new(gb_bytes).map_err(|e| encoding_error(&format!("创建CString失败: {}", e)))
    }

    // 从C字符串指针读取GB18030编码并转换为UTF-8
    //
    // # 安全性
    // 此函数是unsafe的，因为它解引用原始指针
    //
    // # 参数
    // * `ptr` - 指向GB18030编码C字符串的指针
    //
    // # 返回值
    // UTF-8字符串
    pub unsafe fn cstring_to_utf8(ptr: *const i8) -> CtpResult<String> {
        if ptr.is_null() {
            return Ok(String::new());
        }

        let c_str = CStr::from_ptr(ptr);
        let bytes = c_str.to_bytes();
        Self::gb18030_to_utf8(bytes)
    }

    // 移除字节数组末尾的空字节
    fn trim_null_bytes(bytes: &[u8]) -> &[u8] {
        bytes
            .iter()
            .rposition(|&b| b != 0)
            .map_or(&[], |pos| &bytes[..=pos])
    }

    // 将固定长度的字节数组转换为UTF-8字符串
    //
    // # 参数
    // * `bytes` - 固定长度的字节数组
    //
    // # 返回值
    // UTF-8字符串
    pub fn fixed_bytes_to_utf8<const N: usize>(bytes: &[u8; N]) -> CtpResult<String> {
        Self::gb18030_to_utf8(bytes)
    }

    // 将UTF-8字符串转换为固定长度的GB18030字节数组
    //
    // # 参数
    // * `utf8_str` - UTF-8字符串
    //
    // # 返回值
    // 固定长度的GB18030字节数组
    pub fn utf8_to_fixed_bytes<const N: usize>(utf8_str: &str) -> CtpResult<[u8; N]> {
        let gb_bytes = Self::utf8_to_gb18030(utf8_str)?;
        let mut result = [0u8; N];

        if gb_bytes.len() >= N {
            result.copy_from_slice(&gb_bytes[..N]);
        } else {
            result[..gb_bytes.len()].copy_from_slice(&gb_bytes);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_to_gb18030_and_back() {
        let original = "测试字符串";
        let gb_bytes = GbkConverter::utf8_to_gb18030(original).unwrap();
        let converted = GbkConverter::gb18030_to_utf8(&gb_bytes).unwrap();
        assert_eq!(original, converted);
    }

    #[test]
    fn test_empty_string() {
        let result = GbkConverter::gb18030_to_utf8(&[]).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_null_terminated_bytes() {
        let bytes = b"test\0\0\0";
        let result = GbkConverter::gb18030_to_utf8(bytes).unwrap();
        assert_eq!(result, "test");
    }

    #[test]
    fn test_fixed_bytes_conversion() {
        let original = "测试";
        let fixed_bytes: [u8; 20] = GbkConverter::utf8_to_fixed_bytes(original).unwrap();
        let converted = GbkConverter::fixed_bytes_to_utf8(&fixed_bytes).unwrap();
        assert_eq!(converted.trim_end_matches('\0'), original);
    }
}
