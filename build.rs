use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::fs;

/// 检查源文件是否比目标文件新
fn source_newer_than_target(source: &PathBuf, target: &PathBuf) -> bool {
    if !target.exists() {
        return true;
    }
    
    let source_time = fs::metadata(source)
        .and_then(|m| m.modified())
        .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        
    let target_time = fs::metadata(target)
        .and_then(|m| m.modified())
        .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        
    source_time > target_time
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=libs/");
    println!("cargo:rerun-if-changed=src/");
    
    // 获取当前构建目标
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    
    // 获取包的根目录和输出目录
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let package_root = PathBuf::from(&manifest_dir);

    // 根据平台选择目录结构，使用包根目录的绝对路径
    let (include_path, lib_path, wrapper_path, lib_suffix) =
        match (target_os.as_str(), target_arch.as_str()) {
            ("macos", "x86_64") | ("macos", "aarch64") => {
                // macOS使用传统lib结构（与Linux保持一致）
                (
                    package_root.join("libs/ctp/mac64/include"),
                    package_root.join("libs/ctp/mac64/lib"),
                    package_root.join("libs/ctp/mac64/wrapper"),
                    "dylib",
                )
            }
            ("linux", "x86_64") => {
                // Linux使用传统lib结构
                (
                    package_root.join("libs/ctp/linux/include"),
                    package_root.join("libs/ctp/linux/lib"),
                    package_root.join("libs/ctp/linux/wrapper"),
                    "so",
                )
            }
            ("linux", "aarch64") => {
                // ARM64 Linux，如果没有专用库则报错
                let arm_path = package_root.join("libs/ctp/linux_arm64");
                if arm_path.exists() {
                    (
                        arm_path.join("include"),
                        arm_path.join("lib"),
                        arm_path.join("wrapper"),
                        "so",
                    )
                } else {
                    panic!("不支持ARM64 Linux平台，CTP库仅支持x86_64架构");
                }
            }
            _ => panic!("不支持的平台: {} {}", target_os, target_arch),
        };

    // 检查库文件是否存在
    if !lib_path.exists() {
        println!("cargo:warning=CTP库目录不存在: {}", lib_path.display());
        println!("cargo:warning=请确保将CTP SDK库文件放置在正确位置");
        println!("cargo:warning=参考README.md中的CTP SDK配置说明");
        println!("cargo:rustc-cfg=feature=\"test_mode\"");
        return;
    }

    // 检查头文件是否存在
    if !include_path.exists() {
        println!("cargo:warning=CTP头文件目录不存在: {}", include_path.display());
        println!("cargo:warning=请确保将CTP SDK头文件放置在正确位置");
        println!("cargo:rustc-cfg=feature=\"test_mode\"");
        return;
    }

    // 验证必要的库文件是否存在
    let (md_lib, trader_lib) = (
        lib_path.join(format!("libthostmduserapi_se.{}", lib_suffix)),
        lib_path.join(format!("libthosttraderapi_se.{}", lib_suffix)),
    );

    let has_md_lib = md_lib.exists();
    let has_trader_lib = trader_lib.exists();

    if !has_md_lib && !has_trader_lib {
        println!("cargo:warning=CTP库文件不存在，将跳过动态链接。如需完整功能，请将CTP库文件放置在正确位置。");
        println!("cargo:warning=行情API库: {}", md_lib.display());
        println!("cargo:warning=交易API库: {}", trader_lib.display());

        // 在测试模式下不进行链接
        println!("cargo:rustc-cfg=feature=\"test_mode\"");
        println!("cargo:warning=构建目标: {} {}", target_os, target_arch);
        println!("cargo:warning=测试模式：将跳过CTP库链接");
        return;
    }

    // 编译 C++ 包装器
    let common_path = package_root.join("libs/ctp/common");
    let wrapper_cpp = wrapper_path.join("ctp_wrapper.cpp");
    let spi_bridge_cpp = wrapper_path.join("spi_bridge.cpp");
    let debug_logger_cpp = common_path.join("debug_logger.cpp");
    
    // 使用 OUT_DIR 存放编译产物
    let wrapper_obj = out_dir.join("ctp_wrapper.o");
    let spi_bridge_obj = out_dir.join("spi_bridge.o");
    let debug_logger_obj = out_dir.join("debug_logger.o");

    // 检查包装器库是否已经存在（同时检查源目录和输出目录）
    let existing_wrapper_lib_src = if target_os == "macos" {
        wrapper_path.join("libctp_wrapper.dylib")
    } else {
        wrapper_path.join("libctp_wrapper.so")
    };
    
    let wrapper_lib_out = if target_os == "macos" {
        out_dir.join("libctp_wrapper.dylib")
    } else {
        out_dir.join("libctp_wrapper.so")
    };

    // 检查源文件是否存在
    if !wrapper_cpp.exists() {
        println!("cargo:warning=CTP wrapper源文件不存在: {}", wrapper_cpp.display());
        println!("cargo:warning=请确保完整下载了ctp-rust包的源代码");
        println!("cargo:rustc-cfg=feature=\"test_mode\"");
        return;
    }
    
    if !spi_bridge_cpp.exists() {
        println!("cargo:warning=SPI bridge源文件不存在: {}", spi_bridge_cpp.display());
        println!("cargo:rustc-cfg=feature=\"test_mode\"");
        return;
    }
    
    if !debug_logger_cpp.exists() {
        println!("cargo:warning=Debug logger源文件不存在: {}", debug_logger_cpp.display());
        println!("cargo:rustc-cfg=feature=\"test_mode\"");
        return;
    }

    // 始终重新编译wrapper库以确保兼容性
    let should_compile = !wrapper_lib_out.exists() || 
        source_newer_than_target(&wrapper_cpp, &wrapper_lib_out) ||
        source_newer_than_target(&spi_bridge_cpp, &wrapper_lib_out) ||
        source_newer_than_target(&debug_logger_cpp, &wrapper_lib_out);

    if should_compile {
        println!("cargo:warning=编译CTP C++包装器");

        // 编译wrapper
        let mut cmd = if target_os == "macos" {
            Command::new("clang++")
        } else {
            Command::new("g++")
        };
        cmd.arg("-c")
            .arg("-std=c++11")
            .arg("-fPIC")
            .arg("-I")
            .arg(&include_path)
            .arg("-I")
            .arg(&common_path)
            .arg("-o")
            .arg(&wrapper_obj)
            .arg(&wrapper_cpp);

        // 添加macOS特定的编译选项
        if target_os == "macos" {
            cmd.arg("-mmacosx-version-min=14.0");
        }

        let output = cmd.output().expect("Failed to compile CTP wrapper");

        if !output.status.success() {
            panic!(
                "Failed to compile CTP wrapper: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        // 编译SPI bridge
        let mut spi_cmd = if target_os == "macos" {
            Command::new("clang++")
        } else {
            Command::new("g++")
        };
        spi_cmd
            .arg("-c")
            .arg("-std=c++11")
            .arg("-fPIC")
            .arg("-I")
            .arg(&include_path)
            .arg("-I")
            .arg(&common_path)
            .arg("-o")
            .arg(&spi_bridge_obj)
            .arg(&spi_bridge_cpp);

        // 添加macOS特定的编译选项
        if target_os == "macos" {
            spi_cmd.arg("-mmacosx-version-min=14.0");
        }

        let spi_output = spi_cmd.output().expect("Failed to compile SPI bridge");

        if !spi_output.status.success() {
            panic!(
                "Failed to compile SPI bridge: {}",
                String::from_utf8_lossy(&spi_output.stderr)
            );
        }

        // 编译debug logger
        let mut debug_cmd = if target_os == "macos" {
            Command::new("clang++")
        } else {
            Command::new("g++")
        };
        debug_cmd
            .arg("-c")
            .arg("-std=c++11")
            .arg("-fPIC")
            .arg("-I")
            .arg(&include_path)
            .arg("-I")
            .arg(&common_path)
            .arg("-o")
            .arg(&debug_logger_obj)
            .arg(&debug_logger_cpp);

        // 添加macOS特定的编译选项
        if target_os == "macos" {
            debug_cmd.arg("-mmacosx-version-min=14.0");
        }

        let debug_output = debug_cmd.output().expect("Failed to compile debug logger");

        if !debug_output.status.success() {
            panic!(
                "Failed to compile debug logger: {}",
                String::from_utf8_lossy(&debug_output.stderr)
            );
        }

        // 创建动态库到OUT_DIR
        let wrapper_lib = wrapper_lib_out.clone();

        // 创建动态库而不是静态库
        let mut link_cmd = if target_os == "macos" {
            let mut cmd = Command::new("clang++");
            cmd.arg("-shared")
                .arg("-o")
                .arg(&wrapper_lib)
                .arg(&wrapper_obj)
                .arg(&spi_bridge_obj)
                .arg(&debug_logger_obj)
                .arg("-L")
                .arg(&lib_path)
                .arg("-lthostmduserapi_se")
                .arg("-lthosttraderapi_se");
            cmd
        } else {
            let mut cmd = Command::new("g++");
            cmd.arg("-shared")
                .arg("-fPIC")
                .arg("-o")
                .arg(&wrapper_lib)
                .arg(&wrapper_obj)
                .arg(&spi_bridge_obj)
                .arg(&debug_logger_obj)
                .arg("-L")
                .arg(&lib_path)
                .arg("-lthostmduserapi_se")
                .arg("-lthosttraderapi_se")
                .arg("-lpthread")
                .arg("-ldl");
            cmd
        };

        let link_output = link_cmd
            .output()
            .expect("Failed to create wrapper dynamic library");

        if !link_output.status.success() {
            panic!(
                "Failed to create wrapper dynamic library: {}",
                String::from_utf8_lossy(&link_output.stderr)
            );
        }

        println!("cargo:warning=CTP C++包装器编译完成");
    } else {
        // 检查是否存在预编译的库
        if existing_wrapper_lib_src.exists() {
            println!("cargo:warning=发现预编译的包装器库，复制到构建目录");
            if let Err(e) = std::fs::copy(&existing_wrapper_lib_src, &wrapper_lib_out) {
                println!("cargo:warning=复制包装器库失败: {}", e);
                // 继续尝试直接使用源目录中的库
                println!("cargo:rustc-link-search=native={}", wrapper_path.display());
            }
        } else if !wrapper_lib_out.exists() {
            println!("cargo:warning=未找到CTP包装器库，尝试重新编译");
            // 强制重新编译
            println!("cargo:warning=编译CTP C++包装器");
            
            // ... 这里可以复制编译逻辑，或者设置一个标志重新执行编译
        }
        
        if wrapper_lib_out.exists() {
            println!("cargo:warning=使用已存在的CTP C++包装器库");
        }
    }

    // 无论如何都要确保输出链接指令
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=dylib=ctp_wrapper");

    // 输出给 Cargo，让 Rust 知道去哪里找库
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // 设置条件编译标志
    if has_md_lib {
        println!("cargo:rustc-cfg=feature=\"md_api\"");
        println!("cargo:rustc-link-lib=dylib=thostmduserapi_se");
        println!("cargo:warning=找到行情API库: {}", md_lib.display());
    } else {
        println!("cargo:warning=行情API库不存在: {}", md_lib.display());
    }

    if has_trader_lib {
        println!("cargo:rustc-cfg=feature=\"trader_api\"");
        println!("cargo:rustc-link-lib=dylib=thosttraderapi_se");
        println!("cargo:warning=找到交易API库: {}", trader_lib.display());
    } else {
        println!("cargo:warning=交易API库不存在: {}", trader_lib.display());
    }

    // 如果没有任何库文件，启用测试模式
    if !has_md_lib && !has_trader_lib {
        println!("cargo:rustc-cfg=feature=\"test_mode\"");
    }

    // 在macOS上可能需要额外的系统库
    if target_os == "macos" {
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=framework=SystemConfiguration");
    }

    // 在Linux上可能需要额外的系统库
    if target_os == "linux" {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=pthread");
        println!("cargo:rustc-link-lib=dylib=dl");
    }

    // 设置rpath以便运行时能找到动态库
    if target_os == "macos" {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());
        // 设置macOS最低版本以匹配系统环境
        println!("cargo:rustc-link-arg=-Wl,-macosx_version_min,14.0");
    } else if target_os == "linux" {
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());
    }

    // rerun 规则
    println!("cargo:rerun-if-changed={}", include_path.display());
    println!("cargo:rerun-if-changed={}", lib_path.display());
    println!("cargo:rerun-if-changed={}", wrapper_path.display());
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");

    // 输出构建信息
    println!("cargo:warning=构建目标: {} {}", target_os, target_arch);
    println!("cargo:warning=使用库目录: {}", lib_path.display());
}
