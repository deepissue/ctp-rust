use std::path::{Path, MAIN_SEPARATOR};

use tracing::debug;

use crate::CtpError;
pub fn normalize_flow_path(path: &str) -> Result<String, CtpError> {
    // 1. 处理空路径
    if path.trim().is_empty() {
        return Err(CtpError::InvalidPath("Path cannot be empty".to_string()));
    }

    // 2. 创建Path对象进行规范化
    let path_obj = Path::new(path);

    // 3. 转换为字符串并确保以分隔符结尾
    let mut normalized = if path_obj.is_absolute() {
        path.to_string()
    } else {
        // 转换相对路径为绝对路径
        match std::env::current_dir() {
            Ok(it) => it,
            Err(err) => return Err(CtpError::InvalidPath(err.to_string())),
        }
        .join(path_obj)
        .to_string_lossy()
        .to_string()
    };
    normalized = normalized.replace("./", "");

    // 4. 确保以路径分隔符结尾
    if !normalized.ends_with(MAIN_SEPARATOR) {
        normalized.push(MAIN_SEPARATOR);
    }

    debug!("Path normalized: '{}' -> '{}'", path, normalized);
    Ok(normalized)
}
