//! 错误处理示例
//!
//! 展示CTP Rust SDK中的错误处理机制和最佳实践

use ctp_rust::encoding::GbkConverter;
use ctp_rust::error::{CtpError, CtpResult};
use ctp_rust::types::{BrokerIdType, ReqUserLoginField, RspInfoField, StringConvert};
use tracing::info;
use tracing_subscriber;

fn main() -> CtpResult<()> {
    // 初始化tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("🔧 CTP错误处理示例");
    info!("==========================================");

    // 1. 错误类型演示
    demo_error_types();

    // 2. 编码错误处理
    demo_encoding_errors()?;

    // 3. 类型转换错误处理
    demo_type_conversion_errors()?;

    // 4. API调用错误处理
    demo_api_errors()?;

    // 5. 响应信息错误处理
    demo_response_errors()?;

    // 6. 错误恢复策略
    demo_error_recovery()?;

    println!("✅ 错误处理演示完成");
    Ok(())
}

/// 错误类型演示
fn demo_error_types() {
    println!("\n📋 1. 错误类型演示");
    println!("------------------------------------------");

    let errors = vec![
        CtpError::FfiError("FFI调用失败".to_string()),
        CtpError::EncodingError("编码转换错误".to_string()),
        CtpError::ConnectionError("网络连接断开".to_string()),
        CtpError::AuthenticationError("用户认证失败".to_string()),
        CtpError::BusinessError(-1, "CTP业务逻辑错误".to_string()),
        CtpError::InitializationError("API初始化失败".to_string()),
        CtpError::TimeoutError("操作超时".to_string()),
        CtpError::InvalidParameterError("参数验证失败".to_string()),
        CtpError::MemoryError("内存分配失败".to_string()),
        CtpError::Other("未知错误".to_string()),
    ];

    for error in errors {
        println!("错误类型: {:?}", error);
        println!("错误信息: {}", error);
        println!(
            "是否为业务错误: {}",
            matches!(error, CtpError::BusinessError(_, _))
        );
        println!();
    }
}

/// 编码错误处理演示
fn demo_encoding_errors() -> CtpResult<()> {
    println!("\n📋 2. 编码错误处理");
    println!("------------------------------------------");

    // 正常编码转换
    println!("✓ 正常编码转换:");
    let normal_str = "正常的中文字符串";
    match GbkConverter::utf8_to_gb18030(normal_str) {
        Ok(bytes) => {
            println!("  转换成功: {} -> {} bytes", normal_str, bytes.len());

            match GbkConverter::gb18030_to_utf8(&bytes) {
                Ok(converted) => println!("  回转成功: {}", converted),
                Err(e) => println!("  回转失败: {}", e),
            }
        }
        Err(e) => println!("  转换失败: {}", e),
    }
    println!();

    // 处理空字符串
    println!("✓ 空字符串处理:");
    let empty_str = "";
    match GbkConverter::utf8_to_gb18030(empty_str) {
        Ok(bytes) => println!("  空字符串转换成功: {} bytes", bytes.len()),
        Err(e) => println!("  空字符串转换失败: {}", e),
    }
    println!();

    // 处理特殊字符
    println!("✓ 特殊字符处理:");
    let special_chars = vec!["🎉", "👍", "🚀"]; // 这些emoji可能在GB18030中无法表示

    for emoji in special_chars {
        match GbkConverter::utf8_to_gb18030(emoji) {
            Ok(bytes) => {
                println!("  特殊字符 '{}' 转换成功: {} bytes", emoji, bytes.len());
                match GbkConverter::gb18030_to_utf8(&bytes) {
                    Ok(converted) => println!("    回转结果: '{}'", converted),
                    Err(e) => println!("    回转失败: {}", e),
                }
            }
            Err(e) => println!("  特殊字符 '{}' 转换失败: {}", emoji, e),
        }
    }
    println!();

    // 处理无效字节序列
    println!("✓ 无效字节序列处理:");
    let invalid_bytes = vec![0xFF, 0xFE, 0xFD]; // 可能的无效GB18030序列
    match GbkConverter::gb18030_to_utf8(&invalid_bytes) {
        Ok(converted) => println!("  无效字节转换成功: '{}'", converted),
        Err(e) => println!("  无效字节转换失败 (预期): {}", e),
    }
    println!();

    Ok(())
}

