//! 行情API模块
//!
//! 提供期货行情数据订阅和接收功能

use crate::api::utils::normalize_flow_path;
use crate::api::{safe_cstr_to_string, to_cstring, CtpApi};
use crate::encoding::GbkConverter;
use crate::error::{CtpError, CtpResult};
use crate::ffi::md_api::*;
use crate::ffi::{CreateMdSpiBridge, MdSpiCallbacks};
use crate::types::{ReqUserLoginField, RspInfoField, RspUserLoginField};
use std::ffi::{c_void, CString};
use std::os::raw::c_int;
use std::ptr;
use std::sync::{Arc, Mutex};

// 行情API封装
#[allow(dead_code)]
pub struct MdApi {
    // C++ API实例指针
    api_ptr: *mut c_void,
    // SPI实例指针
    spi_ptr: *mut c_void,
    // 是否已初始化
    initialized: bool,
    // 请求ID计数器
    request_id: Arc<Mutex<i32>>,
    // 回调处理器
    handler: Option<Box<dyn MdSpiHandler + Send + Sync>>,
}

// 行情SPI回调处理器特质
#[allow(unused_variables)]
pub trait MdSpiHandler {
    // 当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用
    fn on_front_connected(&mut self) {}

    // 当客户端与交易后台通信连接断开时，该方法被调用
    //
    // # 参数
    // * `reason` - 错误原因，0x2003 收到错误报文，0x2006 连接失败，0x2007 读失败，0x2008 写失败
    fn on_front_disconnected(&mut self, reason: i32) {}

    // 心跳超时警告，当长时间未收到报文时，该方法被调用
    //
    // # 参数
    // * `time_lapse` - 距离上次接收报文的时间(秒)
    fn on_heart_beat_warning(&mut self, time_lapse: i32) {}

