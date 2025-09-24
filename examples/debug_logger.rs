use ctp_rust::ffi::{CTP_CleanupDebugLogging, CTP_InitializeDebugLogging, CtpLogConfig};
use ctp_rust::{api::CtpApi, TraderApi};
use std::ffi::CString;
use std::ptr;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== CTP SDK Debug Logger测试 ===");

    // 配置启用Debug日志，输出到控制台
    let config = CtpLogConfig {
        enable_debug: 1,            // 启用调试
        log_file_path: ptr::null(), // 输出到控制台
        max_file_size_mb: 10,
        max_backup_files: 3,
    };

    println!("初始化Debug日志...");
    unsafe {
        CTP_InitializeDebugLogging(&config);
    }

    println!("创建TraderApi实例...");
    let flow_path = CString::new("flow/").unwrap(); // flow文件在项目根目录
    match TraderApi::new(Some(flow_path.to_str().unwrap()), Some(false)) {
        Ok(mut api) => {
            println!("✅ TraderApi创建成功");

            // 注册前置机
            let front_addr = "tcp://180.168.146.187:10211";
            println!("注册前置机地址: {}", front_addr);
            api.register_front(front_addr).expect("注册前置机失败");

            // 初始化API（这会触发连接）
            println!("初始化API...");
            api.init().expect("初始化API失败");

            // 等待一段时间让连接尝试进行
            println!("等待5秒，观察日志输出...");
            thread::sleep(Duration::from_secs(5));

            println!("✅ 测试完成，查看上方的Debug日志输出");
        }
        Err(e) => {
            println!("❌ TraderApi创建失败: {:?}", e);
        }
    }

    // 清理日志
    println!("清理Debug日志...");
    unsafe {
        CTP_CleanupDebugLogging();
    }

    println!("=== 测试结束 ===");
}