/// 类型转换错误处理演示
fn demo_type_conversion_errors() -> CtpResult<()> {
    println!("\n📋 3. 类型转换错误处理");
    println!("------------------------------------------");

    // 正常的类型转换
    println!("✓ 正常类型转换:");
    let normal_broker_id = "9999";
    match BrokerIdType::from_utf8_string(normal_broker_id) {
        Ok(broker_id) => {
            println!("  经纪公司代码转换成功: '{}'", normal_broker_id);
            match broker_id.to_utf8_string() {
                Ok(converted) => println!("  回转成功: '{}'", converted.trim_end_matches('\0')),
                Err(e) => println!("  回转失败: {}", e),
            }
        }
        Err(e) => println!("  转换失败: {}", e),
    }
    println!();

    // 超长字符串处理
    println!("✓ 超长字符串处理:");
    let long_string =
        "这是一个非常长的字符串，超过了经纪公司代码字段的最大长度限制，应该被正确截断或报错";
    match BrokerIdType::from_utf8_string(long_string) {
        Ok(broker_id) => {
            println!("  超长字符串转换成功 (被截断)");
            match broker_id.to_utf8_string() {
                Ok(converted) => println!("  截断结果: '{}'", converted.trim_end_matches('\0')),
                Err(e) => println!("  回转失败: {}", e),
            }
        }
        Err(e) => println!("  超长字符串转换失败: {}", e),
    }
    println!();

    // 包含null字符的字符串
    println!("✓ 包含null字符的字符串:");
    let null_string = "测试\0字符串";
    match BrokerIdType::from_utf8_string(null_string) {
        Ok(broker_id) => {
            println!("  包含null的字符串转换成功");
            match broker_id.to_utf8_string() {
                Ok(converted) => println!("  结果: '{}'", converted.replace('\0', "\\0")),
                Err(e) => println!("  回转失败: {}", e),
            }
        }
        Err(e) => println!("  包含null的字符串转换失败: {}", e),
    }
    println!();

    Ok(())
}

/// API调用错误处理演示
fn demo_api_errors() -> CtpResult<()> {
    println!("\n📋 4. API调用错误处理");
    println!("------------------------------------------");

    // 正常的登录请求创建
    println!("✓ 正常登录请求创建:");
    match ReqUserLoginField::new("9999", "investor1", "password123") {
        Ok(mut req) => {
            println!("✅ 基础登录请求创建成功");

            // 尝试添加产品信息
            req = match req.clone().with_product_info("RustCTP") {
                Ok(req_with_product) => {
                    println!("✅ 添加产品信息成功");
                    req_with_product
                }
                Err(e) => {
                    println!("❌ 添加产品信息失败: {}", e);
                    req
                }
            };

            // 尝试添加MAC地址
            req = match req.clone().with_mac_address("00:11:22:33:44:55") {
                Ok(req_with_mac) => {
                    println!("✅ 添加MAC地址成功");
                    req_with_mac
                }
                Err(e) => {
                    println!("❌ 添加MAC地址失败: {}", e);
                    req
                }
            };

            // 尝试添加客户端IP
            req = match req.clone().with_client_ip("192.168.1.100", "12345") {
                Ok(req_with_ip) => {
                    println!("✅ 添加客户端IP成功");
                    req_with_ip
                }
                Err(e) => {
                    println!("❌ 添加客户端IP失败: {}", e);
                    req
                }
            };

            println!("最终请求: {:?}", req);
        }
        Err(e) => println!("❌ 登录请求创建失败: {}", e),
    }

    println!();

    // 测试无效参数
    println!("✓ 无效参数处理:");

    // 空的经纪公司代码
    match ReqUserLoginField::new("", "investor1", "password123") {
        Ok(_) => println!("  空经纪公司代码被接受"),
        Err(e) => println!("  空经纪公司代码被拒绝: {}", e),
    }

    // 空的用户代码
    match ReqUserLoginField::new("9999", "", "password123") {
        Ok(_) => println!("  空用户代码被接受"),
        Err(e) => println!("  空用户代码被拒绝: {}", e),
    }

    // 空的密码
    match ReqUserLoginField::new("9999", "investor1", "") {
        Ok(_) => println!("  空密码被接受"),
        Err(e) => println!("  空密码被拒绝: {}", e),
    }

    println!();

    Ok(())
}

