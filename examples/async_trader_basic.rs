//! 异步交易功能示例
//!
//! 展示如何使用CTP Rust SDK的异步交易API进行期货交易操作

use ctp_rust::api::AsyncTraderApi;
use ctp_rust::api::async_trader_api::AsyncTraderEvent;
use ctp_rust::types::{
    QryInvestorPositionField, QryTradingAccountField, ReqUserLoginField,
};
use ctp_rust::*;
use tokio::time::{sleep, Duration, timeout};
use tracing::{error, info, warn};
use tracing_subscriber;

#[tokio::main]
async fn main() -> CtpResult<()> {
    // 初始化tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .init();

    info!("🚀 异步CTP交易功能示例启动");
    info!("==========================================");

    // 从环境变量加载配置
    let config = CtpConfig::from_env().map_err(|e| {
        error!("配置加载失败: {}", e);
        CtpError::InitializationError(format!("配置加载失败: {}", e))
    })?;

    info!("配置信息:");
    info!("  交易服务器地址: {}", config.trader_front_address);
    info!("  经纪公司代码: {}", config.broker_id);
    info!("  投资者账号: {}", config.investor_id);
    info!("  流文件路径: {}", config.flow_path);
    info!("==========================================");

    // 创建异步交易API实例
    info!("📡 创建异步交易API...");
    let async_trader = AsyncTraderApi::new(Some(&config.flow_path), Some(true)).await?;

    // 注册前置机地址
    info!("🌐 注册前置机地址...");
    async_trader.register_front(&config.trader_front_address).await?;

    // 初始化API
    info!("⚡ 初始化异步交易API...");
    async_trader.init().await?;

    // 等待连接建立
    info!("🔄 等待连接建立...");
    match async_trader.wait_connected(30).await {
        Ok(_) => info!("✅ 连接成功!"),
        Err(e) => {
            error!("❌ 连接失败: {}", e);
            return Err(e);
        }
    }

    // 创建登录请求
    let login_req = ReqUserLoginField::new(
        &config.broker_id,
        &config.investor_id,
        &config.password,
    )?
    .with_product_info("AsyncRustCTP")?;

    // 异步登录
    info!("🔐 发送登录请求...");
    match async_trader.login(&login_req, 30).await {
        Ok(login_info) => {
            info!("✅ 登录成功!");
            info!("  交易日: {:?}", std::str::from_utf8(&login_info.trading_day).unwrap_or_default().trim_end_matches('\0'));
            info!("  前置编号: {}", login_info.front_id);
            info!("  会话编号: {}", login_info.session_id);
            
            if let Ok(max_order_ref) = login_info.max_order_ref.to_utf8_string() {
                info!("  最大报单引用: {}", max_order_ref.trim_end_matches('\0'));
            }
        }
        Err(e) => {
            error!("❌ 登录失败: {}", e);
            return Err(e);
        }
    }

    info!("==========================================");

    // 等待系统就绪
    info!("⏳ 等待系统就绪...");
    sleep(Duration::from_secs(2)).await;

    // 异步查询资金账户
    info!("💰 查询资金账户...");
    let account_query = QryTradingAccountField::new(&config.broker_id, &config.investor_id)?;
    
    match async_trader.qry_trading_account(&account_query, 10).await {
        Ok(accounts) => {
            if accounts.is_empty() {
                info!("📊 无资金账户数据");
            } else {
                for account in accounts {
                    info!("✅ 资金账户查询成功");
                    if let Ok(account_id) = account.account_id.to_utf8_string() {
                        info!("  账户代码: {}", account_id.trim_end_matches('\0'));
                    }
                    info!("  可用资金: {:.2}", account.available);
                    info!("  当前余额: {:.2}", account.balance);
                    info!("  冻结保证金: {:.2}", account.frozen_margin);
                    info!("  冻结资金: {:.2}", account.frozen_cash);
                    info!("  持仓盈亏: {:.2}", account.position_profit);
                    info!("  平仓盈亏: {:.2}", account.close_profit);
                    info!("  手续费: {:.2}", account.commission);
                    info!(
                        "  风险度: {:.2}%",
                        if account.balance > 0.0 {
                            (account.curr_margin / account.balance) * 100.0
                        } else {
                            0.0
                        }
                    );
                }
            }
        }
        Err(e) => {
            error!("❌ 资金账户查询失败: {}", e);
        }
    }

    info!("==========================================");

    // 异步查询投资者持仓
    info!("📊 查询投资者持仓...");
    let position_query = QryInvestorPositionField::new(&config.broker_id, &config.investor_id)?;

    match async_trader.qry_investor_position(&position_query, 10).await {
        Ok(positions) => {
            if positions.is_empty() {
                info!("📊 无持仓记录");
            } else {
                for position in positions {
                    if let Ok(instrument_id) = position.instrument_id.to_utf8_string() {
                        let instrument = instrument_id.trim_end_matches('\0');
                        if !instrument.is_empty() {
                            info!("📊 持仓信息:");
                            info!("  合约代码: {}", instrument);
                            info!("  持仓方向: {}", position.posi_direction);
                            info!("  总持仓: {}", position.position);
                            info!("  今仓: {}", position.today_position);
                            info!("  昨仓: {}", position.yd_position);
                            info!("  持仓成本: {:.4}", position.position_cost);
                            info!("  开仓成本: {:.4}", position.open_cost);
                            info!("  持仓盈亏: {:.2}", position.position_profit);
                            info!("  上次结算价: {:.4}", position.pre_settlement_price);
                            info!("  结算价: {:.4}", position.settlement_price);
                            info!("  占用保证金: {:.2}", position.use_margin);
                            info!("  ---");
                        }
                    }
                }
            }
        }
        Err(e) => {
            error!("❌ 持仓查询失败: {}", e);
        }
    }

    info!("==========================================");

    // 启动事件处理循环
    info!("🎧 开始监听异步事件...");
    info!("💡 将监听回报事件5秒钟，然后退出");
    
    let event_timeout = Duration::from_secs(5);
    let start_time = tokio::time::Instant::now();

    while start_time.elapsed() < event_timeout {
        // 尝试接收事件，带有短超时
        match timeout(Duration::from_millis(500), async_trader.recv_event()).await {
            Ok(Some(event)) => {
                handle_async_event(event).await;
            }
            Ok(None) => {
                info!("事件通道已关闭");
                break;
            }
            Err(_) => {
                // 超时，继续循环
                continue;
            }
        }
    }

    info!("==========================================");
    info!("✅ 异步交易示例完成!");
    info!("💡 提示: 异步API适合构建高性能的交易系统");
    info!("      - 支持并发查询和操作");
    info!("      - 基于tokio异步运行时");
    info!("      - 事件驱动的响应式编程模型");

    // 释放资源
    async_trader.release().await?;
    
    Ok(())
}

