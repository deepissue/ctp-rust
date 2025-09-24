//! 基础交易功能示例
//!
//! 展示如何使用CTP Rust SDK进行期货交易操作

use ctp_rust::api::trader_api::{
    InputOrderField, InvestorPositionField, OrderField, RspAuthenticateField, TradeField,
    TraderSpiHandler, TradingAccountField,
};
use ctp_rust::api::{CtpApi, TraderApi};
use ctp_rust::types::{
    QryInvestorPositionField, QryTradingAccountField, ReqUserLoginField, RspInfoField,
    RspUserLoginField,
};
use ctp_rust::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tracing::{error, info, warn};
use tracing_subscriber;

/// 交易处理器状态
#[derive(Debug, Clone)]
struct TraderState {
    connected: bool,
    authenticated: bool,
    logged_in: bool,
    front_id: i32,
    session_id: i32,
    max_order_ref: String,
    account_queried: bool,
    position_queried: bool,
}

impl Default for TraderState {
    fn default() -> Self {
        Self {
            connected: false,
            authenticated: false,
            logged_in: false,
            front_id: 0,
            session_id: 0,
            max_order_ref: String::new(),
            account_queried: false,
            position_queried: false,
        }
    }
}

/// 交易事件处理器
#[derive(Clone)]
#[allow(dead_code)]
struct TraderHandler {
    state: Arc<Mutex<TraderState>>,
    order_count: Arc<Mutex<u32>>,
    config: Option<CtpConfig>,
    api: Option<std::sync::Weak<std::sync::Mutex<TraderApi>>>,
}
#[allow(dead_code)]
impl TraderHandler {
    fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(TraderState::default())),
            order_count: Arc::new(Mutex::new(0)),
            config: None,
            api: None,
        }
    }

    fn with_config_and_api(
        mut self,
        config: CtpConfig,
        api_weak: std::sync::Weak<std::sync::Mutex<TraderApi>>,
    ) -> Self {
        self.config = Some(config);
        self.api = Some(api_weak);
        self
    }

    fn get_state(&self) -> TraderState {
        self.state.lock().unwrap().clone()
    }

    fn get_next_order_ref(&self) -> String {
        let mut count = self.order_count.lock().unwrap();
        *count += 1;
        format!("{:012}", *count)
    }
}

