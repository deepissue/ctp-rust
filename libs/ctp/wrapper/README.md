# CTP Common Library

这个目录包含CTP SDK wrapper的共享代码，支持Linux和macOS平台。

## 文件说明

- `debug_logger.h` - 调试日志系统头文件
- `debug_logger.cpp` - 调试日志系统实现
- `README.md` - 本说明文档

## 使用方式

### C++ 代码中使用

```cpp
#include "../../common/debug_logger.h"

// 初始化（一般在程序启动时调用一次）
CTP_DEBUG_INIT(true, nullptr);  // 启用调试，输出到控制台
// 或
CTP_DEBUG_INIT(true, "/path/to/logfile.log");  // 启用调试，输出到文件

// 在代码中添加调试日志
CTP_DEBUG("这是一条调试信息，参数1=%d，参数2=%s", 123, "test");

// 程序结束时清理（可选）
CTP_DEBUG_CLEANUP();
```

### 从Rust代码初始化

```rust
use std::ffi::CString;

let config = CtpLogConfig {
    enable_debug: 1,
    log_file_path: std::ptr::null(), // 输出到控制台
    max_file_size_mb: 10,
    max_backup_files: 3,
};

unsafe {
    CTP_InitializeDebugLogging(&config);
}
```

## 日志格式

```
[2024-09-24 14:30:25.123] [DEBUG] [Thread:12345] [filename.cpp:142:function_name] 日志消息内容
```

## 特性

- 跨平台兼容（Linux/macOS）
- 线程安全
- 支持控制台和文件输出
- 包含时间戳、线程ID、文件名、行号、函数名
- 零性能开销（未启用时）
- 支持printf风格的格式化字符串