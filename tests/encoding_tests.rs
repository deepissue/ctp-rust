//! 编码转换专项测试
//!
//! 重点测试GB18030和UTF-8之间的编码转换功能

use ctp_rust::encoding::GbkConverter;
use ctp_rust::types::StringConvert;

#[test]
fn test_basic_encoding_conversion() {
    // 测试基本的中文字符转换
    let test_cases = vec![
        "测试",
        "期货",
        "交易",
        "行情",
        "投资者",
        "经纪公司",
        "合约代码",
        "买卖方向",
        "开平标志",
        "投机套保",
    ];

    for test_str in test_cases {
        println!("测试字符串: '{}'", test_str);

        // UTF-8 -> GB18030
        let gb_bytes = GbkConverter::utf8_to_gb18030(test_str)
            .expect(&format!("UTF-8转GB18030失败: {}", test_str));

        // GB18030 -> UTF-8
        let converted = GbkConverter::gb18030_to_utf8(&gb_bytes)
            .expect(&format!("GB18030转UTF-8失败: {}", test_str));

        assert_eq!(test_str, converted, "往返转换结果不匹配");
        println!("  ✓ 转换成功");
    }
}

#[test]
fn test_mixed_content_encoding() {
    // 测试中英文混合内容
    let test_cases = vec![
        "SHFE.rb2501",
        "DCE.i2501",
        "CZCE.TA501",
        "CFFEX.IF2501",
        "INE.sc2501",
        "rb2501期货合约",
        "TestApp_V1.0",
        "192.168.1.100:8080",
        "investor_001",
        "开仓Buy",
        "平仓Sell",
    ];

    for test_str in test_cases {
        println!("测试混合内容: '{}'", test_str);

        let gb_bytes = GbkConverter::utf8_to_gb18030(test_str)
            .expect(&format!("混合内容转换失败: {}", test_str));

        let converted = GbkConverter::gb18030_to_utf8(&gb_bytes)
            .expect(&format!("混合内容回转失败: {}", test_str));

        assert_eq!(test_str, converted, "混合内容往返转换失败");
        println!("  ✓ 混合内容转换成功");
    }
}

#[test]
fn test_special_characters() {
    // 测试特殊字符
    let test_cases = vec![
        "测试@#$%",
        "价格：123.45",
        "数量（手）",
        "买卖【方向】",
        "时间：12:34:56",
        "日期-2024/01/01",
        "错误代码：-1",
        "成功！",
        "失败？",
    ];

    for test_str in test_cases {
        println!("测试特殊字符: '{}'", test_str);

        let gb_bytes = GbkConverter::utf8_to_gb18030(test_str)
            .expect(&format!("特殊字符转换失败: {}", test_str));

        let converted = GbkConverter::gb18030_to_utf8(&gb_bytes)
            .expect(&format!("特殊字符回转失败: {}", test_str));

        assert_eq!(test_str, converted, "特殊字符往返转换失败");
        println!("  ✓ 特殊字符转换成功");
    }
}

#[test]
fn test_empty_and_edge_cases() {
    // 测试边界情况

    // 空字符串
    let empty_result = GbkConverter::utf8_to_gb18030("").expect("空字符串转换失败");
    assert!(empty_result.is_empty(), "空字符串转换结果应为空");

    let empty_back = GbkConverter::gb18030_to_utf8(&empty_result).expect("空字节转换失败");
    assert!(empty_back.is_empty(), "空字节转换结果应为空字符串");

    // 纯ASCII
    let ascii_str = "Hello123ABC";
    let ascii_bytes = GbkConverter::utf8_to_gb18030(ascii_str).expect("ASCII转换失败");
    let ascii_back = GbkConverter::gb18030_to_utf8(&ascii_bytes).expect("ASCII回转失败");
    assert_eq!(ascii_str, ascii_back, "ASCII往返转换失败");

    // 单个中文字符
    let single_char = "中";
    let single_bytes = GbkConverter::utf8_to_gb18030(single_char).expect("单字符转换失败");
    let single_back = GbkConverter::gb18030_to_utf8(&single_bytes).expect("单字符回转失败");
    assert_eq!(single_char, single_back, "单字符往返转换失败");

    println!("边界情况测试通过");
}

#[test]
fn test_null_terminated_handling() {
    // 测试带null终止符的字节处理
    let test_str = "测试字符串";
    let mut bytes_with_null = GbkConverter::utf8_to_gb18030(test_str).expect("转换失败");

    // 添加null终止符
    bytes_with_null.push(0);
    bytes_with_null.push(0);
    bytes_with_null.push(0);

    let converted = GbkConverter::gb18030_to_utf8(&bytes_with_null).expect("带null字节转换失败");

    assert_eq!(test_str, converted, "null终止符处理失败");
    println!("null终止符处理测试通过");
}

#[test]
fn test_fixed_bytes_conversion() {
    // 测试固定长度字节数组转换
    let test_str = "测试";

    // 转换为固定长度数组
    let fixed_bytes: [u8; 20] =
        GbkConverter::utf8_to_fixed_bytes(test_str).expect("固定长度转换失败");

    // 从固定长度数组转换回来
    let converted = GbkConverter::fixed_bytes_to_utf8(&fixed_bytes).expect("固定长度回转失败");

    assert_eq!(
        test_str,
        converted.trim_end_matches('\0'),
        "固定长度往返转换失败"
    );

    // 测试超长字符串截断
    let long_str = "这是一个很长的测试字符串，用来测试截断功能";
    let truncated: [u8; 10] = GbkConverter::utf8_to_fixed_bytes(long_str).expect("截断转换失败");

    let truncated_back = GbkConverter::fixed_bytes_to_utf8(&truncated).expect("截断回转失败");

    // 截断后的字符串应该不等于原字符串
    assert_ne!(
        long_str,
        truncated_back.trim_end_matches('\0'),
        "截断功能验证失败"
    );

    println!("固定长度字节数组转换测试通过");
}

