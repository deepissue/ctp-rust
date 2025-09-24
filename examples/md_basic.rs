//! 基础行情订阅示例
//!
//! 展示如何使用CTP Rust SDK订阅期货合约行情数据

use ctp_rust::api::md_api::{
    DepthMarketDataField, ForQuoteRspField, MdSpiHandler, SpecificInstrumentField,
};
use ctp_rust::api::{CtpApi, MdApi};
use ctp_rust::types::{ReqUserLoginField, RspInfoField, RspUserLoginField};
use ctp_rust::*;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use tracing_subscriber;

/// CTP事件类型
#[derive(Debug, Clone)]
enum CtpEvent {
    /// 连接建立
    FrontConnected,
    /// 连接断开
    FrontDisconnected(i32),
    /// 登录成功
    LoginSuccess(RspUserLoginField),
    /// 登录失败
    LoginFailed(String),
    /// 订阅成功
    SubscribeSuccess(String),
    /// 订阅失败
    SubscribeFailed(String),
    /// 行情数据
    MarketData(DepthMarketDataField),
    /// 询价应答
    ForQuoteRsp(ForQuoteRspField),
}

/// 行情处理器 - 纯事件发送器
#[derive(Clone)]
struct MarketDataHandler {
    /// 事件发送器
    event_sender: Sender<CtpEvent>,
}

impl MarketDataHandler {
    fn new(event_sender: Sender<CtpEvent>) -> Self {
        Self { event_sender }
    }
}

impl MdSpiHandler for MarketDataHandler {
    fn on_front_connected(&mut self) {
        info!("✓ 已连接到行情服务器");
        // 只发送事件，不处理业务逻辑
        if let Err(e) = self.event_sender.send(CtpEvent::FrontConnected) {
            error!("发送连接事件失败: {}", e);
        }
    }

    fn on_front_disconnected(&mut self, reason: i32) {
        warn!("✗ 与行情服务器断开连接，原因代码: {}", reason);
        // 只发送事件，不处理业务逻辑
        if let Err(e) = self.event_sender.send(CtpEvent::FrontDisconnected(reason)) {
            error!("发送断连事件失败: {}", e);
        }
    }

    fn on_rsp_user_login(
        &mut self,
        user_login: Option<RspUserLoginField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!(
            "收到登录响应 - 请求ID: {}, 是否最后: {}",
            request_id, is_last
        );

        if let Some(rsp) = rsp_info {
            if rsp.is_success() {
                info!("✓ 登录成功");
                if let Some(login_info) = user_login {
                    info!(
                        "  交易日: {:?}",
                        std::str::from_utf8(&login_info.trading_day)
                            .unwrap_or_default()
                            .trim_end_matches('\0')
                    );
                    info!("  前置编号: {}", login_info.front_id);
                    info!("  会话编号: {}", login_info.session_id);

                    // 只发送事件，不处理业务逻辑
                    if let Err(e) = self.event_sender.send(CtpEvent::LoginSuccess(login_info)) {
                        error!("发送登录成功事件失败: {}", e);
                    }
                } else {
                    // 即使没有login_info也认为登录成功
                    if let Err(e) =
                        self.event_sender
                            .send(CtpEvent::LoginSuccess(RspUserLoginField {
                                trading_day: [0; 9],
                                login_time: [0; 9],
                                broker_id: [0; 11],
                                user_id: [0; 16],
                                system_name: [0; 41],
                                front_id: 0,
                                session_id: 0,
                                max_order_ref: [0; 13],
                                shfe_time: [0; 9],
                                dce_time: [0; 9],
                                czce_time: [0; 9],
                                ffex_time: [0; 9],
                                ine_time: [0; 9],
                            }))
                    {
                        error!("发送登录成功事件失败: {}", e);
                    }
                }
            } else {
                let error_msg = if let Ok(msg) = rsp.get_error_msg() {
                    msg.trim_end_matches('\0').to_string()
                } else {
                    "未知登录错误".to_string()
                };
                error!("✗ 登录失败: {}", error_msg);

                // 只发送事件，不处理业务逻辑
                if let Err(e) = self.event_sender.send(CtpEvent::LoginFailed(error_msg)) {
                    error!("发送登录失败事件失败: {}", e);
                }
            }
        }
    }

    fn on_rsp_user_logout(
        &mut self,
        _user_logout: Option<()>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!(
            "收到登出响应 - 请求ID: {}, 是否最后: {}",
            request_id, is_last
        );

        if let Some(rsp) = rsp_info {
            if rsp.is_success() {
                info!("✓ 登出成功");
            } else {
                if let Ok(error_msg) = rsp.get_error_msg() {
                    error!("✗ 登出失败: {}", error_msg.trim_end_matches('\0'));
                }
            }
        }
        // 注意：在事件驱动架构中，状态管理应该在事件循环中处理
        // 这里可以发送一个登出事件，但当前示例中暂不实现
    }