    // 登录请求响应
    fn on_rsp_user_login(
        &mut self,
        user_login: Option<RspUserLoginField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 登出请求响应
    fn on_rsp_user_logout(
        &mut self,
        user_logout: Option<()>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 错误应答
    fn on_rsp_error(&mut self, rsp_info: Option<RspInfoField>, request_id: i32, is_last: bool) {}

    // 订阅行情应答
    fn on_rsp_sub_market_data(
        &mut self,
        specific_instrument: Option<SpecificInstrumentField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 取消订阅行情应答
    fn on_rsp_unsub_market_data(
        &mut self,
        specific_instrument: Option<SpecificInstrumentField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 深度行情通知
    fn on_rtn_depth_market_data(&mut self, market_data: DepthMarketDataField) {}

    // 询价通知
    fn on_rtn_for_quote_rsp(&mut self, for_quote_rsp: ForQuoteRspField) {}
}

// 深度行情数据 - 必须与C++结构体CThostFtdcDepthMarketDataField完全匹配
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DepthMarketDataField {
    /// 交易日
    pub trading_day: [u8; 9],
    /// 保留的无效字段
    pub reserve1: [u8; 31],
    /// 交易所代码
    pub exchange_id: [u8; 9],
    /// 保留的无效字段
    pub reserve2: [u8; 31],
    /// 最新价
    pub last_price: f64,
    /// 上次结算价
    pub pre_settlement_price: f64,
    /// 昨收盘
    pub pre_close_price: f64,
    /// 昨持仓量
    pub pre_open_interest: f64,
    /// 今开盘
    pub open_price: f64,
    /// 最高价
    pub highest_price: f64,
    /// 最低价
    pub lowest_price: f64,
    /// 数量
    pub volume: i32,
    /// 成交金额
    pub turnover: f64,
    /// 持仓量
    pub open_interest: f64,
    /// 今收盘
    pub close_price: f64,
    /// 本次结算价
    pub settlement_price: f64,
    /// 涨停板价
    pub upper_limit_price: f64,
    /// 跌停板价
    pub lower_limit_price: f64,
    /// 昨虚实度
    pub pre_delta: f64,
    /// 今虚实度
    pub curr_delta: f64,
    /// 最后修改时间
    pub update_time: [u8; 9],
    /// 最后修改毫秒
    pub update_millisec: i32,
    /// 申买价一
    pub bid_price1: f64,
    /// 申买量一
    pub bid_volume1: i32,
    /// 申卖价一
    pub ask_price1: f64,
    /// 申卖量一
    pub ask_volume1: i32,
    /// 申买价二
    pub bid_price2: f64,
    /// 申买量二
    pub bid_volume2: i32,
    /// 申卖价二
    pub ask_price2: f64,
    /// 申卖量二
    pub ask_volume2: i32,
    /// 申买价三
    pub bid_price3: f64,
    /// 申买量三
    pub bid_volume3: i32,
    /// 申卖价三
    pub ask_price3: f64,
    /// 申卖量三
    pub ask_volume3: i32,
    /// 申买价四
    pub bid_price4: f64,
    /// 申买量四
    pub bid_volume4: i32,
    /// 申卖价四
    pub ask_price4: f64,
    /// 申卖量四
    pub ask_volume4: i32,
    /// 申买价五
    pub bid_price5: f64,
    /// 申买量五
    pub bid_volume5: i32,
    /// 申卖价五
    pub ask_price5: f64,
    /// 申卖量五
    pub ask_volume5: i32,
    /// 当日均价
    pub average_price: f64,
    /// 业务日期
    pub action_day: [u8; 9],
    /// 合约代码
    pub instrument_id: [u8; 81],
    /// 合约在交易所的代码
    pub exchange_inst_id: [u8; 81],
    /// 上带价
    pub banding_upper_price: f64,
    /// 下带价
    pub banding_lower_price: f64,
}

impl Default for DepthMarketDataField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl DepthMarketDataField {
    /// 获取合约代码的UTF-8字符串
    pub fn get_instrument_id(&self) -> CtpResult<String> {
        GbkConverter::fixed_bytes_to_utf8(&self.instrument_id)
    }

    /// 获取交易所代码的UTF-8字符串
    pub fn get_exchange_id(&self) -> CtpResult<String> {
        GbkConverter::fixed_bytes_to_utf8(&self.exchange_id)
    }
}

// 指定的合约
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SpecificInstrumentField {
    /// 保留的无效字段
    pub reserve1: [u8; 31],
    /// 合约代码
    pub instrument_id: [u8; 81],
}

impl Default for SpecificInstrumentField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl SpecificInstrumentField {
    /// 获取合约代码的UTF-8字符串
    pub fn get_instrument_id(&self) -> CtpResult<String> {
        GbkConverter::fixed_bytes_to_utf8(&self.instrument_id)
    }
}

// 询价响应
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ForQuoteRspField {
    // 交易日
    pub trading_day: [u8; 9],
    // 合约代码
    pub instrument_id: [u8; 31],
    // 询价编号
    pub for_quote_ref: [u8; 13],
    // 用户代码
    pub user_id: [u8; 16],
    // 本地询价编号
    pub for_quote_local_id: [u8; 13],
    // 交易所代码
    pub exchange_id: [u8; 9],
    // 会员代码
    pub participant_id: [u8; 11],
    // 客户代码
    pub client_id: [u8; 11],
    // 合约在交易所的代码
    pub exchange_inst_id: [u8; 31],
    // 交易员代码
    pub trader_id: [u8; 21],
    // 安装编号
    pub install_id: i32,
    // 询价时间
    pub insert_time: [u8; 9],
    // 本地询价编号
    pub for_quote_local_id2: [u8; 13],
    // 业务日期
    pub action_day: [u8; 9],
}

impl Default for ForQuoteRspField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

unsafe impl Send for MdApi {}
unsafe impl Sync for MdApi {}

impl MdApi {
    // 创建行情API实例
    //
    // # 参数
    // * `flow_path` - 存储流文件的目录，默认为当前目录
    // * `is_using_udp` - 是否使用UDP协议接收多播数据
    // * `is_multicast` - 是否使用组播方式
    // * `is_production_mode` - 是否使用生产版本API，true:生产版本 false:测评版本
    pub fn new(
        flow_path: Option<&str>,
        is_using_udp: bool,
        is_multicast: bool,
        is_production_mode: bool,
    ) -> CtpResult<Self> {
        let flow_path_cstr = match flow_path {
            Some(path) => {
                let npath = normalize_flow_path(path)?;
                Some(to_cstring(npath.as_str())?)
            }
            None => None,
        };

        let flow_path_ptr = flow_path_cstr
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(ptr::null());
        let api_ptr = unsafe {
            CThostFtdcMdApi_CreateFtdcMdApi(
                flow_path_ptr,
                is_using_udp,
                is_multicast,
                is_production_mode,
            )
        };

        if api_ptr.is_null() {
            return Err(CtpError::InitializationError("创建行情API失败".to_string()));
        }

        Ok(MdApi {
            api_ptr,
            spi_ptr: ptr::null_mut(),
            initialized: false,
            request_id: Arc::new(Mutex::new(1)),
            handler: None,
        })
    }

    // 注册回调处理器
    pub fn register_spi<T>(&mut self, handler: T) -> CtpResult<()>
    where
        T: MdSpiHandler + Send + Sync + 'static,
    {
        self.handler = Some(Box::new(handler));

        // 创建回调结构体
        let callbacks = MdSpiCallbacks {
            user_data: self as *mut _ as *mut c_void,
            on_front_connected: Some(on_front_connected_callback),
            on_front_disconnected: Some(on_front_disconnected_callback),
            on_heart_beat_warning: Some(on_heart_beat_warning_callback),
            on_rsp_user_login: Some(on_rsp_user_login_callback),
            on_rsp_user_logout: Some(on_rsp_user_logout_callback),
            on_rsp_error: Some(on_rsp_error_callback),
            on_rsp_sub_market_data: Some(on_rsp_sub_market_data_callback),
            on_rsp_unsub_market_data: Some(on_rsp_unsub_market_data_callback),
            on_rtn_depth_market_data: Some(on_rtn_depth_market_data_callback),
            on_rtn_for_quote_rsp: Some(on_rtn_for_quote_rsp_callback),
        };

        // 创建SPI桥接器并注册到C++ API
        self.spi_ptr = unsafe { CreateMdSpiBridge(&callbacks) };

        if self.spi_ptr.is_null() {
            return Err(CtpError::InitializationError(
                "创建SPI桥接器失败".to_string(),
            ));
        }

        // 注册SPI到CTP API
        unsafe {
            CThostFtdcMdApi_RegisterSpi(self.api_ptr, self.spi_ptr);
        }

        Ok(())
    }

    // 用户登录请求
    pub fn req_user_login(&mut self, req: &ReqUserLoginField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }
        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcMdApi_ReqUserLogin(self.api_ptr, req as *const _ as *const c_void, request_id)
        };
        if result != 0 {
            return Err(CtpError::FfiError(format!("登录请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 登出请求
    pub fn req_user_logout(&mut self) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result =
            unsafe { CThostFtdcMdApi_ReqUserLogout(self.api_ptr, ptr::null(), request_id) };

        if result != 0 {
            return Err(CtpError::FfiError(format!("登出请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 订阅行情
    //
    // # 参数
    // * `instrument_ids` - 合约代码数组
    pub fn subscribe_market_data(&mut self, instrument_ids: &[&str]) -> CtpResult<()> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        // 转换为C字符串
        let c_strings: Result<Vec<CString>, _> = instrument_ids
            .iter()
            .map(|&id| GbkConverter::utf8_to_gb18030_cstring(id))
            .collect();

        let c_strings = c_strings?;
        let c_ptrs: Vec<*const i8> = c_strings.iter().map(|s| s.as_ptr()).collect();

        let result = unsafe {
            CThostFtdcMdApi_SubscribeMarketData(self.api_ptr, c_ptrs.as_ptr(), c_ptrs.len() as i32)
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("订阅行情失败: {}", result)));
        }

        Ok(())
    }

    // 退订行情
    //
    // # 参数
    // * `instrument_ids` - 合约代码数组
    pub fn unsubscribe_market_data(&mut self, instrument_ids: &[&str]) -> CtpResult<()> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        // 转换为C字符串
        let c_strings: Result<Vec<CString>, _> = instrument_ids
            .iter()
            .map(|&id| GbkConverter::utf8_to_gb18030_cstring(id))
            .collect();

        let c_strings = c_strings?;
        let c_ptrs: Vec<*const i8> = c_strings.iter().map(|s| s.as_ptr()).collect();

        let result = unsafe {
            CThostFtdcMdApi_UnSubscribeMarketData(
                self.api_ptr,
                c_ptrs.as_ptr(),
                c_ptrs.len() as i32,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("退订行情失败: {}", result)));
        }

        Ok(())
    }

    // 订阅询价
    pub fn subscribe_for_quote_rsp(&mut self, instrument_ids: &[&str]) -> CtpResult<()> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let c_strings: Result<Vec<CString>, _> = instrument_ids
            .iter()
            .map(|&id| GbkConverter::utf8_to_gb18030_cstring(id))
            .collect();

        let c_strings = c_strings?;
        let c_ptrs: Vec<*const i8> = c_strings.iter().map(|s| s.as_ptr()).collect();

        let result = unsafe {
            CThostFtdcMdApi_SubscribeForQuoteRsp(self.api_ptr, c_ptrs.as_ptr(), c_ptrs.len() as i32)
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("订阅询价失败: {}", result)));
        }

        Ok(())
    }

    // 退订询价
    pub fn unsubscribe_for_quote_rsp(&mut self, instrument_ids: &[&str]) -> CtpResult<()> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let c_strings: Result<Vec<CString>, _> = instrument_ids
            .iter()
            .map(|&id| GbkConverter::utf8_to_gb18030_cstring(id))
            .collect();

        let c_strings = c_strings?;
        let c_ptrs: Vec<*const i8> = c_strings.iter().map(|s| s.as_ptr()).collect();

        let result = unsafe {
            CThostFtdcMdApi_UnSubscribeForQuoteRsp(
                self.api_ptr,
                c_ptrs.as_ptr(),
                c_ptrs.len() as i32,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("退订询价失败: {}", result)));
        }

        Ok(())
    }

    // 获取下一个请求ID
    fn next_request_id(&self) -> i32 {
        let mut id = self.request_id.lock().unwrap();
        let current = *id;
        *id += 1;
        current
    }
}

impl CtpApi for MdApi {
    fn get_version() -> CtpResult<String> {
        let version_ptr = unsafe { CThostFtdcMdApi_GetApiVersion() };
        safe_cstr_to_string(version_ptr)
    }

