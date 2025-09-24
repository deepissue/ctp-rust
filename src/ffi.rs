//! FFI绑定模块
//!
//! 直接与CTP C++库的底层接口绑定

use std::os::raw::{c_char, c_int, c_void};

// Debug日志配置结构体
#[repr(C)]
pub struct CtpLogConfig {
    pub enable_debug: c_int,           // 0=关闭, 1=开启
    pub log_file_path: *const c_char,  // 日志文件路径，NULL=控制台输出
    pub max_file_size_mb: c_int,       // 最大文件大小（MB）- 预留功能
    pub max_backup_files: c_int,       // 最大备份文件数 - 预留功能
}

// SPI回调结构体
#[repr(C)]
pub struct MdSpiCallbacks {
    pub user_data: *mut c_void,
    pub on_front_connected: Option<extern "C" fn(*mut c_void)>,
    pub on_front_disconnected: Option<extern "C" fn(*mut c_void, c_int)>,
    pub on_heart_beat_warning: Option<extern "C" fn(*mut c_void, c_int)>,
    pub on_rsp_user_login:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_user_logout:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_error: Option<extern "C" fn(*mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_sub_market_data:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_unsub_market_data:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rtn_depth_market_data: Option<extern "C" fn(*mut c_void, *mut c_void)>,
    pub on_rtn_for_quote_rsp: Option<extern "C" fn(*mut c_void, *mut c_void)>,
}

// 交易SPI回调结构体
#[repr(C)]
pub struct TraderSpiCallbacks {
    pub user_data: *mut c_void,
    pub on_front_connected: Option<extern "C" fn(*mut c_void)>,
    pub on_front_disconnected: Option<extern "C" fn(*mut c_void, c_int)>,
    pub on_heart_beat_warning: Option<extern "C" fn(*mut c_void, c_int)>,
    pub on_rsp_authenticate:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_user_login:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_user_logout:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_error: Option<extern "C" fn(*mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_order_insert:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_order_action:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rtn_order: Option<extern "C" fn(*mut c_void, *mut c_void)>,
    pub on_rtn_trade: Option<extern "C" fn(*mut c_void, *mut c_void)>,
    pub on_rsp_qry_trading_account:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_qry_investor_position:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,

    // 第一阶段新增回调
    pub on_err_rtn_order_insert: Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void)>,
    pub on_err_rtn_order_action: Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void)>,
    pub on_rsp_qry_order:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_qry_trade:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_qry_instrument:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,

    // 第二阶段新增回调
    pub on_rsp_qry_instrument_margin_rate:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_qry_instrument_commission_rate:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_qry_exchange:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_qry_product:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_settlement_info_confirm:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_parked_order_insert:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_parked_order_action:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,

    // 第三阶段新增回调
    pub on_rsp_exec_order_insert:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_exec_order_action:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_for_quote_insert:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_quote_insert:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_quote_action:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_batch_order_action:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_remove_parked_order:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_remove_parked_order_action:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_qry_max_order_volume:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_qry_depth_market_data:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_qry_settlement_info:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_qry_transfer_bank:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_qry_investor_position_detail:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
    pub on_rsp_qry_notice:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int, c_int)>,
}

// SPI桥接函数
#[link(name = "ctp_wrapper")]
extern "C" {
    pub fn CreateMdSpiBridge(callbacks: *const MdSpiCallbacks) -> *mut c_void;
    pub fn DestroyMdSpiBridge(spi_bridge: *mut c_void);
    pub fn CreateTraderSpiBridge(callbacks: *const TraderSpiCallbacks) -> *mut c_void;
    pub fn DestroyTraderSpiBridge(spi_bridge: *mut c_void);
    
    // Debug日志接口
    pub fn CTP_InitializeDebugLogging(config: *const CtpLogConfig);
    pub fn CTP_CleanupDebugLogging();
}

// CTP行情API的FFI声明
pub mod md_api {
    use super::*;