/// 响应信息错误处理演示
fn demo_response_errors() -> CtpResult<()> {
    println!("\n📋 5. 响应信息错误处理");
    println!("------------------------------------------");

    // 成功响应
    println!("✓ 成功响应处理:");
    let success_rsp = RspInfoField {
        error_id: 0,
        error_msg: [0; 81], // 空错误消息
    };

    if success_rsp.is_success() {
        println!("  操作成功");
    } else {
        match success_rsp.get_error_msg() {
            Ok(msg) => println!("  操作失败: {}", msg.trim_end_matches('\0')),
            Err(e) => println!("  获取错误消息失败: {}", e),
        }
    }
    println!();

    // 失败响应
    println!("✓ 失败响应处理:");
    let mut error_rsp = RspInfoField {
        error_id: -1,
        error_msg: [0; 81],
    };

    // 设置错误消息
    if let Ok(error_bytes) = GbkConverter::utf8_to_fixed_bytes::<81>("用户名或密码错误") {
        error_rsp.error_msg = error_bytes;
    }

    if error_rsp.is_success() {
        println!("  操作成功");
    } else {
        match error_rsp.get_error_msg() {
            Ok(msg) => println!(
                "  操作失败 [{}]: {}",
                error_rsp.error_id,
                msg.trim_end_matches('\0')
            ),
            Err(e) => println!("  获取错误消息失败: {}", e),
        }
    }
    println!();

    // 常见CTP错误代码处理
    println!("✓ 常见CTP错误代码:");
    let common_errors = vec![
        (-1, "用户名不存在"),
        (-2, "用户密码错误"),
        (-3, "用户已经登录"),
        (-4, "用户不在线"),
        (-5, "重复登录"),
        (-6, "登录失败"),
        (-7, "未授权的IP地址"),
        (-8, "MAC地址不匹配"),
        (-9, "认证失败"),
        (-10, "版本不匹配"),
    ];

    for (error_code, error_desc) in common_errors {
        let mut rsp = RspInfoField {
            error_id: error_code,
            error_msg: [0; 81],
        };

        if let Ok(error_bytes) = GbkConverter::utf8_to_fixed_bytes::<81>(error_desc) {
            rsp.error_msg = error_bytes;
        }

        if !rsp.is_success() {
            match rsp.get_error_msg() {
                Ok(msg) => println!("  错误 [{}]: {}", error_code, msg.trim_end_matches('\0')),
                Err(e) => println!("  错误 [{}]: 获取消息失败 - {}", error_code, e),
            }
        }
    }
    println!();

    Ok(())
}

/// 错误恢复策略演示
fn demo_error_recovery() -> CtpResult<()> {
    println!("\n📋 6. 错误恢复策略");
    println!("------------------------------------------");

    // 重试机制示例
    println!("✓ 重试机制:");
    let max_retries = 3;
    let mut attempt = 0;

    loop {
        attempt += 1;
        println!("  尝试第 {} 次连接...", attempt);

        // 模拟可能失败的操作
        let success = attempt >= 2; // 第2次尝试成功

        if success {
            println!("  ✓ 连接成功");
            break;
        } else {
            println!("  ✗ 连接失败");

            if attempt >= max_retries {
                println!("  达到最大重试次数，放弃连接");
                return Err(CtpError::ConnectionError("重试次数超限".to_string()));
            }

            println!("  等待重试...");
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    println!();

    // 降级处理示例
    println!("✓ 降级处理:");

    // 尝试连接主服务器
    let primary_server_result = simulate_server_connection("主服务器", false);
    match primary_server_result {
        Ok(_) => println!("  ✓ 主服务器连接成功"),
        Err(e) => {
            println!("  ✗ 主服务器连接失败: {}", e);
            println!("  尝试连接备用服务器...");

            // 降级到备用服务器
            let backup_server_result = simulate_server_connection("备用服务器", true);
            match backup_server_result {
                Ok(_) => println!("  ✓ 备用服务器连接成功"),
                Err(e) => {
                    println!("  ✗ 备用服务器连接失败: {}", e);
                    println!("  进入离线模式...");
                    return Err(CtpError::ConnectionError("所有服务器都不可用".to_string()));
                }
            }
        }
    }
    println!();

    // 错误上报和日志记录
    println!("✓ 错误日志记录:");
    let sample_errors = vec![
        CtpError::ConnectionError("网络连接超时".to_string()),
        CtpError::AuthenticationError("认证失败".to_string()),
        CtpError::BusinessError(-3, "用户已登录".to_string()),
    ];

    for error in sample_errors {
        log_error(&error);
    }
    println!();

    Ok(())
}

/// 模拟服务器连接
fn simulate_server_connection(server_name: &str, success: bool) -> CtpResult<()> {
    if success {
        Ok(())
    } else {
        Err(CtpError::ConnectionError(format!("{}不可用", server_name)))
    }
}

/// 错误日志记录
fn log_error(error: &CtpError) {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    match error {
        CtpError::ConnectionError(msg) => {
            println!("  [{}] 连接错误: {}", timestamp, msg);
        }
        CtpError::AuthenticationError(msg) => {
            println!("  [{}] 认证错误: {}", timestamp, msg);
        }
        CtpError::BusinessError(code, msg) => {
            println!("  [{}] 业务错误 [{}]: {}", timestamp, code, msg);
        }
        _ => {
            println!("  [{}] 其他错误: {}", timestamp, error);
        }
    }
}