    fn init(&mut self) -> CtpResult<()> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API指针为空".to_string()));
        }

        unsafe {
            CThostFtdcMdApi_Init(self.api_ptr);
        }

        self.initialized = true;
        Ok(())
    }

    fn release(&mut self) {
        if !self.api_ptr.is_null() {
            unsafe {
                CThostFtdcMdApi_Release(self.api_ptr);
            }
            self.api_ptr = ptr::null_mut();
        }
        self.initialized = false;
    }

    fn get_trading_day(&self) -> CtpResult<String> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let trading_day_ptr = unsafe { CThostFtdcMdApi_GetTradingDay(self.api_ptr) };

        safe_cstr_to_string(trading_day_ptr)
    }

    fn register_front(&mut self, front_address: &str) -> CtpResult<()> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let front_address_cstr = to_cstring(front_address)?;

        unsafe {
            CThostFtdcMdApi_RegisterFront(self.api_ptr, front_address_cstr.as_ptr());
        }

        Ok(())
    }

    fn join(&self) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let result = unsafe { CThostFtdcMdApi_Join(self.api_ptr) };

        Ok(result)
    }
}

impl Drop for MdApi {
    fn drop(&mut self) {
        self.release();
    }
}

// 回调函数实现
extern "C" fn on_front_connected_callback(user_data: *mut c_void) {
    unsafe {
        if let Some(api) = (user_data as *mut MdApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                handler.on_front_connected();
            }
        }
    }
}