    fn on_rsp_sub_market_data(
        &mut self,
        specific_instrument: Option<SpecificInstrumentField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!(
            "收到订阅行情响应 - 请求ID: {}, 是否最后: {}",
            request_id, is_last
        );

        if let Some(rsp) = rsp_info {
            if rsp.is_success() {
                let instrument_id = if let Some(instrument) = specific_instrument {
                    if let Ok(id) = instrument.get_instrument_id() {
                        let id = id.trim_end_matches('\0').to_string();
                        info!("✓ 成功订阅合约: {}", id);
                        id
                    } else {
                        info!("✓ 订阅成功");
                        "未知合约".to_string()
                    }
                } else {
                    info!("✓ 订阅成功");
                    "未知合约".to_string()
                };

                // 只发送事件，不处理业务逻辑
                if let Err(e) = self
                    .event_sender
                    .send(CtpEvent::SubscribeSuccess(instrument_id))
                {
                    error!("发送订阅成功事件失败: {}", e);
                }
            } else {
                let error_msg = if let Ok(msg) = rsp.get_error_msg() {
                    msg.trim_end_matches('\0').to_string()
                } else {
                    "未知订阅错误".to_string()
                };
                error!("✗ 订阅失败: {}", error_msg);

                // 只发送事件，不处理业务逻辑
                if let Err(e) = self.event_sender.send(CtpEvent::SubscribeFailed(error_msg)) {
                    error!("发送订阅失败事件失败: {}", e);
                }
            }
        }
    }

    fn on_rsp_unsub_market_data(
        &mut self,
        specific_instrument: Option<SpecificInstrumentField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!(
            "收到取消订阅响应 - 请求ID: {}, 是否最后: {}",
            request_id, is_last
        );

        if let Some(rsp) = rsp_info {
            if rsp.is_success() {
                if let Some(instrument) = specific_instrument {
                    if let Ok(instrument_id) = instrument.get_instrument_id() {
                        info!(
                            "✓ 成功取消订阅合约: {}",
                            instrument_id.trim_end_matches('\0')
                        );
                    } else {
                        info!("✓ 取消订阅成功");
                    }
                } else {
                    info!("✓ 取消订阅成功");
                }
            } else {
                if let Ok(error_msg) = rsp.get_error_msg() {
                    error!("✗ 取消订阅失败: {}", error_msg.trim_end_matches('\0'));
                }
            }
        }
    }

    fn on_rtn_depth_market_data(&mut self, market_data: DepthMarketDataField) {
        // 只发送事件，不处理业务逻辑
        if let Err(e) = self.event_sender.send(CtpEvent::MarketData(market_data)) {
            error!("发送行情数据事件失败: {}", e);
        }
    }

    fn on_rtn_for_quote_rsp(&mut self, for_quote_rsp: ForQuoteRspField) {
        // 只发送事件，不处理业务逻辑
        if let Err(e) = self.event_sender.send(CtpEvent::ForQuoteRsp(for_quote_rsp)) {
            error!("发送询价应答事件失败: {}", e);
        }
    }
}

/// 应用状态
#[derive(Debug)]
struct AppState {
    connected: bool,
    logged_in: bool,
    subscribed: bool,
    start_time: std::time::Instant,
}

impl AppState {
    fn new() -> Self {
        Self {
            connected: false,
            logged_in: false,
            subscribed: false,
            start_time: std::time::Instant::now(),
        }
    }
}

/// 处理行情数据显示
fn display_market_data(market_data: &DepthMarketDataField) {
    if let Ok(instrument_id) = market_data.get_instrument_id() {
        let instrument_id = instrument_id.trim_end_matches('\0');
        if !instrument_id.is_empty() {
            // 构建行情数据字符串
            let mut market_info = format!(
                "📈 {}: 最新价{:.4} 成交量{} 持仓量{}",
                instrument_id,
                market_data.last_price,
                market_data.volume,
                market_data.open_interest
            );

            if market_data.bid_price1 > 0.0 {
                market_info.push_str(&format!(
                    " 买一{:.4}({})",
                    market_data.bid_price1, market_data.bid_volume1
                ));
            }
            if market_data.ask_price1 > 0.0 {
                market_info.push_str(&format!(
                    " 卖一{:.4}({})",
                    market_data.ask_price1, market_data.ask_volume1
                ));
            }

            // 解析交易时间
            if let Ok(update_time) = market_data.update_time.to_utf8_string() {
                let update_time = update_time.trim_end_matches('\0');
                if !update_time.is_empty() {
                    market_info.push_str(&format!(
                        " 时间{}.{:03}",
                        update_time, market_data.update_millisec
                    ));
                }
            }

            info!("{}", market_info);
        }
    }
}