/// 处理异步交易事件
async fn handle_async_event(event: AsyncTraderEvent) {
    match event {
        AsyncTraderEvent::Connected => {
            info!("🎉 异步事件: 连接成功");
        }
        AsyncTraderEvent::Disconnected(reason) => {
            warn!("❌ 异步事件: 连接断开, 原因: {}", reason);
        }
        AsyncTraderEvent::HeartBeatWarning(time_lapse) => {
            warn!("💓 异步事件: 心跳警告, 时间间隔: {}秒", time_lapse);
        }
        AsyncTraderEvent::LoginResponse { 
            user_login: _, 
            rsp_info, 
            request_id, 
            is_last 
        } => {
            info!("🔐 异步事件: 登录响应 (ID: {}, 最后: {})", request_id, is_last);
            if let Some(rsp) = rsp_info {
                if !rsp.is_success() {
                    if let Ok(error_msg) = rsp.get_error_msg() {
                        error!("  登录错误: {}", error_msg);
                    }
                }
            }
        }
        AsyncTraderEvent::OrderReturn(order) => {
            info!("📊 异步事件: 报单回报");
            if let Ok(instrument_id) = order.instrument_id.to_utf8_string() {
                info!("  合约代码: {}", instrument_id.trim_end_matches('\0'));
            }
            if let Ok(order_ref) = order.order_ref.to_utf8_string() {
                info!("  报单引用: {}", order_ref.trim_end_matches('\0'));
            }
            info!("  报单状态: {}", order.order_status);
            info!("  买卖方向: {}", order.direction);
            info!("  数量: {}", order.volume_total_original);
            info!("  价格: {}", order.limit_price);
        }
        AsyncTraderEvent::TradeReturn(trade) => {
            info!("💰 异步事件: 成交回报");
            if let Ok(instrument_id) = trade.instrument_id.to_utf8_string() {
                info!("  合约代码: {}", instrument_id.trim_end_matches('\0'));
            }
            if let Ok(trade_id) = trade.trade_id.to_utf8_string() {
                info!("  成交编号: {}", trade_id.trim_end_matches('\0'));
            }
            info!("  买卖方向: {}", trade.direction);
            info!("  成交价格: {}", trade.price);
            info!("  成交数量: {}", trade.volume);
        }
        AsyncTraderEvent::ErrorResponse { 
            rsp_info, 
            request_id, 
            is_last 
        } => {
            error!("❌ 异步事件: 错误响应 (ID: {}, 最后: {})", request_id, is_last);
            if let Some(rsp) = rsp_info {
                if let Ok(error_msg) = rsp.get_error_msg() {
                    error!("  错误信息: {}", error_msg);
                }
            }
        }
        _ => {
            // 其他事件类型的处理
            info!("📬 异步事件: {:?}", event);
        }
    }
}