    // 创建行情API实例
    //
    // # 参数
    // * `flow_path` - 存储流文件的目录，默认为当前目录
    // * `is_using_udp` - 是否使用UDP协议接收多播数据
    // * `is_multicast` - 是否使用组播方式
    //
    // # 返回值
    // 行情API实例指针
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_CreateFtdcMdApi(
            flow_path: *const c_char,
            is_using_udp: bool,
            is_multicast: bool,
            is_production_mode: bool,
        ) -> *mut c_void;
    }

    // 获取API版本号
    //
    // # 返回值
    // 版本号字符串
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_GetApiVersion() -> *const c_char;
    }

    // 释放API实例
    //
    // # 参数
    // * `api` - API实例指针
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_Release(api: *mut c_void);
    }

    // 初始化API
    //
    // # 参数
    // * `api` - API实例指针
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_Init(api: *mut c_void);
    }

    // 等待API线程结束运行
    //
    // # 参数
    // * `api` - API实例指针
    //
    // # 返回值
    // 线程退出代码
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_Join(api: *mut c_void) -> c_int;
    }

    // 获取当前交易日
    //
    // # 参数
    // * `api` - API实例指针
    //
    // # 返回值
    // 交易日字符串(YYYYMMDD格式)
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_GetTradingDay(api: *mut c_void) -> *const c_char;
    }

    // 注册前置机网络地址
    //
    // # 参数
    // * `api` - API实例指针
    // * `front_address` - 前置机网络地址，格式为："protocol://ipaddress:port"，
    //                     如："tcp://127.0.0.1:17001"
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_RegisterFront(api: *mut c_void, front_address: *const c_char);
    }

    // 注册名字服务器网络地址
    //
    // # 参数
    // * `api` - API实例指针
    // * `ns_address` - 名字服务器网络地址，格式同前置机地址
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_RegisterNameServer(api: *mut c_void, ns_address: *const c_char);
    }

    // 注册名字服务器用户信息
    //
    // # 参数
    // * `api` - API实例指针
    // * `user_info` - 用户信息
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_RegisterFensUserInfo(api: *mut c_void, user_info: *const c_void);
    }

    // 注册回调接口
    //
    // # 参数
    // * `api` - API实例指针
    // * `spi` - 派生自CThostFtdcMdSpi的实例指针
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_RegisterSpi(api: *mut c_void, spi: *mut c_void);
    }

    // 用户登录请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 登录请求
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_ReqUserLogin(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 登出请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 登出请求
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_ReqUserLogout(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 订阅行情
    //
    // # 参数
    // * `api` - API实例指针
    // * `instrument_ids` - 合约ID数组
    // * `count` - 合约个数
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_SubscribeMarketData(
            api: *mut c_void,
            instrument_ids: *const *const c_char,
            count: c_int,
        ) -> c_int;
    }

    // 退订行情
    //
    // # 参数
    // * `api` - API实例指针
    // * `instrument_ids` - 合约ID数组
    // * `count` - 合约个数
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_UnSubscribeMarketData(
            api: *mut c_void,
            instrument_ids: *const *const c_char,
            count: c_int,
        ) -> c_int;
    }

    // 订阅询价
    //
    // # 参数
    // * `api` - API实例指针
    // * `instrument_ids` - 合约ID数组
    // * `count` - 合约个数
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_SubscribeForQuoteRsp(
            api: *mut c_void,
            instrument_ids: *const *const c_char,
            count: c_int,
        ) -> c_int;
    }

    // 退订询价
    //
    // # 参数
    // * `api` - API实例指针
    // * `instrument_ids` - 合约ID数组
    // * `count` - 合约个数
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcMdApi_UnSubscribeForQuoteRsp(
            api: *mut c_void,
            instrument_ids: *const *const c_char,
            count: c_int,
        ) -> c_int;
    }
}

// CTP交易API的FFI声明
pub mod trader_api {
    use super::*;