#[test]
fn test_c_string_conversion() {
    // 测试C字符串转换
    let test_str = "CTP测试";

    let c_string = GbkConverter::utf8_to_gb18030_cstring(test_str).expect("C字符串转换失败");

    // 模拟从C字符串指针读取
    let ptr = c_string.as_ptr();
    let converted = unsafe { GbkConverter::cstring_to_utf8(ptr) }.expect("C字符串回转失败");

    assert_eq!(test_str, converted, "C字符串往返转换失败");

    // 测试null指针处理
    let null_result =
        unsafe { GbkConverter::cstring_to_utf8(std::ptr::null()) }.expect("null指针处理失败");

    assert!(null_result.is_empty(), "null指针应返回空字符串");

    println!("C字符串转换测试通过");
}

#[test]
fn test_string_convert_trait() {
    // 测试StringConvert特质的实现
    use ctp_rust::types::{
        BrokerIdType, InstrumentIdType, PasswordType, ProductInfoType, UserIdType,
    };

    let test_cases = vec![
        ("经纪商", "BrokerIdType"),
        ("投资者001", "UserIdType"),
        ("rb2501", "InstrumentIdType"),
        ("password123", "PasswordType"),
        ("TestApp", "ProductInfoType"),
    ];

    for (test_str, type_name) in test_cases {
        println!("测试 {} 类型转换: '{}'", type_name, test_str);

        match type_name {
            "BrokerIdType" => {
                let typed = BrokerIdType::from_utf8_string(test_str)
                    .expect(&format!("{} 转换失败", type_name));
                let back = typed
                    .to_utf8_string()
                    .expect(&format!("{} 回转失败", type_name));
                assert_eq!(test_str, back.trim_end_matches('\0'));
            }
            "UserIdType" => {
                let typed = UserIdType::from_utf8_string(test_str)
                    .expect(&format!("{} 转换失败", type_name));
                let back = typed
                    .to_utf8_string()
                    .expect(&format!("{} 回转失败", type_name));
                assert_eq!(test_str, back.trim_end_matches('\0'));
            }
            "InstrumentIdType" => {
                let typed = InstrumentIdType::from_utf8_string(test_str)
                    .expect(&format!("{} 转换失败", type_name));
                let back = typed
                    .to_utf8_string()
                    .expect(&format!("{} 回转失败", type_name));
                assert_eq!(test_str, back.trim_end_matches('\0'));
            }
            "PasswordType" => {
                let typed = PasswordType::from_utf8_string(test_str)
                    .expect(&format!("{} 转换失败", type_name));
                let back = typed
                    .to_utf8_string()
                    .expect(&format!("{} 回转失败", type_name));
                assert_eq!(test_str, back.trim_end_matches('\0'));
            }
            "ProductInfoType" => {
                let typed = ProductInfoType::from_utf8_string(test_str)
                    .expect(&format!("{} 转换失败", type_name));
                let back = typed
                    .to_utf8_string()
                    .expect(&format!("{} 回转失败", type_name));
                assert_eq!(test_str, back.trim_end_matches('\0'));
            }
            _ => panic!("未知类型: {}", type_name),
        }

        println!("  ✓ {} 转换成功", type_name);
    }
}

#[test]
fn test_performance() {
    // 性能测试
    use std::time::Instant;

    let test_str = "这是一个用于性能测试的中文字符串，包含常见的期货交易术语";
    let iterations = 10000;

    let start = Instant::now();

    for _ in 0..iterations {
        let gb_bytes = GbkConverter::utf8_to_gb18030(test_str).expect("性能测试转换失败");
        let _converted = GbkConverter::gb18030_to_utf8(&gb_bytes).expect("性能测试回转失败");
    }

    let duration = start.elapsed();
    let per_conversion = duration.as_nanos() / (iterations * 2) as u128; // 往返算两次转换

    println!("性能测试完成:");
    println!("  总时间: {:?}", duration);
    println!("  迭代次数: {}", iterations);
    println!("  每次转换耗时: {} ns", per_conversion);

    // 确保性能在合理范围内（每次转换少于1微秒）
    assert!(per_conversion < 1000, "编码转换性能不满足要求");
}

#[test]
fn test_concurrent_encoding() {
    // 并发编码测试
    use std::sync::Arc;
    use std::thread;

    let test_strings = Arc::new(vec![
        "线程1测试",
        "线程2测试",
        "线程3测试",
        "线程4测试",
        "并发编码测试",
        "多线程安全",
    ]);

    let mut handles = vec![];

    for i in 0..6 {
        let strings = Arc::clone(&test_strings);
        let handle = thread::spawn(move || {
            let test_str = &strings[i];

            for _ in 0..1000 {
                let gb_bytes = GbkConverter::utf8_to_gb18030(test_str).expect("并发测试转换失败");
                let converted = GbkConverter::gb18030_to_utf8(&gb_bytes).expect("并发测试回转失败");

                assert_eq!(test_str, &converted, "并发测试结果不匹配");
            }

            println!("线程 {} 完成", i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("线程join失败");
    }

    println!("并发编码测试通过");
}