extern "C" fn on_front_disconnected_callback(user_data: *mut c_void, reason: c_int) {
    unsafe {
        if let Some(api) = (user_data as *mut MdApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                handler.on_front_disconnected(reason);
            }
        }
    }
}

extern "C" fn on_heart_beat_warning_callback(user_data: *mut c_void, time_lapse: c_int) {
    unsafe {
        if let Some(api) = (user_data as *mut MdApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                handler.on_heart_beat_warning(time_lapse);
            }
        }
    }
}

extern "C" fn on_rsp_user_login_callback(
    user_data: *mut c_void,
    user_login: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut MdApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // 解析user_login指针
                let parsed_user_login = if !user_login.is_null() {
                    let login_ptr = user_login as *const RspUserLoginField;
                    Some((*login_ptr).clone())
                } else {
                    None
                };

                // 解析rsp_info指针
                let parsed_rsp_info = if !rsp_info.is_null() {
                    let rsp_ptr = rsp_info as *const RspInfoField;
                    Some((*rsp_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_user_login(
                    parsed_user_login,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_user_logout_callback(
    user_data: *mut c_void,
    _user_logout: *mut c_void,
    _rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut MdApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // TODO: 解析user_logout和rsp_info结构体
                handler.on_rsp_user_logout(None, None, request_id, is_last != 0);
            }
        }
    }
}

extern "C" fn on_rsp_error_callback(
    user_data: *mut c_void,
    _rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut MdApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // TODO: 解析rsp_info结构体
                handler.on_rsp_error(None, request_id, is_last != 0);
            }
        }
    }
}