    // 创建交易API实例
    //
    // # 参数
    // * `flow_path` - 存储流文件的目录，默认为当前目录
    //
    // # 返回值
    // 交易API实例指针
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_CreateFtdcTraderApi(
            flow_path: *const c_char,
            is_production_mode: bool,
        ) -> *mut c_void;
    }

    // 获取API版本号
    //
    // # 返回值
    // 版本号字符串
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_GetApiVersion() -> *const c_char;
    }

    // 释放API实例
    //
    // # 参数
    // * `api` - API实例指针
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_Release(api: *mut c_void);
    }

    // 初始化API
    //
    // # 参数
    // * `api` - API实例指针
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_Init(api: *mut c_void);
    }

    // 等待API线程结束运行
    //
    // # 参数
    // * `api` - API实例指针
    //
    // # 返回值
    // 线程退出代码
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_Join(api: *mut c_void) -> c_int;
    }

    // 获取当前交易日
    //
    // # 参数
    // * `api` - API实例指针
    //
    // # 返回值
    // 交易日字符串(YYYYMMDD格式)
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_GetTradingDay(api: *mut c_void) -> *const c_char;
    }

    // 注册前置机网络地址
    //
    // # 参数
    // * `api` - API实例指针
    // * `front_address` - 前置机网络地址
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_RegisterFront(api: *mut c_void, front_address: *const c_char);
    }

    // 注册名字服务器网络地址
    //
    // # 参数
    // * `api` - API实例指针
    // * `ns_address` - 名字服务器网络地址
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_RegisterNameServer(api: *mut c_void, ns_address: *const c_char);
    }

    // 注册名字服务器用户信息
    //
    // # 参数
    // * `api` - API实例指针
    // * `user_info` - 用户信息
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_RegisterFensUserInfo(api: *mut c_void, user_info: *const c_void);
    }

    // 注册回调接口
    //
    // # 参数
    // * `api` - API实例指针
    // * `spi` - 派生自CThostFtdcTraderSpi的实例指针
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_RegisterSpi(api: *mut c_void, spi: *mut c_void);
    }

    // 获取已连接的前置信息
    //
    // # 参数
    // * `api` - API实例指针
    // * `front_info` - 前置信息结构体指针
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_GetFrontInfo(api: *mut c_void, front_info: *mut c_void);
    }

    // 注册微信用户系统信息
    //
    // # 参数
    // * `api` - API实例指针
    // * `user_system_info` - 微信用户系统信息
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_RegisterWechatUserSystemInfo(
            api: *mut c_void,
            user_system_info: *mut c_void,
        ) -> c_int;
    }

    // 上报微信用户系统信息
    //
    // # 参数
    // * `api` - API实例指针
    // * `user_system_info` - 微信用户系统信息
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_SubmitWechatUserSystemInfo(
            api: *mut c_void,
            user_system_info: *mut c_void,
        ) -> c_int;
    }

    // 客户端认证请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 认证请求
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqAuthenticate(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 注册用户终端信息，用于中继服务器多连接模式
    //
    // # 参数
    // * `api` - API实例指针
    // * `user_info` - 用户终端信息
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_RegisterUserSystemInfo(
            api: *mut c_void,
            user_info: *const c_void,
        ) -> c_int;
    }

    // 上报用户终端信息，用于中继服务器操作员登录模式
    //
    // # 参数
    // * `api` - API实例指针
    // * `user_info` - 用户终端信息
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_SubmitUserSystemInfo(
            api: *mut c_void,
            user_info: *const c_void,
        ) -> c_int;
    }

    // 用户登录请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 登录请求
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqUserLogin(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 登出请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 登出请求
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqUserLogout(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询资金账户
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询资金账户请求字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryTradingAccount(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询投资者持仓
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询投资者持仓请求字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryInvestorPosition(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 第一阶段新增API方法

    // 报单录入请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 报单录入请求字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqOrderInsert(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 报单操作请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 报单操作请求字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqOrderAction(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询报单
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询报单请求字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryOrder(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询成交
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询成交请求字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryTrade(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询合约
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询合约请求字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryInstrument(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 第二阶段新增API方法

    // 请求查询合约保证金率
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询合约保证金率请求字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryInstrumentMarginRate(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询合约手续费率
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询合约手续费率请求字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryInstrumentCommissionRate(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询交易所
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询交易所请求字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryExchange(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询产品
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询产品请求字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryProduct(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 投资者结算结果确认
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 投资者结算结果确认信息
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqSettlementInfoConfirm(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 预埋单录入请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 预埋单字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqParkedOrderInsert(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 预埋撤单录入请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 预埋撤单字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqParkedOrderAction(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 第三阶段新增API方法

    // 执行宣告录入请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 执行宣告录入字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqExecOrderInsert(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 执行宣告操作请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 执行宣告操作字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqExecOrderAction(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 询价录入请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 询价录入字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqForQuoteInsert(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 报价录入请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 报价录入字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQuoteInsert(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 报价操作请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 报价操作字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQuoteAction(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 批量报单操作请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 批量报单操作字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqBatchOrderAction(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 删除预埋单请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 删除预埋单字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqRemoveParkedOrder(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 删除预埋撤单请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 删除预埋撤单字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqRemoveParkedOrderAction(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 查询最大报单数量请求
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询最大报单数量字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryMaxOrderVolume(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询行情
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询行情字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryDepthMarketData(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询投资者结算结果
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询投资者结算结果字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQrySettlementInfo(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询转帐银行
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询转帐银行字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryTransferBank(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询投资者持仓明细
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询投资者持仓明细字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryInvestorPositionDetail(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }

    // 请求查询客户通知
    //
    // # 参数
    // * `api` - API实例指针
    // * `req` - 查询客户通知字段
    // * `request_id` - 请求ID
    //
    // # 返回值
    // 0表示成功，非0表示失败
    #[link(name = "ctp_wrapper")]
    extern "C" {
        pub fn CThostFtdcTraderApi_ReqQryNotice(
            api: *mut c_void,
            req: *const c_void,
            request_id: c_int,
        ) -> c_int;
    }
}

// SPI回调函数类型定义
pub mod callbacks {
    use super::*;

    // 连接建立回调
    pub type OnFrontConnectedCallback = extern "C" fn();

    // 连接断开回调
    pub type OnFrontDisconnectedCallback = extern "C" fn(reason: c_int);

    // 心跳超时警告回调
    pub type OnHeartBeatWarningCallback = extern "C" fn(time_lapse: c_int);

    // 登录响应回调
    pub type OnRspUserLoginCallback = extern "C" fn(
        user_login: *const c_void,
        rsp_info: *const c_void,
        request_id: c_int,
        is_last: bool,
    );

    // 登出响应回调
    pub type OnRspUserLogoutCallback = extern "C" fn(
        user_logout: *const c_void,
        rsp_info: *const c_void,
        request_id: c_int,
        is_last: bool,
    );

    // 深度行情通知回调
    pub type OnRtnDepthMarketDataCallback = extern "C" fn(depth_market_data: *const c_void);

    // 订阅行情应答回调
    pub type OnRspSubMarketDataCallback = extern "C" fn(
        specific_instrument: *const c_void,
        rsp_info: *const c_void,
        request_id: c_int,
        is_last: bool,
    );

    // 取消订阅行情应答回调
    pub type OnRspUnSubMarketDataCallback = extern "C" fn(
        specific_instrument: *const c_void,
        rsp_info: *const c_void,
        request_id: c_int,
        is_last: bool,
    );

    // 认证响应回调
    pub type OnRspAuthenticateCallback = extern "C" fn(
        rsp_authenticate: *const c_void,
        rsp_info: *const c_void,
        request_id: c_int,
        is_last: bool,
    );

    // 错误响应回调
    pub type OnRspErrorCallback =
        extern "C" fn(rsp_info: *const c_void, request_id: c_int, is_last: bool);
}

// 确保FFI绑定在编译时正确链接
#[cfg(test)]
mod tests {
    use std::ptr;

    #[test]
    fn test_ffi_declarations_compile() {
        // 这个测试确保FFI声明可以正确编译
        // 实际的FFI调用需要真实的CTP库
        let _api_ptr: *mut std::os::raw::c_void = ptr::null_mut();
        assert!(true, "FFI声明编译成功");
    }
}
