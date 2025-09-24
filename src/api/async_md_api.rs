//! 异步行情API模块
//!
//! 基于同步MdApi提供异步封装，使用tokio实现

use crate::api::md_api::{
    DepthMarketDataField, ForQuoteRspField, MdApi, MdSpiHandler, SpecificInstrumentField,
};
use crate::api::CtpApi;
use crate::error::{CtpError, CtpResult};
use crate::types::{ReqUserLoginField, RspInfoField, RspUserLoginField};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, Notify};
use tokio::time::{timeout, Duration};
use tracing::{debug, error, warn};

/// 异步事件类型
#[derive(Debug, Clone)]
pub enum AsyncMdEvent {
    /// 连接成功
    Connected,
    /// 连接断开
    Disconnected(i32),
    /// 心跳超时警告
    HeartBeatWarning(i32),
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
    /// 错误响应
    ErrorResponse {
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
    /// 订阅行情响应
    SubMarketDataResponse {
        specific_instrument: Option<SpecificInstrumentField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
    /// 取消订阅行情响应
    UnsubMarketDataResponse {
        specific_instrument: Option<SpecificInstrumentField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    },
    /// 深度行情数据
    DepthMarketData(DepthMarketDataField),
    /// 询价响应
    ForQuoteResponse(ForQuoteRspField),
}

/// 异步行情API状态
#[derive(Debug, Clone, Default)]
pub struct AsyncMdState {
    pub connected: bool,
    pub logged_in: bool,
    pub login_info: Option<RspUserLoginField>,
}

/// 异步行情API适配器
pub struct AsyncMdApi {
    /// 内部同步API
    inner: Arc<Mutex<MdApi>>,
    /// 事件发送器
    event_sender: mpsc::UnboundedSender<AsyncMdEvent>,
    /// 事件接收器
    event_receiver: Arc<Mutex<mpsc::UnboundedReceiver<AsyncMdEvent>>>,
    /// 当前状态
    state: Arc<Mutex<AsyncMdState>>,
    /// 连接通知
    connected_notify: Arc<Notify>,
    /// 登录通知
    login_notify: Arc<Notify>,
}

impl AsyncMdApi {
    /// 创建异步行情API实例
    pub async fn new(
        flow_path: Option<&str>,
        is_using_udp: bool,
        is_multicast: bool,
        is_production_mode: Option<bool>,
    ) -> CtpResult<Self> {
        let md_api = MdApi::new(
            flow_path,
            is_using_udp,
            is_multicast,
            is_production_mode.unwrap_or(false),
        )?;
        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        Ok(Self {
            inner: Arc::new(Mutex::new(md_api)),
            event_sender,
            event_receiver: Arc::new(Mutex::new(event_receiver)),
            state: Arc::new(Mutex::new(AsyncMdState::default())),
            connected_notify: Arc::new(Notify::new()),
            login_notify: Arc::new(Notify::new()),
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
        let handler = AsyncMdHandler::new(
            self.event_sender.clone(),
            self.state.clone(),
            self.connected_notify.clone(),
            self.login_notify.clone(),
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

    /// 异步登录
    pub async fn login(
        &self,
        req: &ReqUserLoginField,
        timeout_secs: u64,
    ) -> CtpResult<RspUserLoginField> {
        let mut api = self.inner.lock().await;
        api.req_user_login(req)?;
        drop(api);

        match timeout(
            Duration::from_secs(timeout_secs),
            self.login_notify.notified(),
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
            Err(_) => Err(CtpError::InitializationError("登录超时".to_string())),
        }
    }

    /// 订阅行情数据
    pub async fn subscribe_market_data(&self, instrument_ids: &[&str]) -> CtpResult<()> {
        let mut api = self.inner.lock().await;
        api.subscribe_market_data(instrument_ids)
    }

    /// 取消订阅行情数据
    pub async fn unsubscribe_market_data(&self, instrument_ids: &[&str]) -> CtpResult<()> {
        let mut api = self.inner.lock().await;
        api.unsubscribe_market_data(instrument_ids)
    }

    /// 接收下一个事件
    pub async fn recv_event(&self) -> Option<AsyncMdEvent> {
        let mut receiver = self.event_receiver.lock().await;
        receiver.recv().await
    }

    /// 尝试接收事件（非阻塞）
    pub async fn try_recv_event(&self) -> Result<AsyncMdEvent, mpsc::error::TryRecvError> {
        let mut receiver = self.event_receiver.lock().await;
        receiver.try_recv()
    }

    /// 获取当前状态
    pub async fn get_state(&self) -> AsyncMdState {
        self.state.lock().await.clone()
    }
}

/// 异步事件处理器
#[derive(Clone)]
struct AsyncMdHandler {
    event_sender: mpsc::UnboundedSender<AsyncMdEvent>,
    state: Arc<Mutex<AsyncMdState>>,
    connected_notify: Arc<Notify>,
    login_notify: Arc<Notify>,
}

impl AsyncMdHandler {
    fn new(
        event_sender: mpsc::UnboundedSender<AsyncMdEvent>,
        state: Arc<Mutex<AsyncMdState>>,
        connected_notify: Arc<Notify>,
        login_notify: Arc<Notify>,
    ) -> Self {
        Self {
            event_sender,
            state,
            connected_notify,
            login_notify,
        }
    }
}

impl MdSpiHandler for AsyncMdHandler {
    fn on_front_connected(&mut self) {
        debug!("异步API: 连接成功");

        // 更新状态
        if let Ok(mut state) = self.state.try_lock() {
            state.connected = true;
        }

        // 通知等待者
        self.connected_notify.notify_waiters();

        // 发送事件
        let _ = self.event_sender.send(AsyncMdEvent::Connected);
    }

    fn on_front_disconnected(&mut self, reason: i32) {
        warn!("异步API: 连接断开, 原因: {}", reason);

        // 更新状态
        if let Ok(mut state) = self.state.try_lock() {
            state.connected = false;
            state.logged_in = false;
            state.login_info = None;
        }

        // 发送事件
        let _ = self.event_sender.send(AsyncMdEvent::Disconnected(reason));
    }

    fn on_heart_beat_warning(&mut self, time_lapse: i32) {
        warn!("异步API: 心跳超时警告, 时间间隔: {}秒", time_lapse);
        let _ = self
            .event_sender
            .send(AsyncMdEvent::HeartBeatWarning(time_lapse));
    }

    fn on_rsp_user_login(
        &mut self,
        user_login: Option<RspUserLoginField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步API: 收到登录响应");

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
        let _ = self.event_sender.send(AsyncMdEvent::LoginResponse {
            user_login,
            rsp_info,
            request_id,
            is_last,
        });
    }

    fn on_rsp_user_logout(
        &mut self,
        _user_logout: Option<()>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步API: 收到登出响应");

        if is_last {
            if let Ok(mut state) = self.state.try_lock() {
                state.logged_in = false;
                state.login_info = None;
            }
        }

        let _ = self.event_sender.send(AsyncMdEvent::LogoutResponse {
            rsp_info,
            request_id,
            is_last,
        });
    }

    fn on_rsp_error(&mut self, rsp_info: Option<RspInfoField>, request_id: i32, is_last: bool) {
        error!("异步API: 收到错误响应");
        let _ = self.event_sender.send(AsyncMdEvent::ErrorResponse {
            rsp_info,
            request_id,
            is_last,
        });
    }

    fn on_rsp_sub_market_data(
        &mut self,
        specific_instrument: Option<SpecificInstrumentField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步API: 收到订阅行情响应");
        let _ = self.event_sender.send(AsyncMdEvent::SubMarketDataResponse {
            specific_instrument,
            rsp_info,
            request_id,
            is_last,
        });
    }

    fn on_rsp_unsub_market_data(
        &mut self,
        specific_instrument: Option<SpecificInstrumentField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("异步API: 收到取消订阅响应");
        let _ = self
            .event_sender
            .send(AsyncMdEvent::UnsubMarketDataResponse {
                specific_instrument,
                rsp_info,
                request_id,
                is_last,
            });
    }

    fn on_rtn_depth_market_data(&mut self, market_data: DepthMarketDataField) {
        // 这里不使用debug，因为行情数据量大
        let _ = self
            .event_sender
            .send(AsyncMdEvent::DepthMarketData(market_data));
    }

    fn on_rtn_for_quote_rsp(&mut self, for_quote_rsp: ForQuoteRspField) {
        debug!("异步API: 收到询价响应");
        let _ = self
            .event_sender
            .send(AsyncMdEvent::ForQuoteResponse(for_quote_rsp));
    }
}