extern "C" fn on_rsp_sub_market_data_callback(
    user_data: *mut c_void,
    specific_instrument: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut MdApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // 解析specific_instrument指针
                let parsed_specific_instrument = if !specific_instrument.is_null() {
                    let instrument_ptr = specific_instrument as *const SpecificInstrumentField;
                    Some((*instrument_ptr).clone())
                } else {
                    None
                };

                // 解析rsp_info指针
                let parsed_rsp_info = if !rsp_info.is_null() {
                    let rsp_ptr = rsp_info as *const RspInfoField;
                    Some((*rsp_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_sub_market_data(
                    parsed_specific_instrument,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_unsub_market_data_callback(
    user_data: *mut c_void,
    specific_instrument: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut MdApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // 解析specific_instrument指针
                let parsed_specific_instrument = if !specific_instrument.is_null() {
                    let instrument_ptr = specific_instrument as *const SpecificInstrumentField;
                    Some((*instrument_ptr).clone())
                } else {
                    None
                };

                // 解析rsp_info指针
                let parsed_rsp_info = if !rsp_info.is_null() {
                    let rsp_ptr = rsp_info as *const RspInfoField;
                    Some((*rsp_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_unsub_market_data(
                    parsed_specific_instrument,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rtn_depth_market_data_callback(user_data: *mut c_void, market_data: *mut c_void) {
    unsafe {
        if let Some(api) = (user_data as *mut MdApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // 解析market_data指针
                if !market_data.is_null() {
                    let data_ptr = market_data as *const DepthMarketDataField;
                    let parsed_data = (*data_ptr).clone();
                    handler.on_rtn_depth_market_data(parsed_data);
                }
            }
        }
    }
}

extern "C" fn on_rtn_for_quote_rsp_callback(user_data: *mut c_void, _for_quote_rsp: *mut c_void) {
    unsafe {
        if let Some(api) = (user_data as *mut MdApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // TODO: 解析for_quote_rsp结构体
                // 临时创建一个空的for_quote_rsp
                let temp_data = ForQuoteRspField::default();
                handler.on_rtn_for_quote_rsp(temp_data);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        // 测试版本获取函数的调用
        match MdApi::get_version() {
            Ok(version) => eprintln!("版本: {}", version),
            Err(e) => eprintln!("获取版本失败: {}", e),
        }
    }
}