fn main() -> CtpResult<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .init();

    info!("🚀 CTP行情订阅示例启动 (事件驱动架构)");
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

    // 创建事件通道
    let (event_sender, event_receiver): (Sender<CtpEvent>, Receiver<CtpEvent>) = mpsc::channel();

    // 创建行情API实例
    info!("📡 创建行情API...");
    let md_api = MdApi::new(Some(&config.flow_path), false, false, true)?;
    let api_arc = Arc::new(Mutex::new(md_api));

    // 创建事件处理器
    let handler = MarketDataHandler::new(event_sender);

    // 注册事件处理器
    api_arc.lock().unwrap().register_spi(handler)?;

    // 注册前置机地址
    info!("🔗 注册前置机地址...");
    api_arc
        .lock()
        .unwrap()
        .register_front(&config.md_front_address)?;

    // 初始化API
    info!("⚡ 初始化API...");
    api_arc.lock().unwrap().init()?;

    // 应用状态
    let mut app_state = AppState::new();

    info!("🔄 开始事件循环...");
    info!("==========================================");

    // 事件循环
    loop {
        match event_receiver.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => {
                match event {
                    CtpEvent::FrontConnected => {
                        info!("📡 收到连接事件");
                        app_state.connected = true;

                        // 发送登录请求
                        info!("🔐 连接成功，发送登录请求...");
                        thread::sleep(Duration::from_millis(1000)); // 稍微等待

                        let login_result = (|| -> Result<(), Box<dyn std::error::Error>> {
                            let login_req = ReqUserLoginField::new(
                                config.broker_id.as_str(),
                                config.investor_id.as_str(),
                                config.password.as_str(),
                            )?
                            .with_product_info("RustCTP")?;

                            api_arc.lock().unwrap().req_user_login(&login_req)?;
                            Ok(())
                        })();

                        match login_result {
                            Ok(_) => info!("📤 登录请求已发送"),
                            Err(e) => error!("❌ 发送登录请求失败: {}", e),
                        }
                    }
                    CtpEvent::FrontDisconnected(reason) => {
                        warn!("💔 连接断开，原因: {}", reason);
                        app_state.connected = false;
                        app_state.logged_in = false;
                        app_state.subscribed = false;
                    }
                    CtpEvent::LoginSuccess(_login_info) => {
                        info!("✅ 登录成功!");
                        app_state.logged_in = true;

                        // 发送订阅请求
                        info!("📊 登录成功，发送订阅请求...");
                        let instruments_refs: Vec<&str> =
                            config.instruments.iter().map(|s| s.as_str()).collect();
                        match api_arc
                            .lock()
                            .unwrap()
                            .subscribe_market_data(&instruments_refs)
                        {
                            Ok(_) => info!("📤 订阅请求已发送"),
                            Err(e) => error!("❌ 发送订阅请求失败: {}", e),
                        }
                    }
                    CtpEvent::LoginFailed(error_msg) => {
                        error!("❌ 登录失败: {}", error_msg);
                        break;
                    }
                    CtpEvent::SubscribeSuccess(instrument_id) => {
                        info!("✅ 订阅成功: {}", instrument_id);
                        app_state.subscribed = true;
                        info!("🎉 开始接收行情数据...");
                    }
                    CtpEvent::SubscribeFailed(error_msg) => {
                        error!("❌ 订阅失败: {}", error_msg);
                    }
                    CtpEvent::MarketData(market_data) => {
                        display_market_data(&market_data);
                    }
                    CtpEvent::ForQuoteRsp(for_quote_rsp) => {
                        if let Ok(instrument_id) = for_quote_rsp.instrument_id.to_utf8_string() {
                            let instrument_id = instrument_id.trim_end_matches('\0');
                            if !instrument_id.is_empty() {
                                info!("💬 收到询价应答: {}", instrument_id);
                            }
                        }
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // 检查是否运行超过30秒
                if app_state.subscribed && app_state.start_time.elapsed() > Duration::from_secs(30)
                {
                    info!("⏰ 运行时间到达30秒，准备退出...");
                    break;
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                warn!("📡 事件通道断开，退出事件循环");
                break;
            }
        }
    }

    // 清理工作
    if app_state.subscribed {
        info!("📤 取消订阅行情数据...");
        let instruments_refs: Vec<&str> = config.instruments.iter().map(|s| s.as_str()).collect();
        if let Err(e) = api_arc
            .lock()
            .unwrap()
            .unsubscribe_market_data(&instruments_refs)
        {
            error!("取消订阅失败: {}", e);
        }
        thread::sleep(Duration::from_secs(1));
    }

    info!("✅ 示例程序结束");
    Ok(())
}