impl TraderSpiHandler for TraderHandler {
    fn on_front_connected(&mut self) {
        info!("🎉 SUCCESS: 已成功连接到交易服务器!");
        info!("🔗 连接状态: TCP连接已建立");
        info!("📡 网络通道: 前置机通信链路正常");
        info!("⏰ 连接时间: {:?}", std::time::SystemTime::now());

        self.state.lock().unwrap().connected = true;

        // 立即在回调中发送登录请求
        if let (Some(config), Some(api_weak)) = (&self.config, &self.api) {
            if let Some(api_arc) = api_weak.upgrade() {
                info!("🔐 连接成功，立即发送登录请求...");
                info!("📋 登录参数:");
                info!("   • 经纪公司: {}", config.broker_id);
                info!("   • 投资者账号: {}", config.investor_id);
                info!("   • 密码长度: {} 字符", config.password);

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
                    Ok(_) => {
                        info!("📤 登录请求已发送，等待服务器响应");
                    }
                    Err(e) => {
                        error!("❌ 发送登录请求失败: {}", e);
                    }
                }
            } else {
                error!("❌ API实例已失效，无法发送登录请求");
            }
        } else {
            error!("❌ 配置信息或API引用缺失，无法发送登录请求");
        }
    }

    fn on_front_disconnected(&mut self, reason: i32) {
        warn!("❌ DISCONNECT: 与交易服务器连接断开!");
        warn!("🔌 断开原因代码: {}", reason);
        warn!(
            "📋 断开原因说明: {}",
            match reason {
                0x1001 => "网络读取失败",
                0x1002 => "网络写入失败",
                0x2001 => "接收心跳超时",
                0x2002 => "发送心跳失败",
                0x2003 => "收到错误报文",
                0x2004 => "网络连接已断开",
                0x2005 => "网络连接超时",
                0x2006 => "网络连接被拒绝",
                _ => "未知原因",
            }
        );
        warn!("⚠️  正在重置连接状态...");

        let mut state = self.state.lock().unwrap();
        state.connected = false;
        state.authenticated = false;
        state.logged_in = false;
    }

    fn on_heart_beat_warning(&mut self, time_lapse: i32) {
        warn!("💓 心跳警告: 距离上次心跳已过 {} 毫秒", time_lapse);
        warn!("⚠️  网络状况可能不稳定，请检查网络连接");
    }

    fn on_rsp_authenticate(
        &mut self,
        rsp_authenticate: Option<RspAuthenticateField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        info!(
            "收到认证响应 - 请求ID: {}, 是否最后: {}",
            request_id, is_last
        );

        if let Some(rsp) = rsp_info {
            if rsp.is_success() {
                info!("✓ 客户端认证成功");
                self.state.lock().unwrap().authenticated = true;

                if let Some(auth_info) = rsp_authenticate {
                    if let Ok(broker_id) = auth_info.broker_id.to_utf8_string() {
                        info!("  经纪公司: {}", broker_id.trim_end_matches('\0'));
                    }
                    if let Ok(user_id) = auth_info.user_id.to_utf8_string() {
                        info!("  用户代码: {}", user_id.trim_end_matches('\0'));
                    }
                }
            } else {
                if let Ok(error_msg) = rsp.get_error_msg() {
                    info!("✗ 认证失败: {}", error_msg.trim_end_matches('\0'));
                }
            }
        }
    }

    fn on_rsp_user_login(
        &mut self,
        user_login: Option<RspUserLoginField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        info!(
            "收到登录响应 - 请求ID: {}, 是否最后: {}",
            request_id, is_last
        );

        if let Some(rsp) = rsp_info {
            if rsp.is_success() {
                info!("✓ 交易登录成功");

                if let Some(login_info) = user_login {
                    let mut state = self.state.lock().unwrap();
                    state.logged_in = true;
                    state.front_id = login_info.front_id;
                    state.session_id = login_info.session_id;

                    if let Ok(max_order_ref) = login_info.max_order_ref.to_utf8_string() {
                        state.max_order_ref = max_order_ref.trim_end_matches('\0').to_string();
                    }

                    info!(
                        "  交易日: {:?}",
                        std::str::from_utf8(&login_info.trading_day)
                            .unwrap_or_default()
                            .trim_end_matches('\0')
                    );
                    info!("  前置编号: {}", login_info.front_id);
                    info!("  会话编号: {}", login_info.session_id);
                    info!("  最大报单引用: {}", state.max_order_ref);
                }
            } else {
                if let Ok(error_msg) = rsp.get_error_msg() {
                    info!("✗ 登录失败: {}", error_msg.trim_end_matches('\0'));
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
        info!(
            "收到登出响应 - 请求ID: {}, 是否最后: {}",
            request_id, is_last
        );

        if let Some(rsp) = rsp_info {
            if rsp.is_success() {
                info!("✓ 登出成功");
            } else {
                if let Ok(error_msg) = rsp.get_error_msg() {
                    info!("✗ 登出失败: {}", error_msg.trim_end_matches('\0'));
                }
            }
        }

        let mut state = self.state.lock().unwrap();
        state.logged_in = false;
        state.authenticated = false;
    }

    fn on_rsp_order_insert(
        &mut self,
        input_order: Option<InputOrderField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        info!(
            "收到报单录入响应 - 请求ID: {}, 是否最后: {}",
            request_id, is_last
        );

        if let Some(rsp) = rsp_info {
            if rsp.is_success() {
                info!("✓ 报单录入成功");

                if let Some(order) = input_order {
                    if let Ok(instrument_id) = order.instrument_id.to_utf8_string() {
                        info!("  合约代码: {}", instrument_id.trim_end_matches('\0'));
                    }
                    if let Ok(order_ref) = order.order_ref.to_utf8_string() {
                        info!("  报单引用: {}", order_ref.trim_end_matches('\0'));
                    }
                    info!("  买卖方向: {}", order.direction);
                    info!("  开平标志: {:?}", order.comb_offset_flag);
                    info!("  数量: {}", order.volume_total_original);
                    info!("  价格: {}", order.limit_price);
                }
            } else {
                if let Ok(error_msg) = rsp.get_error_msg() {
                    info!("✗ 报单录入失败: {}", error_msg.trim_end_matches('\0'));
                }
            }
        }
    }

    fn on_rsp_order_action(
        &mut self,
        input_order_action: Option<InputOrderActionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        info!(
            "收到报单操作响应 - 请求ID: {}, 是否最后: {}",
            request_id, is_last
        );

        if let Some(rsp) = rsp_info {
            if rsp.is_success() {
                info!("✓ 报单操作成功");

                if let Some(action) = input_order_action {
                    if let Ok(instrument_id) = action.instrument_id.to_utf8_string() {
                        info!("  合约代码: {}", instrument_id.trim_end_matches('\0'));
                    }
                    if let Ok(order_ref) = action.order_ref.to_utf8_string() {
                        info!("  报单引用: {}", order_ref.trim_end_matches('\0'));
                    }
                    info!("  操作标志: {}", action.action_flag);
                }
            } else {
                if let Ok(error_msg) = rsp.get_error_msg() {
                    info!("✗ 报单操作失败: {}", error_msg.trim_end_matches('\0'));
                }
            }
        }
    }

    fn on_rtn_order(&mut self, order: OrderField) {
        info!("📊 收到报单回报");

        if let Ok(instrument_id) = order.instrument_id.to_utf8_string() {
            info!("  合约代码: {}", instrument_id.trim_end_matches('\0'));
        }

        if let Ok(order_ref) = order.order_ref.to_utf8_string() {
            info!("  报单引用: {}", order_ref.trim_end_matches('\0'));
        }

        if let Ok(order_sys_id) = order.order_sys_id.to_utf8_string() {
            info!("  报单编号: {:?}", order_sys_id.trim_end_matches('\0'));
        }

        info!("  报单状态: {}", order.order_status);
        info!("  买卖方向: {}", order.direction);
        info!("  开平标志: {:?}", order.comb_offset_flag);
        info!("  数量: {}", order.volume_total_original);
        info!("  价格: {}", order.limit_price);
        info!("  今成交数量: {}", order.volume_traded);
        info!("  剩余数量: {}", order.volume_total);

        if let Ok(status_msg) = order.status_msg.to_utf8_string() {
            let status_msg = status_msg.trim_end_matches('\0');
            if !status_msg.is_empty() {
                info!("  状态信息: {}", status_msg);
            }
        }

        info!("  ---");
    }

    fn on_rtn_trade(&mut self, trade: TradeField) {
        info!("💰 收到成交回报");

        if let Ok(instrument_id) = trade.instrument_id.to_utf8_string() {
            info!("  合约代码: {}", instrument_id.trim_end_matches('\0'));
        }

        if let Ok(order_ref) = trade.order_ref.to_utf8_string() {
            info!("  报单引用: {}", order_ref.trim_end_matches('\0'));
        }

        if let Ok(trade_id) = trade.trade_id.to_utf8_string() {
            info!("  成交编号: {}", trade_id.trim_end_matches('\0'));
        }

        info!("  买卖方向: {}", trade.direction);
        info!("  开平标志: {}", trade.offset_flag);
        info!("  成交价格: {}", trade.price);
        info!("  成交数量: {}", trade.volume);

        if let Ok(trade_date) = trade.trade_date.to_utf8_string() {
            info!("  成交日期: {}", trade_date.trim_end_matches('\0'));
        }

        if let Ok(trade_time) = trade.trade_time.to_utf8_string() {
            info!("  成交时间: {}", trade_time.trim_end_matches('\0'));
        }

        info!("  ---");
    }

    fn on_rsp_qry_trading_account(
        &mut self,
        trading_account: Option<TradingAccountField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        let status_msg = match &rsp_info {
            None => "成功".to_string(),
            Some(info) => format!("错误码: {}, 错误信息: {:?}", info.error_id, info.error_msg),
        };
        info!(
            "收到资金账户查询响应 - 请求ID: {}, 是否最后: {}, 状态: {}",
            request_id, is_last, status_msg
        );
        if let Some(account) = trading_account {
            info!("✓ 资金账户查询成功");
            if is_last {
                self.state.lock().unwrap().account_queried = true;
            }
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

        info!("==========================================");
    }

    fn on_rsp_qry_investor_position(
        &mut self,
        investor_position: Option<InvestorPositionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        info!(
            "收到持仓查询响应 - 请求ID: {}, 是否最后: {}",
            request_id, is_last
        );
        if !is_last {
            return;
        }
        if let Some(rsp_info) = rsp_info {
            if rsp_info.error_id != 0 {
                if let Ok(error_msg) = rsp_info.get_error_msg() {
                    error!("持仓查询失败: {}", error_msg);
                }
                return;
            }
        }
        if let Some(position) = investor_position {
            self.state.lock().unwrap().position_queried = true;
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
        } else {
            info!("📊 无持仓记录");
        }
    }
}

fn main() -> CtpResult<()> {
    // 初始化tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_file(true) // 显示文件名
        .with_line_number(true) // 显示行号
        .with_target(true) // 显示模块路径
        .init();

    info!("🚀 CTP交易功能示例启动");
    info!("==========================================");

    // 详细的环境检查
    info!("🔍 环境检查:");
    info!("   • 操作系统: {}", std::env::consts::OS);
    info!("   • 架构: {}", std::env::consts::ARCH);
    info!(
        "   • DYLD_LIBRARY_PATH: {:?}",
        std::env::var("DYLD_LIBRARY_PATH")
    );
    info!(
        "   • LD_LIBRARY_PATH: {:?}",
        std::env::var("LD_LIBRARY_PATH")
    );
    info!("   • 工作目录: {:?}", std::env::current_dir());

    // 检查CTP库文件是否存在
    let dylib_paths = [
        "libs/ctp/lib/mac64/libthostmduserapi_se.dylib",
        "libs/ctp/lib/mac64/libthosttraderapi_se.dylib",
    ];

    for path in &dylib_paths {
        if std::path::Path::new(path).exists() {
            info!("   ✅ CTP库文件存在: {}", path);
        } else {
            warn!("   ❌ CTP库文件缺失: {}", path);
        }
    }
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

    // 创建交易API实例并用Arc包装
    info!("📡 创建交易API...");
    let trader_api = TraderApi::new(Some(&config.flow_path), Some(true))?;
    let api_arc = Arc::new(Mutex::new(trader_api));
    let api_weak = Arc::downgrade(&api_arc);

    // 创建事件处理器并传入API弱引用
    let handler = TraderHandler::new().with_config_and_api(config.clone(), api_weak);
    let handler_state = handler.state.clone();

    // 注册事件处理器
    api_arc.lock().unwrap().register_spi(handler.clone())?;

    // 注册前置机地址
    info!("🌐 注册前置机地址...");
    info!("📍 目标地址: {}", config.trader_front_address);

    // 解析地址信息
    let addr_without_protocol = config
        .trader_front_address
        .strip_prefix("tcp://")
        .unwrap_or(&config.trader_front_address);
    if let Some((host, port)) = addr_without_protocol.split_once(':') {
        info!("🖥️  服务器主机: {}", host);
        info!("🔌 服务器端口: {}", port);

        // 尝试解析IP地址
        use std::net::ToSocketAddrs;
        match format!("{}:{}", host, port).to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    info!("🌍 解析的IP地址: {}", addr.ip());
                }
            }
            Err(e) => {
                warn!("⚠️  DNS解析失败: {}", e);
            }
        }
    }

    api_arc
        .lock()
        .unwrap()
        .register_front(&config.trader_front_address)?;

    info!("✅ 前置机地址注册完成");

    // 初始化API
    info!("⚡ 初始化交易API...");
    info!("📂 流文件路径: {}", config.flow_path);
    info!("🔧 正在启动CTP内核...");
    info!("🌐 网络环境检查:");
    info!(
        "   • DYLD_LIBRARY_PATH: {:?}",
        std::env::var("DYLD_LIBRARY_PATH")
    );
    info!("   • 当前工作目录: {:?}", std::env::current_dir());
    info!("   • 系统时间: {:?}", std::time::SystemTime::now());
    api_arc.lock().unwrap().init()?;
    info!("✅ API初始化完成，开始建立连接");

    // 等待连接建立
    info!("🔄 正在连接到交易服务器...");
    info!("⏳ 连接超时设置: 30秒");

    let mut connection_attempts = 0;
    for i in 0..30 {
        thread::sleep(Duration::from_secs(1));
        connection_attempts += 1;

        if handler_state.lock().unwrap().connected {
            info!("✅ 连接成功! 耗时: {}秒", connection_attempts);
            break;
        }

        // 每5秒输出一次连接状态
        if i % 5 == 4 {
            info!("\n⏱️  连接中... ({}秒)", connection_attempts);
            info!("🔍 连接状态检查:");
            info!("   • 网络连接是否正常");
            info!("   • 服务器地址: {}", config.trader_front_address);
            info!("   • CTP动态库路径是否正确");
            info!("   • 防火墙是否阻挡连接");
            info!("   • 服务器是否在维护时间");
            info!("   • 是否在交易时段 (工作日 09:00-15:00, 21:00-02:30)");

            // 尝试简单网络测试
            if let Some((host, _)) = config.trader_front_address.split_once(':') {
                use std::process::Command;
                match Command::new("ping").arg("-c").arg("1").arg(host).output() {
                    Ok(output) => {
                        if output.status.success() {
                            info!("   ✅ 主机 {} 网络可达", host);
                        } else {
                            warn!("   ❌ 主机 {} 网络不可达", host);
                        }
                    }
                    Err(_) => {
                        info!("   ❓ 无法测试网络连通性 (ping命令不可用)");
                    }
                }
            }
        } else {
            print!(".");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }

        if i >= 29 {
            error!("\n\n❌ 连接超时失败!");
            error!("🔍 详细故障排查建议:");
            error!("   1. 检查网络连接是否正常");
            error!(
                "   2. 确认服务器地址 {} 是否正确",
                config.trader_front_address
            );
            error!(
                "   3. 验证DYLD_LIBRARY_PATH设置: {:?}",
                std::env::var("DYLD_LIBRARY_PATH")
            );
            error!("   4. 检查CTP动态库文件是否存在:");
            error!("      - libs/ctp/lib/mac64/libthostmduserapi_se.dylib");
            error!("      - libs/ctp/lib/mac64/libthosttraderapi_se.dylib");
            error!("   5. 检查防火墙是否阻挡了连接");
            error!("   6. 确认CTP服务器是否正常运行");
            error!("   7. 检查是否在交易时间段 (工作日 09:00-15:00, 21:00-02:30)");
            error!(
                "   8. 尝试使用telnet测试连接: telnet {} [port]",
                config.trader_front_address.split(':').next().unwrap_or("")
            );
            error!("   9. 检查是否有模拟环境的账号密码");
            error!("  10. 检查broker_id和investor_id是否正确");
            error!("");
            error!("💡 调试命令:");
            error!("   export RUST_LOG=debug");
            error!("   DYLD_LIBRARY_PATH=libs/ctp/lib/mac64:$DYLD_LIBRARY_PATH cargo run --example trader_basic");
            return Ok(());
        }
    }

    // 客户端认证（如果需要）
    // if !auth_code.is_empty() && !app_id.is_empty() {
    //     info!("🔐 发送认证请求...");

    //     // 这里需要根据实际的认证字段结构来构造请求
    //     // let auth_req = ReqAuthenticateField {
    //     //     broker_id: BrokerIdType::from_utf8_string(broker_id)?,
    //     //     user_id: UserIdType::from_utf8_string(investor_id)?,
    //     //     auth_code: AuthCodeType::from_utf8_string(auth_code)?,
    //     //     app_id: AppIdType::from_utf8_string(app_id)?,
    //     //     ..Default::default()
    //     // };

    //     // trader_api.req_authenticate(&auth_req)?;

    //     // 等待认证完成
    //     info!("⏳ 等待认证完成...");
    //     for i in 0..10 {
    //         thread::sleep(Duration::from_secs(1));
    //         if handler_state.lock().unwrap().authenticated {
    //             break;
    //         }
    //         if i >= 9 {
    //             info!("✗ 认证超时");
    //             return Ok(());
    //         }
    //         print!(".");
    //     }
    //     info!();
    // }

    // 等待登录完成（登录请求在on_front_connected回调中自动发送）
    info!("⏳ 等待连接和登录完成...");
    info!("💡 登录请求将在连接成功回调中自动发送");

    for i in 0..30 {
        thread::sleep(Duration::from_secs(1));
        let current_state = handler.get_state();

        if current_state.logged_in {
            info!("✅ 登录成功!");
            break;
        }

        if i >= 29 {
            error!("\n❌ 登录超时!");
            error!("🔍 登录失败可能原因:");
            error!("   • 网络连接问题");
            error!("   • 投资者账号或密码错误");
            error!("   • 账号被锁定或未激活");
            error!("   • 超过最大登录次数限制");
            error!("   • 服务器繁忙或维护中");
            error!("   • 交易时间段限制");
            error!("💡 建议检查环境变量配置:");
            error!("   export CTP_BROKER_ID=your_broker_id");
            error!("   export CTP_INVESTOR_ID=your_investor_id");
            error!("   export CTP_PASSWORD=your_password");
            error!("   export CTP_TRADER_FRONT=tcp://your_server:port");
            return Ok(());
        }

        // 每5秒显示一次状态
        if i % 5 == 4 {
            if current_state.connected {
                info!("\n🔄 已连接，等待登录响应... ({}秒)", i + 1);
            } else {
                info!("\n🔄 等待连接建立... ({}秒)", i + 1);
            }
        } else {
            print!(".");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
    }

    let state = handler_state.lock().unwrap().clone();
    info!("🎉 交易功能已就绪！");
    info!("  前置编号: {}", state.front_id);
    info!("  会话编号: {}", state.session_id);
    info!("  最大报单引用: {}", state.max_order_ref);
    info!("==========================================");

    // 等待一下再查询，确保系统就绪
    info!("⏳ 等待系统就绪...");
    thread::sleep(Duration::from_secs(2));

    // 查询资金账户
    info!("💰 查询资金账户...");
    let account_query = QryTradingAccountField::new(&config.broker_id, &config.investor_id)?;
    info!("发送资金账户查询请求...");

    match api_arc
        .lock()
        .unwrap()
        .req_qry_trading_account(&account_query)
    {
        Ok(request_id) => {
            info!("资金账户查询请求已发送，请求ID: {}", request_id);
        }
        Err(e) => {
            error!("发送资金账户查询请求失败: {}", e);
            return Err(e);
        }
    }

    // 等待资金账户查询完成
    for i in 0..10 {
        thread::sleep(Duration::from_millis(500));
        if handler_state.lock().unwrap().account_queried {
            break;
        }
        if i >= 9 {
            warn!("资金账户查询超时");
        }
    }

    // 查询持仓
    info!("📊 查询投资者持仓...");
    let position_query = QryInvestorPositionField::new(&config.broker_id, &config.investor_id)?;
    api_arc
        .lock()
        .unwrap()
        .req_qry_investor_position(&position_query)?;

    // 等待持仓查询完成
    for i in 0..10 {
        thread::sleep(Duration::from_millis(500));
        if handler_state.lock().unwrap().position_queried {
            break;
        }
        if i >= 9 {
            warn!("持仓查询超时");
        }
    }

    info!("✅ 查询完成");
    info!("💡 提示: 查询功能演示完成，可根据需要添加更多交易功能");
    info!("      包括: 查询报单、下单、撤单等操作");

    // 运行一段时间
    info!("⏳ 程序将在10秒后退出...");
    thread::sleep(Duration::from_secs(10));

    // 登出
    info!("📤 发送登出请求...");
    // let logout_req = (); // 简化的登出请求
    // api_arc.lock().unwrap().req_user_logout(&logout_req)?;

    thread::sleep(Duration::from_secs(2));

    info!("✅ 示例程序结束");
    Ok(())
}
