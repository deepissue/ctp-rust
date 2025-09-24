//! 编码转换演示
//!
//! 展示CTP Rust SDK中的GB18030和UTF-8编码转换功能

use ctp_rust::encoding::GbkConverter;
use ctp_rust::error::CtpResult;
use ctp_rust::types::{
    BrokerIdType, InstrumentIdType, PasswordType, ProductInfoType, ReqUserLoginField,
    StringConvert, UserIdType,
};
use tracing::info;
use tracing_subscriber;

fn main() -> CtpResult<()> {
    // 初始化tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("🔄 CTP编码转换演示");
    info!("==========================================");

    // 1. 基础编码转换
    demo_basic_encoding()?;

    // 2. 固定长度类型转换
    demo_fixed_length_types()?;

    // 3. 结构体字段转换
    demo_struct_conversion()?;

    // 4. 特殊字符处理
    demo_special_characters()?;

    // 5. C字符串转换
    demo_c_string_conversion()?;

    // 6. 性能测试
    demo_performance_test()?;

    println!("✅ 编码转换演示完成");
    Ok(())
}

/// 基础编码转换演示
fn demo_basic_encoding() -> CtpResult<()> {
    println!("\n📋 1. 基础编码转换");
    println!("------------------------------------------");

    let test_strings = vec![
        "期货交易",
        "螺纹钢",
        "铁矿石",
        "动力煤",
        "白糖",
        "SHFE.rb2501",
        "DCE.i2501",
        "Hello World",
        "测试Test123",
        "",
    ];

    for test_str in test_strings {
        println!("原始字符串: '{}'", test_str);

        // UTF-8 -> GB18030
        let gb_bytes = GbkConverter::utf8_to_gb18030(test_str)?;
        println!("  GB18030字节: {} bytes", gb_bytes.len());

        // GB18030 -> UTF-8
        let converted = GbkConverter::gb18030_to_utf8(&gb_bytes)?;
        println!("  转换结果: '{}'", converted);

        // 验证一致性
        assert_eq!(test_str, converted, "往返转换结果不一致");
        println!("  ✓ 往返转换一致");
        println!();
    }

    Ok(())
}

/// 固定长度类型转换演示
fn demo_fixed_length_types() -> CtpResult<()> {
    println!("\n📋 2. 固定长度类型转换");
    println!("------------------------------------------");

    // 经纪公司代码 (11字符)
    let broker_id_str = "9999";
    let broker_id = BrokerIdType::from_utf8_string(broker_id_str)?;
    let converted_broker = broker_id.to_utf8_string()?;
    println!("经纪公司代码:");
    println!("  原始: '{}'", broker_id_str);
    println!("  字节数组: {:?}", &broker_id[..8]); // 只显示前8个字节
    println!("  转换回: '{}'", converted_broker.trim_end_matches('\0'));
    println!("  ✓ 转换成功");
    println!();

    // 用户代码 (16字符)
    let user_id_str = "投资者001";
    let user_id = UserIdType::from_utf8_string(user_id_str)?;
    let converted_user = user_id.to_utf8_string()?;
    println!("用户代码:");
    println!("  原始: '{}'", user_id_str);
    println!("  字节数组: {:?}", &user_id[..12]); // 只显示前12个字节
    println!("  转换回: '{}'", converted_user.trim_end_matches('\0'));
    println!("  ✓ 转换成功");
    println!();

    // 合约代码 (31字符)
    let instrument_id_str = "rb2501";
    let instrument_id = InstrumentIdType::from_utf8_string(instrument_id_str)?;
    let converted_instrument = instrument_id.to_utf8_string()?;
    println!("合约代码:");
    println!("  原始: '{}'", instrument_id_str);
    println!("  字节数组: {:?}", &instrument_id[..8]);
    println!(
        "  转换回: '{}'",
        converted_instrument.trim_end_matches('\0')
    );
    println!("  ✓ 转换成功");
    println!();

    // 密码 (41字符)
    let password_str = "password123";
    let password = PasswordType::from_utf8_string(password_str)?;
    let converted_password = password.to_utf8_string()?;
    println!("密码:");
    println!("  原始: '{}'", password_str);
    println!("  转换回: '{}'", converted_password.trim_end_matches('\0'));
    println!("  ✓ 转换成功");
    println!();

    // 产品信息 (11字符)
    let product_str = "RustCTP";
    let product = ProductInfoType::from_utf8_string(product_str)?;
    let converted_product = product.to_utf8_string()?;
    println!("产品信息:");
    println!("  原始: '{}'", product_str);
    println!("  转换回: '{}'", converted_product.trim_end_matches('\0'));
    println!("  ✓ 转换成功");
    println!();

    Ok(())
}

