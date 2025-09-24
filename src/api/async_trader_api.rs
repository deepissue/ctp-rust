//! 异步交易API模块
//!
//! 基于同步TraderApi提供异步封装，使用tokio实现

use crate::api::trader_api::{
    InputOrderField, InvestorPositionField, OrderField, ReqAuthenticateField, RspAuthenticateField,
    TradeField, TraderApi, TraderSpiHandler, TradingAccountField,
};
use crate::api::CtpApi;
use crate::error::{CtpError, CtpResult};
use crate::types::{
    InputOrderActionField, QryInvestorPositionField, QryTradingAccountField, ReqUserLoginField,
    RspInfoField, RspUserLoginField,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, Notify};
use tokio::time::{timeout, Duration};
use tracing::{debug, error, warn};

/// 异步交易事件类型
#[derive(Debug, Clone)]
pub enum AsyncTraderEvent {
    /// 连接成功
    Connected,
    /// 连接断开
    Disconnected(i32),
    /// 心跳超时警告
    HeartBeatWarning(i32),
    /// 认证响应
    AuthenticateResponse {
        rsp_authenticate: Option<RspAuthenticateField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
    /// 登录响应
    LoginResponse {
        user_login: Option<RspUserLoginField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
    /// 登出响应
    LogoutResponse {
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
    /// 报单录入响应
    OrderInsertResponse {
        input_order: Option<InputOrderField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
    /// 报单操作响应
    OrderActionResponse {
        input_order_action: Option<InputOrderActionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
    /// 查询交易账户响应
    QryTradingAccountResponse {
        trading_account: Option<TradingAccountField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
    /// 查询投资者持仓响应
    QryInvestorPositionResponse {
        investor_position: Option<InvestorPositionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
    /// 查询报单响应
    QryOrderResponse {
        order: Option<OrderField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
    /// 查询成交响应
    QryTradeResponse {
        trade: Option<TradeField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
    /// 报单回报
    OrderReturn(OrderField),
    /// 成交回报
    TradeReturn(TradeField),
    /// 错误响应
    ErrorResponse {
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
}

/// 异步交易API状态
#[derive(Debug, Clone, Default)]
pub struct AsyncTraderState {
    pub connected: bool,
    pub authenticated: bool,
    pub logged_in: bool,
    pub auth_info: Option<RspAuthenticateField>,
    pub login_info: Option<RspUserLoginField>,
}

/// 待处理的异步请求
#[derive(Debug, Clone)]
struct PendingRequest {
    notify: Arc<Notify>,
    response_data: Arc<Mutex<Option<AsyncTraderEvent>>>,
}

/// 异步交易API适配器
pub struct AsyncTraderApi {
    /// 内部同步API
    inner: Arc<Mutex<TraderApi>>,
    /// 事件发送器
    event_sender: mpsc::UnboundedSender<AsyncTraderEvent>,
    /// 事件接收器
    event_receiver: Arc<Mutex<mpsc::UnboundedReceiver<AsyncTraderEvent>>>,
    /// 当前状态
    state: Arc<Mutex<AsyncTraderState>>,
    /// 连接通知
    connected_notify: Arc<Notify>,
    /// 认证通知
    auth_notify: Arc<Notify>,
    /// 登录通知
    login_notify: Arc<Notify>,
    /// 待处理的请求映射 (request_id -> PendingRequest)
    pending_requests: Arc<Mutex<HashMap<i32, PendingRequest>>>,
}

impl AsyncTraderApi {
    /// 创建异步交易API实例
    pub async fn new(flow_path: Option<&str>, is_production_mode: Option<bool>) -> CtpResult<Self> {
        let trader_api = TraderApi::new(flow_path, is_production_mode)?;
        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        Ok(Self {
            inner: Arc::new(Mutex::new(trader_api)),
            event_sender,
            event_receiver: Arc::new(Mutex::new(event_receiver)),
            state: Arc::new(Mutex::new(AsyncTraderState::default())),
            connected_notify: Arc::new(Notify::new()),
            auth_notify: Arc::new(Notify::new()),
            login_notify: Arc::new(Notify::new()),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// 注册前置机地址
    pub async fn register_front(&self, front_address: &str) -> CtpResult<()> {
        let mut api = self.inner.lock().await;
        api.register_front(front_address)
    }

    /// 初始化API
    pub async fn init(&self) -> CtpResult<()> {
        let mut api = self.inner.lock().await;

        // 创建异步事件处理器
        let handler = AsyncTraderHandler::new(
            self.event_sender.clone(),
            self.state.clone(),
            self.connected_notify.clone(),
            self.auth_notify.clone(),
            self.login_notify.clone(),
            self.pending_requests.clone(),
        );

        // 注册处理器
        api.register_spi(handler)?;

        // 初始化
        api.init()
    }

    /// 等待连接建立（带超时）
    pub async fn wait_connected(&self, timeout_secs: u64) -> CtpResult<()> {
        let state = self.state.lock().await;
        if state.connected {
            return Ok(());
        }
        drop(state);

        match timeout(
            Duration::from_secs(timeout_secs),
            self.connected_notify.notified(),
        )
        .await
        {
            Ok(_) => {
                let state = self.state.lock().await;
                if state.connected {
                    Ok(())
                } else {
                    Err(CtpError::InitializationError("连接失败".to_string()))
                }
            }
            Err(_) => Err(CtpError::InitializationError("连接超时".to_string())),
        }
    }

    /// 异步认证
    pub async fn authenticate(
        &self,
        req: &ReqAuthenticateField,
        timeout_secs: u64,
    ) -> CtpResult<RspAuthenticateField> {
        let mut api = self.inner.lock().await;
        let request_id = api.req_authenticate(req)?;
        drop(api);

        let pending_request = PendingRequest {
            notify: Arc::new(Notify::new()),
            response_data: Arc::new(Mutex::new(None)),
        };

        // 注册待处理请求
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(request_id, pending_request.clone());
        }

        match timeout(
            Duration::from_secs(timeout_secs),
            pending_request.notify.notified(),
        )
        .await
        {
            Ok(_) => {
                let state = self.state.lock().await;
                if let Some(auth_info) = &state.auth_info {
                    Ok(auth_info.clone())
                } else {
                    Err(CtpError::InitializationError("认证失败".to_string()))
                }
            }
            Err(_) => {
                // 清理待处理请求
                let mut pending = self.pending_requests.lock().await;
                pending.remove(&request_id);
                Err(CtpError::InitializationError("认证超时".to_string()))
            }
        }
    }

    /// 异步登录
    pub async fn login(
        &self,
        req: &ReqUserLoginField,
        timeout_secs: u64,
    ) -> CtpResult<RspUserLoginField> {
        let mut api = self.inner.lock().await;
        let request_id = api.req_user_login(req)?;
        drop(api);

        let pending_request = PendingRequest {
            notify: Arc::new(Notify::new()),
            response_data: Arc::new(Mutex::new(None)),
        };

        // 注册待处理请求
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(request_id, pending_request.clone());
        }

        match timeout(
            Duration::from_secs(timeout_secs),
            pending_request.notify.notified(),
        )
        .await
        {
            Ok(_) => {
                let state = self.state.lock().await;
                if let Some(login_info) = &state.login_info {
                    Ok(login_info.clone())
                } else {
                    Err(CtpError::InitializationError("登录失败".to_string()))
                }
            }
            Err(_) => {
                // 清理待处理请求
                let mut pending = self.pending_requests.lock().await;
                pending.remove(&request_id);
                Err(CtpError::InitializationError("登录超时".to_string()))
            }
        }
    }

    /// 异步报单录入
    pub async fn order_insert(
        &self,
        req: &InputOrderField,
        timeout_secs: u64,
    ) -> CtpResult<AsyncTraderEvent> {
        let mut api = self.inner.lock().await;
        let request_id = api.req_order_insert(req)?;
        drop(api);

        self.wait_for_response(request_id, timeout_secs).await
    }

    /// 异步报单操作
    pub async fn order_action(
        &self,
        req: &InputOrderActionField,
        timeout_secs: u64,
    ) -> CtpResult<AsyncTraderEvent> {
        let mut api = self.inner.lock().await;
        let request_id = api.req_order_action(req)?;
        drop(api);

        self.wait_for_response(request_id, timeout_secs).await
    }

    /// 异步查询交易账户
    pub async fn qry_trading_account(
        &self,
        req: &QryTradingAccountField,
        timeout_secs: u64,
    ) -> CtpResult<Vec<TradingAccountField>> {
        let mut api = self.inner.lock().await;
        let request_id = api.req_qry_trading_account(req)?;
        drop(api);

        // 收集所有响应数据
        let mut results = Vec::new();
        let mut is_finished = false;

        let start_time = std::time::Instant::now();
        let timeout_duration = Duration::from_secs(timeout_secs);

        while !is_finished && start_time.elapsed() < timeout_duration {
            if let Some(event) = self.recv_event().await {
                match event {
                    AsyncTraderEvent::QryTradingAccountResponse {
                        trading_account,
                        rsp_info,
                        request_id: resp_id,
                        is_last,
                    } if resp_id == request_id => {
                        if let Some(rsp) = rsp_info {
                            if !rsp.is_success() {
                                return Err(CtpError::BusinessError(
                                    rsp.error_id,
                                    rsp.get_error_msg().unwrap_or_default(),
                                ));
                            }
                        }
                        if let Some(account) = trading_account {
                            results.push(account);
                        }
                        is_finished = is_last;
                    }
                    _ => continue,
                }
            }
        }

        if is_finished {
            Ok(results)
        } else {
            Err(CtpError::InitializationError("查询超时".to_string()))
        }
    }

    /// 异步查询投资者持仓
    pub async fn qry_investor_position(
        &self,
        req: &QryInvestorPositionField,
        timeout_secs: u64,
    ) -> CtpResult<Vec<InvestorPositionField>> {
        let mut api = self.inner.lock().await;
        let request_id = api.req_qry_investor_position(req)?;
        drop(api);

        // 收集所有响应数据
        let mut results = Vec::new();
        let mut is_finished = false;

        let start_time = std::time::Instant::now();
        let timeout_duration = Duration::from_secs(timeout_secs);

        while !is_finished && start_time.elapsed() < timeout_duration {
            if let Some(event) = self.recv_event().await {
                match event {
                    AsyncTraderEvent::QryInvestorPositionResponse {
                        investor_position,
                        rsp_info,
                        request_id: resp_id,
                        is_last,
                    } if resp_id == request_id => {
                        if let Some(rsp) = rsp_info {
                            if !rsp.is_success() {
                                return Err(CtpError::BusinessError(
                                    rsp.error_id,
                                    rsp.get_error_msg().unwrap_or_default(),
                                ));
                            }
                        }
                        if let Some(position) = investor_position {
                            results.push(position);
                        }
                        is_finished = is_last;
                    }
                    _ => continue,
                }
            }
        }

        if is_finished {
            Ok(results)
        } else {
            Err(CtpError::InitializationError("查询超时".to_string()))
        }
    }

    /// 等待指定请求的响应
    async fn wait_for_response(
        &self,
        request_id: i32,
        timeout_secs: u64,
    ) -> CtpResult<AsyncTraderEvent> {
        let pending_request = PendingRequest {
            notify: Arc::new(Notify::new()),
            response_data: Arc::new(Mutex::new(None)),
        };

        // 注册待处理请求
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(request_id, pending_request.clone());
        }

        match timeout(
            Duration::from_secs(timeout_secs),
            pending_request.notify.notified(),
        )
        .await
        {
            Ok(_) => {
                let response_data = pending_request.response_data.lock().await;
                if let Some(event) = response_data.as_ref() {
                    Ok(event.clone())
                } else {
                    Err(CtpError::InitializationError("响应数据为空".to_string()))
                }
            }
            Err(_) => {
                // 清理待处理请求
                let mut pending = self.pending_requests.lock().await;
                pending.remove(&request_id);
                Err(CtpError::InitializationError("请求超时".to_string()))
            }
        }
    }

    /// 接收下一个事件
    pub async fn recv_event(&self) -> Option<AsyncTraderEvent> {
        let mut receiver = self.event_receiver.lock().await;
        receiver.recv().await
    }

    /// 尝试接收事件（非阻塞）
    pub async fn try_recv_event(&self) -> Result<AsyncTraderEvent, mpsc::error::TryRecvError> {
        let mut receiver = self.event_receiver.lock().await;
        receiver.try_recv()
    }

    /// 获取当前状态
    pub async fn get_state(&self) -> AsyncTraderState {
        self.state.lock().await.clone()
    }

    /// 释放资源
    pub async fn release(&self) -> CtpResult<()> {
        let mut api = self.inner.lock().await;
        api.release();
        Ok(())
    }
}

/// 异步事件处理器
#[derive(Clone)]
struct AsyncTraderHandler {
    event_sender: mpsc::UnboundedSender<AsyncTraderEvent>,
    state: Arc<Mutex<AsyncTraderState>>,
    connected_notify: Arc<Notify>,
    auth_notify: Arc<Notify>,
    login_notify: Arc<Notify>,
    pending_requests: Arc<Mutex<HashMap<i32, PendingRequest>>>,
}

impl AsyncTraderHandler {
    fn new(
        event_sender: mpsc::UnboundedSender<AsyncTraderEvent>,
        state: Arc<Mutex<AsyncTraderState>>,
        connected_notify: Arc<Notify>,
        auth_notify: Arc<Notify>,
        login_notify: Arc<Notify>,
        pending_requests: Arc<Mutex<HashMap<i32, PendingRequest>>>,
    ) -> Self {
        Self {
            event_sender,
            state,
            connected_notify,
            auth_notify,
            login_notify,
            pending_requests,
        }
    }

    /// 通知待处理的请求
    fn notify_pending_request(&self, request_id: i32, event: AsyncTraderEvent) {
        if let Ok(mut pending) = self.pending_requests.try_lock() {
            if let Some(req) = pending.remove(&request_id) {
                // 设置响应数据
                if let Ok(mut data) = req.response_data.try_lock() {
                    *data = Some(event);
                }
                // 通知等待者
                req.notify.notify_waiters();
            }
        }
    }
}

impl TraderSpiHandler for AsyncTraderHandler {
    fn on_front_connected(&mut self) {
        debug!("异步交易API: 连接成功");

        // 更新状态
        if let Ok(mut state) = self.state.try_lock() {
            state.connected = true;
        }

        // 通知等待者
        self.connected_notify.notify_waiters();

        // 发送事件
        let _ = self.event_sender.send(AsyncTraderEvent::Connected);
    }

    fn on_front_disconnected(&mut self, reason: i32) {
        warn!("异步交易API: 连接断开, 原因: {}", reason);

        // 更新状态
        if let Ok(mut state) = self.state.try_lock() {
            state.connected = false;
            state.authenticated = false;
            state.logged_in = false;
            state.auth_info = None;
            state.login_info = None;
        }

        // 发送事件
        let _ = self
            .event_sender
            .send(AsyncTraderEvent::Disconnected(reason));
    }

    fn on_heart_beat_warning(&mut self, time_lapse: i32) {
        warn!("异步交易API: 心跳超时警告, 时间间隔: {}秒", time_lapse);
        let _ = self
            .event_sender
            .send(AsyncTraderEvent::HeartBeatWarning(time_lapse));
    }

    fn on_rsp_authenticate(
        &mut self,
        rsp_authenticate: Option<RspAuthenticateField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步交易API: 收到认证响应");

        // 检查是否成功
        let success = rsp_info.as_ref().map_or(true, |info| info.is_success());

        if success && is_last {
            if let Ok(mut state) = self.state.try_lock() {
                state.authenticated = true;
                state.auth_info = rsp_authenticate.clone();
            }
            // 通知认证完成
            self.auth_notify.notify_waiters();
        }

        // 发送事件
        let event = AsyncTraderEvent::AuthenticateResponse {
            rsp_authenticate,
            rsp_info,
            request_id,
            is_last,
        };

        let _ = self.event_sender.send(event.clone());

        // 通知待处理的请求
        self.notify_pending_request(request_id, event);
    }

    fn on_rsp_user_login(
        &mut self,
        user_login: Option<RspUserLoginField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步交易API: 收到登录响应");

        // 检查是否成功
        let success = rsp_info.as_ref().map_or(true, |info| info.is_success());

        if success && is_last {
            if let Ok(mut state) = self.state.try_lock() {
                state.logged_in = true;
                state.login_info = user_login.clone();
            }
            // 通知登录完成
            self.login_notify.notify_waiters();
        }

        // 发送事件
        let event = AsyncTraderEvent::LoginResponse {
            user_login,
            rsp_info,
            request_id,
            is_last,
        };

        let _ = self.event_sender.send(event.clone());

        // 通知待处理的请求
        self.notify_pending_request(request_id, event);
    }

    fn on_rsp_user_logout(
        &mut self,
        _user_logout: Option<()>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步交易API: 收到登出响应");

        if is_last {
            if let Ok(mut state) = self.state.try_lock() {
                state.logged_in = false;
                state.login_info = None;
            }
        }

        let event = AsyncTraderEvent::LogoutResponse {
            rsp_info,
            request_id,
            is_last,
        };

        let _ = self.event_sender.send(event.clone());

        // 通知待处理的请求
        self.notify_pending_request(request_id, event);
    }

    fn on_rsp_error(&mut self, rsp_info: Option<RspInfoField>, request_id: i32, is_last: bool) {
        error!("异步交易API: 收到错误响应");

        let event = AsyncTraderEvent::ErrorResponse {
            rsp_info,
            request_id,
            is_last,
        };

        let _ = self.event_sender.send(event.clone());

        // 通知待处理的请求
        self.notify_pending_request(request_id, event);
    }

    fn on_rsp_order_insert(
        &mut self,
        input_order: Option<InputOrderField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步交易API: 收到报单录入响应");

        let event = AsyncTraderEvent::OrderInsertResponse {
            input_order,
            rsp_info,
            request_id,
            is_last,
        };

        let _ = self.event_sender.send(event.clone());

        // 通知待处理的请求
        self.notify_pending_request(request_id, event);
    }

    fn on_rsp_order_action(
        &mut self,
        input_order_action: Option<InputOrderActionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步交易API: 收到报单操作响应");

        let event = AsyncTraderEvent::OrderActionResponse {
            input_order_action,
            rsp_info,
            request_id,
            is_last,
        };

        let _ = self.event_sender.send(event.clone());

        // 通知待处理的请求
        self.notify_pending_request(request_id, event);
    }

    fn on_rsp_qry_trading_account(
        &mut self,
        trading_account: Option<TradingAccountField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步交易API: 收到查询交易账户响应");

        let _ = self
            .event_sender
            .send(AsyncTraderEvent::QryTradingAccountResponse {
                trading_account,
                rsp_info,
                request_id,
                is_last,
            });
    }

    fn on_rsp_qry_investor_position(
        &mut self,
        investor_position: Option<InvestorPositionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步交易API: 收到查询投资者持仓响应");

        let _ = self
            .event_sender
            .send(AsyncTraderEvent::QryInvestorPositionResponse {
                investor_position,
                rsp_info,
                request_id,
                is_last,
            });
    }

    fn on_rsp_qry_order(
        &mut self,
        order: Option<OrderField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步交易API: 收到查询报单响应");

        let _ = self.event_sender.send(AsyncTraderEvent::QryOrderResponse {
            order,
            rsp_info,
            request_id,
            is_last,
        });
    }

    fn on_rsp_qry_trade(
        &mut self,
        trade: Option<TradeField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步交易API: 收到查询成交响应");

        let _ = self.event_sender.send(AsyncTraderEvent::QryTradeResponse {
            trade,
            rsp_info,
            request_id,
            is_last,
        });
    }

    fn on_rtn_order(&mut self, order: OrderField) {
        debug!("异步交易API: 收到报单回报");
        let _ = self.event_sender.send(AsyncTraderEvent::OrderReturn(order));
    }

    fn on_rtn_trade(&mut self, trade: TradeField) {
        debug!("异步交易API: 收到成交回报");
        let _ = self.event_sender.send(AsyncTraderEvent::TradeReturn(trade));
    }
}
