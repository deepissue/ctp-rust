//! 异步行情API示例
//!
//! 展示如何使用CTP Rust SDK的异步接口

use ctp_rust::api::async_md_api::{AsyncMdApi, AsyncMdEvent};
use ctp_rust::error::{CtpError, CtpResult};
use ctp_rust::types::ReqUserLoginField;
use ctp_rust::CtpConfig;
use tokio::time::{timeout, Duration};
use tracing::{debug, error, info, warn};
use tracing_subscriber;

#[tokio::main]
async fn main() -> CtpResult<()> {
    // 初始化tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("🚀 CTP异步行情API示例启动");
    info!("==========================================");

    // 从环境变量加载配置
    let config = CtpConfig::from_env().map_err(|e| {
        error!("配置加载失败: {}", e);
        CtpError::InitializationError(format!("配置加载失败: {}", e))
    })?;

    info!("配置信息:");
    info!("  服务器地址: {}", config.md_front_address);
    info!("  经纪公司代码: {}", config.broker_id);
    info!("  投资者账号: {}", config.investor_id);
    info!("  流文件路径: {}", config.flow_path);
    info!("  订阅合约: {:?}", config.instruments);
    info!("==========================================");

    // 创建异步行情API实例
    info!("📡 创建异步行情API...");
    let async_api = AsyncMdApi::new(Some(&config.flow_path), false, false, None).await?;

    // 注册前置机地址
    info!("🔗 注册前置机地址...");
    async_api.register_front(&config.md_front_address).await?;

    // 初始化API
    info!("⚡ 初始化API...");
    async_api.init().await?;

    // 等待连接建立
    info!("⏳ 等待连接建立...");
    match async_api.wait_connected(30).await {
        Ok(_) => info!("✓ 连接成功"),
        Err(e) => {
            error!("✗ 连接失败: {}", e);
            return Err(e);
        }
    }

    // 创建登录请求
    info!("🔐 创建登录请求...");
    let login_req =
        ReqUserLoginField::new(&config.broker_id, &config.investor_id, &config.password)?
            .with_product_info("AsyncRustCTP")?;

    // 异步登录
    info!("📤 发送登录请求...");
    match async_api.login(&login_req, 10).await {
        Ok(login_info) => {
            info!("✓ 登录成功");
            if let Ok(trading_day) = std::str::from_utf8(&login_info.trading_day) {
                info!("  交易日: {}", trading_day.trim_end_matches('\0'));
            }
            info!("  前置编号: {}", login_info.front_id);
            info!("  会话编号: {}", login_info.session_id);
        }
        Err(e) => {
            error!("✗ 登录失败: {}", e);
            return Err(e);
        }
    }

    // 订阅行情数据
    info!("📈 订阅行情数据...");
    let instruments: Vec<&str> = config.instruments.iter().map(|s| s.as_str()).collect();
    async_api.subscribe_market_data(&instruments).await?;
    info!("✓ 行情订阅请求已发送");

    info!("==========================================");
    info!("📊 开始接收行情数据...");
    info!("  按 Ctrl+C 退出程序");
    info!("==========================================");

    // 创建一个任务来处理事件
    let event_handler = tokio::spawn(async move {
        let mut event_count = 0;
        let mut market_data_count = 0;

        loop {
            // 使用超时来避免无限等待
            match timeout(Duration::from_secs(1), async_api.recv_event()).await {
                Ok(Some(event)) => {
                    event_count += 1;

                    match event {
                        AsyncMdEvent::Connected => {
                            info!("📡 异步事件: 连接成功");
                        }
                        AsyncMdEvent::Disconnected(reason) => {
                            warn!("📡 异步事件: 连接断开, 原因: {}", reason);
                        }
                        AsyncMdEvent::HeartBeatWarning(time_lapse) => {
                            warn!("❤️ 异步事件: 心跳超时警告, 时间间隔: {}秒", time_lapse);
                        }
                        AsyncMdEvent::LoginResponse {
                            user_login: _,
                            rsp_info,
                            request_id,
                            is_last,
                        } => {
                            debug!(
                                "🔐 异步事件: 登录响应 (请求ID: {}, 最后: {})",
                                request_id, is_last
                            );
                            if let Some(rsp) = rsp_info {
                                if !rsp.is_success() {
                                    if let Ok(error_msg) = rsp.get_error_msg() {
                                        error!("登录错误: {}", error_msg.trim_end_matches('\0'));
                                    }
                                }
                            }
                        }
                        AsyncMdEvent::SubMarketDataResponse {
                            specific_instrument,
                            rsp_info,
                            request_id,
                            is_last,
                        } => {
                            debug!(
                                "📈 异步事件: 订阅行情响应 (请求ID: {}, 最后: {})",
                                request_id, is_last
                            );
                            if let Some(rsp) = rsp_info {
                                if rsp.is_success() {
                                    if let Some(instrument) = specific_instrument {
                                        if let Ok(instrument_id) = instrument.get_instrument_id() {
                                            info!(
                                                "✓ 成功订阅合约: {}",
                                                instrument_id.trim_end_matches('\0')
                                            );
                                        }
                                    }
                                } else {
                                    if let Ok(error_msg) = rsp.get_error_msg() {
                                        error!("订阅失败: {}", error_msg.trim_end_matches('\0'));
                                    }
                                }
                            }
                        }
                        AsyncMdEvent::DepthMarketData(market_data) => {
                            market_data_count += 1;

                            // 每100条数据打印一次统计信息
                            if market_data_count % 100 == 0 {
                                info!("📊 已接收 {} 条行情数据", market_data_count);
                            }

                            // 详细显示第一条和每1000条数据
                            if market_data_count == 1 || market_data_count % 1000 == 0 {
                                if let Ok(instrument_id) = market_data.get_instrument_id() {
                                    let instrument_id = instrument_id.trim_end_matches('\0');
                                    info!("💹 行情数据 #{}: {}", market_data_count, instrument_id);
                                    info!("  最新价: {:.2}", market_data.last_price);
                                    info!(
                                        "  买一价: {:.2} (量: {})",
                                        market_data.bid_price1, market_data.bid_volume1
                                    );
                                    info!(
                                        "  卖一价: {:.2} (量: {})",
                                        market_data.ask_price1, market_data.ask_volume1
                                    );
                                    info!("  成交量: {}", market_data.volume);
                                    info!("  持仓量: {:.0}", market_data.open_interest);

                                    if let Ok(update_time) =
                                        std::str::from_utf8(&market_data.update_time)
                                    {
                                        let update_time = update_time.trim_end_matches('\0');
                                        if !update_time.is_empty() {
                                            info!(
                                                "  更新时间: {}.{:03}",
                                                update_time, market_data.update_millisec
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        AsyncMdEvent::ErrorResponse {
                            rsp_info,
                            request_id,
                            is_last,
                        } => {
                            if let Some(rsp) = rsp_info {
                                if let Ok(error_msg) = rsp.get_error_msg() {
                                    error!(
                                        "❌ 异步事件: 错误响应 (请求ID: {}, 最后: {}): {}",
                                        request_id,
                                        is_last,
                                        error_msg.trim_end_matches('\0')
                                    );
                                }
                            }
                        }
                        _ => {
                            debug!("📡 异步事件: {:?}", event);
                        }
                    }
                }
                Ok(None) => {
                    warn!("📡 事件流已关闭");
                    break;
                }
                Err(_) => {
                    // 超时，继续循环
                    if event_count > 0 && event_count % 60 == 0 {
                        debug!("📡 已处理 {} 个事件", event_count);
                    }
                }
            }
        }

        info!(
            "📡 事件处理器退出，共处理 {} 个事件，{} 条行情数据",
            event_count, market_data_count
        );
    });

    // 运行60秒
    info!("⏳ 程序将运行60秒...");
    tokio::time::sleep(Duration::from_secs(60)).await;

    // 取消事件处理任务
    event_handler.abort();

    info!("==========================================");
    info!("✅ 异步示例程序结束");
    Ok(())
}