/// 结构体字段转换演示
fn demo_struct_conversion() -> CtpResult<()> {
    println!("\n📋 3. 结构体字段转换");
    println!("------------------------------------------");

    // 创建登录请求
    let login_req = ReqUserLoginField::new("9999", "投资者001", "password123")?
        .with_product_info("RustCTP")?
        .with_auth_code("AUTH123456")?
        .with_mac_address("00:11:22:33:44:55")?
        .with_client_ip("192.168.1.100", "12345")?;

    println!("登录请求结构体:");

    // 解析并显示各字段
    let broker_id = login_req.broker_id.to_utf8_string()?;
    println!("  经纪公司代码: '{}'", broker_id.trim_end_matches('\0'));

    let user_id = login_req.user_id.to_utf8_string()?;
    println!("  用户代码: '{}'", user_id.trim_end_matches('\0'));

    let password = login_req.password.to_utf8_string()?;
    println!("  密码: '{}'", password.trim_end_matches('\0'));

    let product_info = login_req.user_product_info.to_utf8_string()?;
    println!("  产品信息: '{}'", product_info.trim_end_matches('\0'));

    let auth_code = login_req.one_time_password.to_utf8_string()?;
    println!("  认证码: '{}'", auth_code.trim_end_matches('\0'));

    let mac_address = login_req.mac_address.to_utf8_string()?;
    println!("  MAC地址: '{}'", mac_address.trim_end_matches('\0'));

    let client_ip = login_req.client_ip_address.to_utf8_string()?;
    println!("  客户端IP: '{}'", client_ip.trim_end_matches('\0'));

    let client_port = login_req.client_ip_port.to_utf8_string()?;
    println!("  客户端端口: '{}'", client_port.trim_end_matches('\0'));

    println!("  ✓ 结构体字段转换成功");
    println!();

    Ok(())
}

/// 特殊字符处理演示
fn demo_special_characters() -> CtpResult<()> {
    println!("\n📋 4. 特殊字符处理");
    println!("------------------------------------------");

    let special_strings = vec![
        "价格：123.45",
        "数量（手）",
        "买卖【方向】",
        "时间：12:34:56",
        "日期-2024/01/01",
        "错误代码：-1",
        "成功！",
        "失败？",
        "测试@#$%^&*()",
        "混合ASCII中文123",
    ];

    for test_str in special_strings {
        println!("特殊字符串: '{}'", test_str);

        // 转换测试
        let gb_bytes = GbkConverter::utf8_to_gb18030(test_str)?;
        let converted = GbkConverter::gb18030_to_utf8(&gb_bytes)?;

        // 验证
        assert_eq!(test_str, converted, "特殊字符转换失败");
        println!("  ✓ 特殊字符处理正确");

        // 显示字节信息
        println!(
            "  UTF-8字节数: {}, GB18030字节数: {}",
            test_str.len(),
            gb_bytes.len()
        );
        println!();
    }

    Ok(())
}

/// C字符串转换演示
fn demo_c_string_conversion() -> CtpResult<()> {
    println!("\n📋 5. C字符串转换");
    println!("------------------------------------------");

    let test_strings = vec!["CTP交易API", "期货合约代码", "投资者账户", "Hello CTP", ""];

    for test_str in test_strings {
        println!("C字符串转换: '{}'", test_str);

        // 转换为C字符串
        let c_string = GbkConverter::utf8_to_gb18030_cstring(test_str)?;
        println!("  C字符串长度: {} bytes", c_string.as_bytes().len());

        // 从C字符串指针读取
        let ptr = c_string.as_ptr();
        let converted = unsafe { GbkConverter::cstring_to_utf8(ptr) }?;

        // 验证
        assert_eq!(test_str, converted, "C字符串转换失败");
        println!("  ✓ C字符串转换正确");
        println!();
    }

    // 测试null指针处理
    println!("测试null指针处理:");
    let null_result = unsafe { GbkConverter::cstring_to_utf8(std::ptr::null()) }?;
    assert!(null_result.is_empty(), "null指针应返回空字符串");
    println!("  ✓ null指针处理正确");
    println!();

    Ok(())
}

/// 性能测试演示
fn demo_performance_test() -> CtpResult<()> {
    println!("\n📋 6. 性能测试");
    println!("------------------------------------------");

    let test_str = "这是一个用于性能测试的中文字符串，包含常见的期货交易术语和英文字符ABC123";
    let iterations = 10000;

    println!("测试字符串: '{}'", test_str);
    println!("迭代次数: {}", iterations);

    // UTF-8 -> GB18030 性能测试
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _gb_bytes = GbkConverter::utf8_to_gb18030(test_str)?;
    }
    let utf8_to_gb_duration = start.elapsed();

    // GB18030 -> UTF-8 性能测试
    let gb_bytes = GbkConverter::utf8_to_gb18030(test_str)?;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _converted = GbkConverter::gb18030_to_utf8(&gb_bytes)?;
    }
    let gb_to_utf8_duration = start.elapsed();

    // 往返转换性能测试
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let gb_bytes = GbkConverter::utf8_to_gb18030(test_str)?;
        let _converted = GbkConverter::gb18030_to_utf8(&gb_bytes)?;
    }
    let roundtrip_duration = start.elapsed();

    println!("性能测试结果:");
    println!(
        "  UTF-8 -> GB18030: {:?} ({:.2} ns/op)",
        utf8_to_gb_duration,
        utf8_to_gb_duration.as_nanos() as f64 / iterations as f64
    );
    println!(
        "  GB18030 -> UTF-8: {:?} ({:.2} ns/op)",
        gb_to_utf8_duration,
        gb_to_utf8_duration.as_nanos() as f64 / iterations as f64
    );
    println!(
        "  往返转换: {:?} ({:.2} ns/op)",
        roundtrip_duration,
        roundtrip_duration.as_nanos() as f64 / iterations as f64
    );

    // 固定长度类型性能测试
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _typed = InstrumentIdType::from_utf8_string("rb2501")?;
    }
    let fixed_type_duration = start.elapsed();

    println!(
        "  固定长度类型转换: {:?} ({:.2} ns/op)",
        fixed_type_duration,
        fixed_type_duration.as_nanos() as f64 / iterations as f64
    );

    println!("  ✓ 性能测试完成");
    println!();

    Ok(())
}
