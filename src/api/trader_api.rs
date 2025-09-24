//! 交易API模块
//!
//! 提供期货交易功能，包括下单、撤单、查询等

use crate::api::md_api::DepthMarketDataField;
use crate::api::utils::normalize_flow_path;
use crate::api::{safe_cstr_to_string, to_cstring, CtpApi};
use crate::error::{CtpError, CtpResult};
use crate::ffi::trader_api::*;
use crate::ffi::{CreateTraderSpiBridge, TraderSpiCallbacks};
use crate::types::{
    ExchangeField, InputBatchOrderActionField, InputExecOrderActionField, InputExecOrderField,
    InputForQuoteField, InputOrderActionField, InputQuoteActionField, InputQuoteField,
    InstrumentCommissionRateField, InstrumentMarginRateField, InvestorPositionDetailField,
    NoticeField, OrderActionField, ParkedOrderActionField, ParkedOrderField, ProductField,
    QryDepthMarketDataField, QryExchangeField, QryInstrumentCommissionRateField,
    QryInstrumentField, QryInstrumentMarginRateField, QryInvestorPositionDetailField,
    QryInvestorPositionField, QryMaxOrderVolumeField, QryNoticeField, QryOrderField,
    QryProductField, QrySettlementInfoField, QryTradeField, QryTradingAccountField,
    QryTransferBankField, RemoveParkedOrderActionField, RemoveParkedOrderField, ReqUserLoginField,
    RspInfoField, RspUserLoginField, SettlementInfoConfirmField, SettlementInfoField,
    TransferBankField,
};
use std::ffi::c_void;
use std::os::raw::c_int;
use std::ptr;
use std::sync::{Arc, Mutex};

// 交易API封装
#[allow(dead_code)]
pub struct TraderApi {
    // C++ API实例指针
    api_ptr: *mut c_void,
    // SPI实例指针
    spi_ptr: *mut c_void,
    // 是否已初始化
    initialized: bool,
    // 请求ID计数器
    request_id: Arc<Mutex<i32>>,
    // 回调处理器
    handler: Option<Box<dyn TraderSpiHandler + Send + Sync>>,
}

// 交易SPI回调处理器特质
#[allow(unused_variables)]
pub trait TraderSpiHandler {
    // 当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用
    fn on_front_connected(&mut self) {}

    // 当客户端与交易后台通信连接断开时，该方法被调用
    fn on_front_disconnected(&mut self, reason: i32) {}

    // 心跳超时警告
    fn on_heart_beat_warning(&mut self, time_lapse: i32) {}

    // 客户端认证响应
    fn on_rsp_authenticate(
        &mut self,
        rsp_authenticate: Option<RspAuthenticateField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

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

    // 报单录入请求响应
    fn on_rsp_order_insert(
        &mut self,
        input_order: Option<InputOrderField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 报单操作请求响应
    fn on_rsp_order_action(
        &mut self,
        input_order_action: Option<InputOrderActionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 报单通知
    fn on_rtn_order(&mut self, order: OrderField) {}

    // 成交通知
    fn on_rtn_trade(&mut self, trade: TradeField) {}

    // 请求查询投资者响应
    fn on_rsp_qry_investor(
        &mut self,
        investor: Option<InvestorField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 请求查询资金账户响应
    fn on_rsp_qry_trading_account(
        &mut self,
        trading_account: Option<TradingAccountField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 请求查询投资者持仓响应
    fn on_rsp_qry_investor_position(
        &mut self,
        investor_position: Option<InvestorPositionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 请求查询合约响应
    fn on_rsp_qry_instrument(
        &mut self,
        instrument: Option<InstrumentField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 第一阶段新增回调方法

    // 报单录入错误回报
    fn on_err_rtn_order_insert(
        &mut self,
        input_order: Option<InputOrderField>,
        rsp_info: Option<RspInfoField>,
    ) {
    }

    // 报单操作错误回报
    fn on_err_rtn_order_action(
        &mut self,
        order_action: Option<OrderActionField>,
        rsp_info: Option<RspInfoField>,
    ) {
    }

    // 请求查询报单响应
    fn on_rsp_qry_order(
        &mut self,
        order: Option<OrderField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 请求查询成交响应
    fn on_rsp_qry_trade(
        &mut self,
        trade: Option<TradeField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 第二阶段新增回调方法

    // 请求查询合约保证金率响应
    fn on_rsp_qry_instrument_margin_rate(
        &mut self,
        margin_rate: Option<InstrumentMarginRateField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 请求查询合约手续费率响应
    fn on_rsp_qry_instrument_commission_rate(
        &mut self,
        commission_rate: Option<InstrumentCommissionRateField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 请求查询交易所响应
    fn on_rsp_qry_exchange(
        &mut self,
        exchange: Option<ExchangeField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 请求查询产品响应
    fn on_rsp_qry_product(
        &mut self,
        product: Option<ProductField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 投资者结算结果确认响应
    fn on_rsp_settlement_info_confirm(
        &mut self,
        settlement_info_confirm: Option<SettlementInfoConfirmField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 预埋单录入请求响应
    fn on_rsp_parked_order_insert(
        &mut self,
        parked_order: Option<ParkedOrderField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 预埋撤单录入请求响应
    fn on_rsp_parked_order_action(
        &mut self,
        parked_order_action: Option<ParkedOrderActionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 第三阶段新增回调方法

    // 执行宣告录入请求响应
    fn on_rsp_exec_order_insert(
        &mut self,
        input_exec_order: Option<InputExecOrderField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 执行宣告操作请求响应
    fn on_rsp_exec_order_action(
        &mut self,
        input_exec_order_action: Option<InputExecOrderActionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 询价录入请求响应
    fn on_rsp_for_quote_insert(
        &mut self,
        input_for_quote: Option<InputForQuoteField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 报价录入请求响应
    fn on_rsp_quote_insert(
        &mut self,
        input_quote: Option<InputQuoteField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 报价操作请求响应
    fn on_rsp_quote_action(
        &mut self,
        input_quote_action: Option<InputQuoteActionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 批量报单操作请求响应
    fn on_rsp_batch_order_action(
        &mut self,
        input_batch_order_action: Option<InputBatchOrderActionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 删除预埋单响应
    fn on_rsp_remove_parked_order(
        &mut self,
        remove_parked_order: Option<RemoveParkedOrderField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 删除预埋撤单响应
    fn on_rsp_remove_parked_order_action(
        &mut self,
        remove_parked_order_action: Option<RemoveParkedOrderActionField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 查询最大报单数量响应
    fn on_rsp_qry_max_order_volume(
        &mut self,
        qry_max_order_volume: Option<QryMaxOrderVolumeField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 请求查询行情响应
    fn on_rsp_qry_depth_market_data(
        &mut self,
        depth_market_data: Option<DepthMarketDataField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 请求查询投资者结算结果响应
    fn on_rsp_qry_settlement_info(
        &mut self,
        settlement_info: Option<SettlementInfoField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 请求查询转帐银行响应
    fn on_rsp_qry_transfer_bank(
        &mut self,
        transfer_bank: Option<TransferBankField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 请求查询投资者持仓明细响应
    fn on_rsp_qry_investor_position_detail(
        &mut self,
        investor_position_detail: Option<InvestorPositionDetailField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }

    // 请求查询客户通知响应
    fn on_rsp_qry_notice(
        &mut self,
        notice: Option<NoticeField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
    }
}

// 客户端认证响应
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RspAuthenticateField {
    // 经纪公司代码
    pub broker_id: [u8; 11],
    // 用户代码
    pub user_id: [u8; 16],
    // 用户端产品信息
    pub user_product_info: [u8; 11],
    // 应用单元代码
    pub app_id: [u8; 33],
    // 应用类型
    pub app_type: u8,
}

impl Default for RspAuthenticateField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 报单录入
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputOrderField {
    // 经纪公司代码
    pub broker_id: [u8; 11],
    // 投资者代码
    pub investor_id: [u8; 13],
    // 合约代码
    pub instrument_id: [u8; 31],
    // 报单引用
    pub order_ref: [u8; 13],
    // 用户代码
    pub user_id: [u8; 16],
    // 报单价格条件
    pub order_price_type: u8,
    // 买卖方向
    pub direction: u8,
    // 组合开平标志
    pub comb_offset_flag: [u8; 5],
    // 组合投机套保标志
    pub comb_hedge_flag: [u8; 5],
    // 价格
    pub limit_price: f64,
    // 数量
    pub volume_total_original: i32,
    // 有效期类型
    pub time_condition: u8,
    // GTD日期
    pub gtd_date: [u8; 9],
    // 成交量类型
    pub volume_condition: u8,
    // 最小成交量
    pub min_volume: i32,
    // 触发条件
    pub contingent_condition: u8,
    // 止损价
    pub stop_price: f64,
    // 强平原因
    pub force_close_reason: u8,
    // 自动挂起标志
    pub is_auto_suspend: i32,
    // 业务单元
    pub business_unit: [u8; 21],
    // 请求编号
    pub request_id: i32,
    // 用户强平标志
    pub user_force_close: i32,
    // 互换单标志
    pub is_swap_order: i32,
    // 交易所代码
    pub exchange_id: [u8; 9],
    // 投资单元代码
    pub invest_unit_id: [u8; 17],
    // 资金账号
    pub account_id: [u8; 13],
    // 币种代码
    pub currency_id: [u8; 4],
    // 交易编码
    pub client_id: [u8; 11],
    // Mac地址
    pub mac_address: [u8; 21],
    // 合约在交易所的代码
    pub exchange_inst_id: [u8; 31],
    // IP地址
    pub ip_address: [u8; 16],
}

impl Default for InputOrderField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 报单
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OrderField {
    // 经纪公司代码
    pub broker_id: [u8; 11],
    // 投资者代码
    pub investor_id: [u8; 13],
    // 合约代码
    pub instrument_id: [u8; 31],
    // 报单引用
    pub order_ref: [u8; 13],
    // 用户代码
    pub user_id: [u8; 16],
    // 报单价格条件
    pub order_price_type: u8,
    // 买卖方向
    pub direction: u8,
    // 组合开平标志
    pub comb_offset_flag: [u8; 5],
    // 组合投机套保标志
    pub comb_hedge_flag: [u8; 5],
    // 价格
    pub limit_price: f64,
    // 数量
    pub volume_total_original: i32,
    // 有效期类型
    pub time_condition: u8,
    // GTD日期
    pub gtd_date: [u8; 9],
    // 成交量类型
    pub volume_condition: u8,
    // 最小成交量
    pub min_volume: i32,
    // 触发条件
    pub contingent_condition: u8,
    // 止损价
    pub stop_price: f64,
    // 强平原因
    pub force_close_reason: u8,
    // 自动挂起标志
    pub is_auto_suspend: i32,
    // 业务单元
    pub business_unit: [u8; 21],
    // 请求编号
    pub request_id: i32,
    // 本地报单编号
    pub order_local_id: [u8; 13],
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
    // 报单提交状态
    pub order_submit_status: u8,
    // 报单提示序号
    pub notify_sequence: i32,
    // 交易日
    pub trading_day: [u8; 9],
    // 结算编号
    pub settlement_id: i32,
    // 报单编号
    pub order_sys_id: [u8; 21],
    // 报单来源
    pub order_source: u8,
    // 报单状态
    pub order_status: u8,
    // 报单类型
    pub order_type: u8,
    // 今成交数量
    pub volume_traded: i32,
    // 剩余数量
    pub volume_total: i32,
    // 报单日期
    pub insert_date: [u8; 9],
    // 委托时间
    pub insert_time: [u8; 9],
    // 激活时间
    pub active_time: [u8; 9],
    // 挂起时间
    pub suspend_time: [u8; 9],
    // 最后修改时间
    pub update_time: [u8; 9],
    // 撤销时间
    pub cancel_time: [u8; 9],
    // 最后修改交易员代码
    pub active_trader_id: [u8; 21],
    // 结算会员编号
    pub clearing_part_id: [u8; 11],
    // 序号
    pub sequence_no: i32,
    // 前置编号
    pub front_id: i32,
    // 会话编号
    pub session_id: i32,
    // 用户端产品信息
    pub user_product_info: [u8; 11],
    // 状态信息
    pub status_msg: [u8; 81],
    // 用户强平标志
    pub user_force_close: i32,
    // 操作用户代码
    pub active_user_id: [u8; 16],
    // 经纪公司报单编号
    pub broker_order_seq: i32,
    // 相关报单
    pub relative_order_sys_id: [u8; 21],
    // 郑商所成交数量
    pub zczc_total_traded_volume: i32,
    // 互换单标志
    pub is_swap_order: i32,
    // 营业部编号
    pub branch_id: [u8; 9],
    // 投资单元代码
    pub invest_unit_id: [u8; 17],
    // 资金账号
    pub account_id: [u8; 13],
    // 币种代码
    pub currency_id: [u8; 4],
    // Mac地址
    pub mac_address: [u8; 21],
    // IP地址
    pub ip_address: [u8; 16],
}

impl Default for OrderField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 成交
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TradeField {
    // 经纪公司代码
    pub broker_id: [u8; 11],
    // 投资者代码
    pub investor_id: [u8; 13],
    // 合约代码
    pub instrument_id: [u8; 31],
    // 报单引用
    pub order_ref: [u8; 13],
    // 用户代码
    pub user_id: [u8; 16],
    // 交易所代码
    pub exchange_id: [u8; 9],
    // 成交编号
    pub trade_id: [u8; 21],
    // 买卖方向
    pub direction: u8,
    // 报单编号
    pub order_sys_id: [u8; 21],
    // 会员代码
    pub participant_id: [u8; 11],
    // 客户代码
    pub client_id: [u8; 11],
    // 交易角色
    pub trading_role: u8,
    // 合约在交易所的代码
    pub exchange_inst_id: [u8; 31],
    // 开平标志
    pub offset_flag: u8,
    // 投机套保标志
    pub hedge_flag: u8,
    // 价格
    pub price: f64,
    // 数量
    pub volume: i32,
    // 成交时期
    pub trade_date: [u8; 9],
    // 成交时间
    pub trade_time: [u8; 9],
    // 成交类型
    pub trade_type: u8,
    // 成交价来源
    pub price_source: u8,
    // 交易员代码
    pub trader_id: [u8; 21],
    // 本地报单编号
    pub order_local_id: [u8; 13],
    // 结算会员编号
    pub clearing_part_id: [u8; 11],
    // 业务单元
    pub business_unit: [u8; 21],
    // 序号
    pub sequence_no: i32,
    // 交易日
    pub trading_day: [u8; 9],
    // 结算编号
    pub settlement_id: i32,
    // 经纪公司报单编号
    pub broker_order_seq: i32,
    // 成交来源
    pub trade_source: u8,
    // 投资单元代码
    pub invest_unit_id: [u8; 17],
}

impl Default for TradeField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 投资者
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InvestorField {
    // 投资者代码
    pub investor_id: [u8; 13],
    // 经纪公司代码
    pub broker_id: [u8; 11],
    // 投资者分组代码
    pub investor_group_id: [u8; 13],
    // 投资者名称
    pub investor_name: [u8; 81],
    // 证件类型
    pub identity_card_type: u8,
    // 证件号码
    pub identity_card_no: [u8; 51],
    // 是否活跃
    pub is_active: i32,
    // 联系电话
    pub telephone: [u8; 41],
    // 通讯地址
    pub address: [u8; 101],
    // 开户日期
    pub open_date: [u8; 9],
    // 手机
    pub mobile: [u8; 41],
    // 手续费率模板代码
    pub comm_model_id: [u8; 13],
    // 保证金率模板代码
    pub margin_model_id: [u8; 13],
}

impl Default for InvestorField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 资金账户
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TradingAccountField {
    // 经纪公司代码
    pub broker_id: [u8; 11],
    // 投资者帐号
    pub account_id: [u8; 13],
    // 上次质押金额
    pub pre_mortgage: f64,
    // 上次信用额度
    pub pre_credit: f64,
    // 上次存款额
    pub pre_deposit: f64,
    // 上次结算准备金
    pub pre_balance: f64,
    // 上次占用的保证金
    pub pre_margin: f64,
    // 利息基数
    pub interest_base: f64,
    // 利息收入
    pub interest: f64,
    // 入金金额
    pub deposit: f64,
    // 出金金额
    pub withdraw: f64,
    // 冻结的保证金
    pub frozen_margin: f64,
    // 冻结的资金
    pub frozen_cash: f64,
    // 冻结的手续费
    pub frozen_commission: f64,
    // 当前保证金总额
    pub curr_margin: f64,
    // 资金差额
    pub cash_in: f64,
    // 手续费
    pub commission: f64,
    // 平仓盈亏
    pub close_profit: f64,
    // 持仓盈亏
    pub position_profit: f64,
    // 期货结算准备金
    pub balance: f64,
    // 可用资金
    pub available: f64,
    // 可取资金
    pub withdraw_quota: f64,
    // 基本准备金
    pub reserve: f64,
    // 交易日
    pub trading_day: [u8; 9],
    // 结算编号
    pub settlement_id: i32,
    // 信用额度
    pub credit: f64,
    // 质押金额
    pub mortgage: f64,
    // 交易所保证金
    pub exchange_margin: f64,
    // 投资者交割保证金
    pub delivery_margin: f64,
    // 交易所交割保证金
    pub exchange_delivery_margin: f64,
    // 保底期货结算准备金
    pub reserve_balance: f64,
    // 币种代码
    pub currency_id: [u8; 4],
    // 上次货币质入金额
    pub pre_fund_mortgage_in: f64,
    // 上次货币质出金额
    pub pre_fund_mortgage_out: f64,
    // 货币质入金额
    pub fund_mortgage_in: f64,
    // 货币质出金额
    pub fund_mortgage_out: f64,
    // 货币质押余额
    pub fund_mortgage_available: f64,
    // 可质押货币金额
    pub mortgageable_fund: f64,
    // 特殊产品占用保证金
    pub spec_product_margin: f64,
    // 特殊产品冻结保证金
    pub spec_product_frozen_margin: f64,
    // 特殊产品手续费
    pub spec_product_commission: f64,
    // 特殊产品冻结手续费
    pub spec_product_frozen_commission: f64,
    // 特殊产品持仓盈亏
    pub spec_product_position_profit: f64,
    // 特殊产品平仓盈亏
    pub spec_product_close_profit: f64,
    // 根据持仓盈亏算法计算的特殊产品持仓盈亏
    pub spec_product_position_profit_by_alg: f64,
    // 特殊产品交易所保证金
    pub spec_product_exchange_margin: f64,
    // 业务类型
    pub biz_type: u8,
    // 延时换汇冻结金额
    pub frozen_swap: f64,
    // 剩余换汇额度
    pub remain_swap: f64,
}

impl Default for TradingAccountField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 投资者持仓
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InvestorPositionField {
    // 合约代码
    pub instrument_id: [u8; 31],
    // 经纪公司代码
    pub broker_id: [u8; 11],
    // 投资者代码
    pub investor_id: [u8; 13],
    // 持仓多空方向
    pub posi_direction: u8,
    // 投机套保标志
    pub hedge_flag: u8,
    // 持仓日期
    pub position_date: u8,
    // 上日持仓
    pub yd_position: i32,
    // 今日持仓
    pub position: i32,
    // 多头冻结
    pub long_frozen: i32,
    // 空头冻结
    pub short_frozen: i32,
    // 开仓冻结金额
    pub long_frozen_amount: f64,
    // 开仓冻结金额
    pub short_frozen_amount: f64,
    // 开仓量
    pub open_volume: i32,
    // 平仓量
    pub close_volume: i32,
    // 开仓金额
    pub open_amount: f64,
    // 平仓金额
    pub close_amount: f64,
    // 持仓成本
    pub position_cost: f64,
    // 上次占用的保证金
    pub pre_margin: f64,
    // 占用的保证金
    pub use_margin: f64,
    // 冻结的保证金
    pub frozen_margin: f64,
    // 冻结的资金
    pub frozen_cash: f64,
    // 冻结的手续费
    pub frozen_commission: f64,
    // 资金差额
    pub cash_in: f64,
    // 手续费
    pub commission: f64,
    // 平仓盈亏
    pub close_profit: f64,
    // 持仓盈亏
    pub position_profit: f64,
    // 上次结算价
    pub pre_settlement_price: f64,
    // 本次结算价
    pub settlement_price: f64,
    // 交易日
    pub trading_day: [u8; 9],
    // 结算编号
    pub settlement_id: i32,
    // 开仓成本
    pub open_cost: f64,
    // 交易所保证金
    pub exchange_margin: f64,
    // 组合成交数量
    pub comb_position: i32,
    // 组合多头冻结
    pub comb_long_frozen: i32,
    // 组合空头冻结
    pub comb_short_frozen: i32,
    // 逐日盯市平仓盈亏
    pub close_profit_by_date: f64,
    // 逐笔对冲平仓盈亏
    pub close_profit_by_trade: f64,
    // 今日持仓
    pub today_position: i32,
    // 保证金率
    pub margin_rate_by_money: f64,
    // 保证金率(按手数)
    pub margin_rate_by_volume: f64,
    // 执行冻结
    pub strike_frozen: i32,
    // 执行冻结金额
    pub strike_frozen_amount: f64,
    // 放弃执行冻结
    pub abandon_frozen: i32,
    // 交易所代码
    pub exchange_id: [u8; 9],
    // 执行冻结的昨仓
    pub yd_strike_frozen: i32,
    // 投资单元代码
    pub invest_unit_id: [u8; 17],
    // 大商所持仓成本差值，只有大商所使用
    pub position_cost_offset: f64,
    // tas持仓手数
    pub tas_position: i32,
    // tas持仓成本
    pub tas_position_cost: f64,
}

impl Default for InvestorPositionField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 合约
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InstrumentField {
    // 合约代码
    pub instrument_id: [u8; 31],
    // 交易所代码
    pub exchange_id: [u8; 9],
    // 合约名称
    pub instrument_name: [u8; 21],
    // 合约在交易所的代码
    pub exchange_inst_id: [u8; 31],
    // 产品代码
    pub product_id: [u8; 31],
    // 产品类型
    pub product_class: u8,
    // 交割年份
    pub delivery_year: i32,
    // 交割月
    pub delivery_month: i32,
    // 市价单最大下单量
    pub max_market_order_volume: i32,
    // 市价单最小下单量
    pub min_market_order_volume: i32,
    // 限价单最大下单量
    pub max_limit_order_volume: i32,
    // 限价单最小下单量
    pub min_limit_order_volume: i32,
    // 合约数量乘数
    pub volume_multiple: i32,
    // 最小变动价位
    pub price_tick: f64,
    // 创建日
    pub create_date: [u8; 9],
    // 上市日
    pub open_date: [u8; 9],
    // 到期日
    pub expire_date: [u8; 9],
    // 开始交割日
    pub start_deliv_date: [u8; 9],
    // 结束交割日
    pub end_deliv_date: [u8; 9],
    // 合约生命周期状态
    pub inst_life_phase: u8,
    // 当前是否交易
    pub is_trading: i32,
    // 持仓类型
    pub position_type: u8,
    // 持仓日期类型
    pub position_date_type: u8,
    // 多头保证金率
    pub long_margin_ratio: f64,
    // 空头保证金率
    pub short_margin_ratio: f64,
    // 是否使用大额单边保证金算法
    pub max_margin_side_algorithm: u8,
    // 基础商品代码
    pub underlying_instr_id: [u8; 31],
    // 执行价
    pub strike_price: f64,
    // 期权类型
    pub options_type: u8,
    // 合约基础商品乘数
    pub underlying_multiple: f64,
    // 组合类型
    pub combination_type: u8,
}

impl Default for InstrumentField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

unsafe impl Send for TraderApi {}
unsafe impl Sync for TraderApi {}

impl TraderApi {
    // 创建交易API实例
    //
    // # 参数
    // * `flow_path` - 存储流文件的目录，默认为当前目录
    // * `is_production_mode` - 是否为生产环境模式，默认为true
    pub fn new(flow_path: Option<&str>, is_production_mode: Option<bool>) -> CtpResult<Self> {
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

        let production_mode = is_production_mode.unwrap_or(true);

        let api_ptr =
            unsafe { CThostFtdcTraderApi_CreateFtdcTraderApi(flow_path_ptr, production_mode) };

        if api_ptr.is_null() {
            return Err(CtpError::InitializationError("创建交易API失败".to_string()));
        }

        Ok(TraderApi {
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
        T: TraderSpiHandler + Send + Sync + 'static,
    {
        self.handler = Some(Box::new(handler));

        // 创建回调结构体
        let callbacks = TraderSpiCallbacks {
            user_data: self as *mut _ as *mut c_void,
            on_front_connected: Some(on_front_connected_callback),
            on_front_disconnected: Some(on_front_disconnected_callback),
            on_heart_beat_warning: Some(on_heart_beat_warning_callback),
            on_rsp_authenticate: Some(on_rsp_authenticate_callback),
            on_rsp_user_login: Some(on_rsp_user_login_callback),
            on_rsp_user_logout: Some(on_rsp_user_logout_callback),
            on_rsp_error: Some(on_rsp_error_callback),
            on_rsp_order_insert: Some(on_rsp_order_insert_callback),
            on_rsp_order_action: Some(on_rsp_order_action_callback),
            on_rtn_order: Some(on_rtn_order_callback),
            on_rtn_trade: Some(on_rtn_trade_callback),
            on_rsp_qry_trading_account: Some(on_rsp_qry_trading_account_callback),
            on_rsp_qry_investor_position: Some(on_rsp_qry_investor_position_callback),

            // 第一阶段新增回调
            on_err_rtn_order_insert: Some(on_err_rtn_order_insert_callback),
            on_err_rtn_order_action: Some(on_err_rtn_order_action_callback),
            on_rsp_qry_order: Some(on_rsp_qry_order_callback),
            on_rsp_qry_trade: Some(on_rsp_qry_trade_callback),
            on_rsp_qry_instrument: Some(on_rsp_qry_instrument_callback),

            // 第二阶段新增回调
            on_rsp_qry_instrument_margin_rate: Some(on_rsp_qry_instrument_margin_rate_callback),
            on_rsp_qry_instrument_commission_rate: Some(
                on_rsp_qry_instrument_commission_rate_callback,
            ),
            on_rsp_qry_exchange: Some(on_rsp_qry_exchange_callback),
            on_rsp_qry_product: Some(on_rsp_qry_product_callback),
            on_rsp_settlement_info_confirm: Some(on_rsp_settlement_info_confirm_callback),
            on_rsp_parked_order_insert: Some(on_rsp_parked_order_insert_callback),
            on_rsp_parked_order_action: Some(on_rsp_parked_order_action_callback),

            // 第三阶段新增回调
            on_rsp_exec_order_insert: Some(on_rsp_exec_order_insert_callback),
            on_rsp_exec_order_action: Some(on_rsp_exec_order_action_callback),
            on_rsp_for_quote_insert: Some(on_rsp_for_quote_insert_callback),
            on_rsp_quote_insert: Some(on_rsp_quote_insert_callback),
            on_rsp_quote_action: Some(on_rsp_quote_action_callback),
            on_rsp_batch_order_action: Some(on_rsp_batch_order_action_callback),
            on_rsp_remove_parked_order: Some(on_rsp_remove_parked_order_callback),
            on_rsp_remove_parked_order_action: Some(on_rsp_remove_parked_order_action_callback),
            on_rsp_qry_max_order_volume: Some(on_rsp_qry_max_order_volume_callback),
            on_rsp_qry_depth_market_data: Some(on_rsp_qry_depth_market_data_callback),
            on_rsp_qry_settlement_info: Some(on_rsp_qry_settlement_info_callback),
            on_rsp_qry_transfer_bank: Some(on_rsp_qry_transfer_bank_callback),
            on_rsp_qry_investor_position_detail: Some(on_rsp_qry_investor_position_detail_callback),
            on_rsp_qry_notice: Some(on_rsp_qry_notice_callback),
        };

        // 创建SPI桥接器并注册到C++ API
        self.spi_ptr = unsafe { CreateTraderSpiBridge(&callbacks) };

        if self.spi_ptr.is_null() {
            return Err(CtpError::InitializationError(
                "创建交易SPI桥接器失败".to_string(),
            ));
        }

        // 注册SPI到CTP API
        unsafe {
            CThostFtdcTraderApi_RegisterSpi(self.api_ptr, self.spi_ptr);
        }

        Ok(())
    }

    // 客户端认证请求
    pub fn req_authenticate(&mut self, req: &ReqAuthenticateField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqAuthenticate(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("认证请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 用户登录请求
    pub fn req_user_login(&mut self, req: &ReqUserLoginField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqUserLogin(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
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
            unsafe { CThostFtdcTraderApi_ReqUserLogout(self.api_ptr, ptr::null(), request_id) };

        if result != 0 {
            return Err(CtpError::FfiError(format!("登出请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 请求查询资金账户
    pub fn req_qry_trading_account(&mut self, req: &QryTradingAccountField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryTradingAccount(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "查询资金账户请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 请求查询投资者持仓
    pub fn req_qry_investor_position(&mut self, req: &QryInvestorPositionField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryInvestorPosition(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "查询投资者持仓请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 第一阶段新增API方法

    // 报单录入请求
    pub fn req_order_insert(&mut self, req: &InputOrderField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqOrderInsert(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("报单录入请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 报单操作请求（撤单）
    pub fn req_order_action(&mut self, req: &InputOrderActionField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqOrderAction(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("报单操作请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 请求查询报单
    pub fn req_qry_order(&mut self, req: &QryOrderField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryOrder(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("查询报单请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 请求查询成交
    pub fn req_qry_trade(&mut self, req: &QryTradeField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryTrade(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("查询成交请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 请求查询合约
    pub fn req_qry_instrument(&mut self, req: &QryInstrumentField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryInstrument(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("查询合约请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 第二阶段新增API方法

    // 请求查询合约保证金率
    pub fn req_qry_instrument_margin_rate(
        &mut self,
        req: &QryInstrumentMarginRateField,
    ) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryInstrumentMarginRate(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "查询合约保证金率请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 请求查询合约手续费率
    pub fn req_qry_instrument_commission_rate(
        &mut self,
        req: &QryInstrumentCommissionRateField,
    ) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryInstrumentCommissionRate(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "查询合约手续费率请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 请求查询交易所
    pub fn req_qry_exchange(&mut self, req: &QryExchangeField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryExchange(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "查询交易所请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 请求查询产品
    pub fn req_qry_product(&mut self, req: &QryProductField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryProduct(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("查询产品请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 投资者结算结果确认
    pub fn req_settlement_info_confirm(
        &mut self,
        req: &SettlementInfoConfirmField,
    ) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqSettlementInfoConfirm(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "投资者结算结果确认请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 预埋单录入请求
    pub fn req_parked_order_insert(&mut self, req: &ParkedOrderField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqParkedOrderInsert(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "预埋单录入请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 预埋撤单录入请求
    pub fn req_parked_order_action(&mut self, req: &ParkedOrderActionField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqParkedOrderAction(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "预埋撤单录入请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 第三阶段新增API方法

    // 执行宣告录入请求
    pub fn req_exec_order_insert(&mut self, req: &InputExecOrderField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqExecOrderInsert(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "执行宣告录入请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 执行宣告操作请求
    pub fn req_exec_order_action(&mut self, req: &InputExecOrderActionField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqExecOrderAction(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "执行宣告操作请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 询价录入请求
    pub fn req_for_quote_insert(&mut self, req: &InputForQuoteField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqForQuoteInsert(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("询价录入请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 报价录入请求
    pub fn req_quote_insert(&mut self, req: &InputQuoteField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQuoteInsert(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("报价录入请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 报价操作请求
    pub fn req_quote_action(&mut self, req: &InputQuoteActionField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQuoteAction(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("报价操作请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 批量报单操作请求
    pub fn req_batch_order_action(&mut self, req: &InputBatchOrderActionField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqBatchOrderAction(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "批量报单操作请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 删除预埋单请求
    pub fn req_remove_parked_order(&mut self, req: &RemoveParkedOrderField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqRemoveParkedOrder(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "删除预埋单请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 删除预埋撤单请求
    pub fn req_remove_parked_order_action(
        &mut self,
        req: &RemoveParkedOrderActionField,
    ) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqRemoveParkedOrderAction(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "删除预埋撤单请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 查询最大报单数量请求
    pub fn req_qry_max_order_volume(&mut self, req: &QryMaxOrderVolumeField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryMaxOrderVolume(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "查询最大报单数量请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 请求查询行情
    pub fn req_qry_depth_market_data(&mut self, req: &QryDepthMarketDataField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryDepthMarketData(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!("查询行情请求失败: {}", result)));
        }

        Ok(request_id)
    }

    // 请求查询投资者结算结果
    pub fn req_qry_settlement_info(&mut self, req: &QrySettlementInfoField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQrySettlementInfo(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "查询投资者结算结果请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 请求查询转帐银行
    pub fn req_qry_transfer_bank(&mut self, req: &QryTransferBankField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryTransferBank(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "查询转帐银行请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 请求查询投资者持仓明细
    pub fn req_qry_investor_position_detail(
        &mut self,
        req: &QryInvestorPositionDetailField,
    ) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryInvestorPositionDetail(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "查询投资者持仓明细请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 请求查询客户通知
    pub fn req_qry_notice(&mut self, req: &QryNoticeField) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let request_id = self.next_request_id();

        let result = unsafe {
            CThostFtdcTraderApi_ReqQryNotice(
                self.api_ptr,
                req as *const _ as *const c_void,
                request_id,
            )
        };

        if result != 0 {
            return Err(CtpError::FfiError(format!(
                "查询客户通知请求失败: {}",
                result
            )));
        }

        Ok(request_id)
    }

    // 获取下一个请求ID
    fn next_request_id(&self) -> i32 {
        let mut id = self.request_id.lock().unwrap();
        let current = *id;
        *id += 1;
        current
    }
}

// 客户端认证请求
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ReqAuthenticateField {
    // 经纪公司代码
    pub broker_id: [u8; 11],
    // 用户代码
    pub user_id: [u8; 16],
    // 用户端产品信息
    pub user_product_info: [u8; 11],
    // 认证码
    pub auth_code: [u8; 17],
    // 应用单元代码
    pub app_id: [u8; 33],
}

impl Default for ReqAuthenticateField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl CtpApi for TraderApi {
    fn get_version() -> CtpResult<String> {
        let version_ptr = unsafe { CThostFtdcTraderApi_GetApiVersion() };
        safe_cstr_to_string(version_ptr)
    }

    fn init(&mut self) -> CtpResult<()> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API指针为空".to_string()));
        }

        unsafe {
            CThostFtdcTraderApi_Init(self.api_ptr);
        }

        self.initialized = true;
        Ok(())
    }

    fn release(&mut self) {
        if !self.api_ptr.is_null() {
            unsafe {
                CThostFtdcTraderApi_Release(self.api_ptr);
            }
            self.api_ptr = ptr::null_mut();
        }
        self.initialized = false;
    }

    fn get_trading_day(&self) -> CtpResult<String> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let trading_day_ptr = unsafe { CThostFtdcTraderApi_GetTradingDay(self.api_ptr) };

        safe_cstr_to_string(trading_day_ptr)
    }

    fn register_front(&mut self, front_address: &str) -> CtpResult<()> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }
        let front_address_cstr = to_cstring(front_address)?;
        unsafe {
            CThostFtdcTraderApi_RegisterFront(self.api_ptr, front_address_cstr.as_ptr());
        }

        Ok(())
    }

    fn join(&self) -> CtpResult<i32> {
        if self.api_ptr.is_null() {
            return Err(CtpError::InitializationError("API未初始化".to_string()));
        }

        let result = unsafe { CThostFtdcTraderApi_Join(self.api_ptr) };

        Ok(result)
    }
}

// 回调函数实现
extern "C" fn on_front_connected_callback(user_data: *mut c_void) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                handler.on_front_connected();
            }
        }
    }
}

extern "C" fn on_front_disconnected_callback(user_data: *mut c_void, reason: c_int) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                handler.on_front_disconnected(reason);
            }
        }
    }
}

extern "C" fn on_heart_beat_warning_callback(user_data: *mut c_void, time_lapse: c_int) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                handler.on_heart_beat_warning(time_lapse);
            }
        }
    }
}

extern "C" fn on_rsp_authenticate_callback(
    user_data: *mut c_void,
    rsp_authenticate: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // 解析rsp_authenticate指针
                let parsed_rsp_authenticate = if !rsp_authenticate.is_null() {
                    let auth_ptr = rsp_authenticate as *const RspAuthenticateField;
                    Some((*auth_ptr).clone())
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

                handler.on_rsp_authenticate(
                    parsed_rsp_authenticate,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
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
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
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
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // 解析rsp_info指针
                let parsed_rsp_info = if !rsp_info.is_null() {
                    let rsp_ptr = rsp_info as *const RspInfoField;
                    Some((*rsp_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_user_logout(None, parsed_rsp_info, request_id, is_last != 0);
            }
        }
    }
}

extern "C" fn on_rsp_error_callback(
    user_data: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // 解析rsp_info指针
                let parsed_rsp_info = if !rsp_info.is_null() {
                    let rsp_ptr = rsp_info as *const RspInfoField;
                    Some((*rsp_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_error(parsed_rsp_info, request_id, is_last != 0);
            }
        }
    }
}

extern "C" fn on_rsp_order_insert_callback(
    user_data: *mut c_void,
    input_order: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // 解析input_order指针
                let parsed_input_order = if !input_order.is_null() {
                    let order_ptr = input_order as *const InputOrderField;
                    Some((*order_ptr).clone())
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

                handler.on_rsp_order_insert(
                    parsed_input_order,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_order_action_callback(
    user_data: *mut c_void,
    input_order_action: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // 解析input_order_action指针
                let parsed_input_order_action = if !input_order_action.is_null() {
                    let action_ptr = input_order_action as *const InputOrderActionField;
                    Some((*action_ptr).clone())
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

                handler.on_rsp_order_action(
                    parsed_input_order_action,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rtn_order_callback(user_data: *mut c_void, order: *mut c_void) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // 解析order指针
                if !order.is_null() {
                    let order_ptr = order as *const OrderField;
                    let parsed_order = (*order_ptr).clone();
                    handler.on_rtn_order(parsed_order);
                } else {
                    // 如果指针为空，创建默认order
                    let temp_order = OrderField::default();
                    handler.on_rtn_order(temp_order);
                }
            }
        }
    }
}

extern "C" fn on_rtn_trade_callback(user_data: *mut c_void, trade: *mut c_void) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                // 解析trade指针
                if !trade.is_null() {
                    let trade_ptr = trade as *const TradeField;
                    let parsed_trade = (*trade_ptr).clone();
                    handler.on_rtn_trade(parsed_trade);
                } else {
                    // 如果指针为空，创建默认trade
                    let temp_trade = TradeField::default();
                    handler.on_rtn_trade(temp_trade);
                }
            }
        }
    }
}

extern "C" fn on_rsp_qry_trading_account_callback(
    user_data: *mut c_void,
    trading_account: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        // 添加调试信息

        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_trading_account = if !trading_account.is_null() {
                    let account_ptr = trading_account as *const TradingAccountField;
                    Some((*account_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_trading_account(
                    parsed_trading_account,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_qry_investor_position_callback(
    user_data: *mut c_void,
    investor_position: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_investor_position = if !investor_position.is_null() {
                    let position_ptr = investor_position as *const InvestorPositionField;
                    Some((*position_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_investor_position(
                    parsed_investor_position,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

// 第一阶段新增回调函数

extern "C" fn on_err_rtn_order_insert_callback(
    user_data: *mut c_void,
    input_order: *mut c_void,
    rsp_info: *mut c_void,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_input_order = if !input_order.is_null() {
                    let order_ptr = input_order as *const InputOrderField;
                    Some((*order_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_err_rtn_order_insert(parsed_input_order, parsed_rsp_info);
            }
        }
    }
}

extern "C" fn on_err_rtn_order_action_callback(
    user_data: *mut c_void,
    order_action: *mut c_void,
    rsp_info: *mut c_void,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_order_action = if !order_action.is_null() {
                    let action_ptr = order_action as *const OrderActionField;
                    Some((*action_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_err_rtn_order_action(parsed_order_action, parsed_rsp_info);
            }
        }
    }
}

extern "C" fn on_rsp_qry_order_callback(
    user_data: *mut c_void,
    order: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_order = if !order.is_null() {
                    let order_ptr = order as *const OrderField;
                    Some((*order_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_order(parsed_order, parsed_rsp_info, request_id, is_last != 0);
            }
        }
    }
}

extern "C" fn on_rsp_qry_trade_callback(
    user_data: *mut c_void,
    trade: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_trade = if !trade.is_null() {
                    let trade_ptr = trade as *const TradeField;
                    Some((*trade_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_trade(parsed_trade, parsed_rsp_info, request_id, is_last != 0);
            }
        }
    }
}

extern "C" fn on_rsp_qry_instrument_callback(
    user_data: *mut c_void,
    instrument: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_instrument = if !instrument.is_null() {
                    let instrument_ptr = instrument as *const InstrumentField;
                    Some((*instrument_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_instrument(
                    parsed_instrument,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

// 第二阶段新增回调函数

extern "C" fn on_rsp_qry_instrument_margin_rate_callback(
    user_data: *mut c_void,
    margin_rate: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_margin_rate = if !margin_rate.is_null() {
                    let margin_rate_ptr = margin_rate as *const InstrumentMarginRateField;
                    Some((*margin_rate_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_instrument_margin_rate(
                    parsed_margin_rate,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_qry_instrument_commission_rate_callback(
    user_data: *mut c_void,
    commission_rate: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_commission_rate = if !commission_rate.is_null() {
                    let commission_rate_ptr =
                        commission_rate as *const InstrumentCommissionRateField;
                    Some((*commission_rate_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_instrument_commission_rate(
                    parsed_commission_rate,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_qry_exchange_callback(
    user_data: *mut c_void,
    exchange: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_exchange = if !exchange.is_null() {
                    let exchange_ptr = exchange as *const ExchangeField;
                    Some((*exchange_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_exchange(
                    parsed_exchange,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_qry_product_callback(
    user_data: *mut c_void,
    product: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_product = if !product.is_null() {
                    let product_ptr = product as *const ProductField;
                    Some((*product_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_product(
                    parsed_product,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_settlement_info_confirm_callback(
    user_data: *mut c_void,
    settlement_info_confirm: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_settlement_info_confirm = if !settlement_info_confirm.is_null() {
                    let settlement_ptr =
                        settlement_info_confirm as *const SettlementInfoConfirmField;
                    Some((*settlement_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_settlement_info_confirm(
                    parsed_settlement_info_confirm,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_parked_order_insert_callback(
    user_data: *mut c_void,
    parked_order: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_parked_order = if !parked_order.is_null() {
                    let parked_order_ptr = parked_order as *const ParkedOrderField;
                    Some((*parked_order_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_parked_order_insert(
                    parsed_parked_order,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_parked_order_action_callback(
    user_data: *mut c_void,
    parked_order_action: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_parked_order_action = if !parked_order_action.is_null() {
                    let parked_action_ptr = parked_order_action as *const ParkedOrderActionField;
                    Some((*parked_action_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_parked_order_action(
                    parsed_parked_order_action,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

// 第三阶段回调函数实现

extern "C" fn on_rsp_exec_order_insert_callback(
    user_data: *mut c_void,
    input_exec_order: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_input_exec_order = if !input_exec_order.is_null() {
                    let exec_order_ptr = input_exec_order as *const InputExecOrderField;
                    Some((*exec_order_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_exec_order_insert(
                    parsed_input_exec_order,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_exec_order_action_callback(
    user_data: *mut c_void,
    input_exec_order_action: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_input_exec_order_action = if !input_exec_order_action.is_null() {
                    let exec_action_ptr =
                        input_exec_order_action as *const InputExecOrderActionField;
                    Some((*exec_action_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_exec_order_action(
                    parsed_input_exec_order_action,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_for_quote_insert_callback(
    user_data: *mut c_void,
    input_for_quote: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_input_for_quote = if !input_for_quote.is_null() {
                    let for_quote_ptr = input_for_quote as *const InputForQuoteField;
                    Some((*for_quote_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_for_quote_insert(
                    parsed_input_for_quote,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_quote_insert_callback(
    user_data: *mut c_void,
    input_quote: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_input_quote = if !input_quote.is_null() {
                    let quote_ptr = input_quote as *const InputQuoteField;
                    Some((*quote_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_quote_insert(
                    parsed_input_quote,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_quote_action_callback(
    user_data: *mut c_void,
    input_quote_action: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_input_quote_action = if !input_quote_action.is_null() {
                    let quote_action_ptr = input_quote_action as *const InputQuoteActionField;
                    Some((*quote_action_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_quote_action(
                    parsed_input_quote_action,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_batch_order_action_callback(
    user_data: *mut c_void,
    input_batch_order_action: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_input_batch_order_action = if !input_batch_order_action.is_null() {
                    let batch_action_ptr =
                        input_batch_order_action as *const InputBatchOrderActionField;
                    Some((*batch_action_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_batch_order_action(
                    parsed_input_batch_order_action,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_remove_parked_order_callback(
    user_data: *mut c_void,
    remove_parked_order: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_remove_parked_order = if !remove_parked_order.is_null() {
                    let remove_ptr = remove_parked_order as *const RemoveParkedOrderField;
                    Some((*remove_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_remove_parked_order(
                    parsed_remove_parked_order,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_remove_parked_order_action_callback(
    user_data: *mut c_void,
    remove_parked_order_action: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_remove_parked_order_action = if !remove_parked_order_action.is_null() {
                    let remove_action_ptr =
                        remove_parked_order_action as *const RemoveParkedOrderActionField;
                    Some((*remove_action_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_remove_parked_order_action(
                    parsed_remove_parked_order_action,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_qry_max_order_volume_callback(
    user_data: *mut c_void,
    qry_max_order_volume: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_qry_max_order_volume = if !qry_max_order_volume.is_null() {
                    let max_volume_ptr = qry_max_order_volume as *const QryMaxOrderVolumeField;
                    Some((*max_volume_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_max_order_volume(
                    parsed_qry_max_order_volume,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_qry_depth_market_data_callback(
    user_data: *mut c_void,
    depth_market_data: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_depth_market_data = if !depth_market_data.is_null() {
                    let market_data_ptr = depth_market_data as *const DepthMarketDataField;
                    Some((*market_data_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_depth_market_data(
                    parsed_depth_market_data,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_qry_settlement_info_callback(
    user_data: *mut c_void,
    settlement_info: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_settlement_info = if !settlement_info.is_null() {
                    let settlement_ptr = settlement_info as *const SettlementInfoField;
                    Some((*settlement_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_settlement_info(
                    parsed_settlement_info,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_qry_transfer_bank_callback(
    user_data: *mut c_void,
    transfer_bank: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_transfer_bank = if !transfer_bank.is_null() {
                    let bank_ptr = transfer_bank as *const TransferBankField;
                    Some((*bank_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_transfer_bank(
                    parsed_transfer_bank,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_qry_investor_position_detail_callback(
    user_data: *mut c_void,
    investor_position_detail: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_investor_position_detail = if !investor_position_detail.is_null() {
                    let detail_ptr = investor_position_detail as *const InvestorPositionDetailField;
                    Some((*detail_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_investor_position_detail(
                    parsed_investor_position_detail,
                    parsed_rsp_info,
                    request_id,
                    is_last != 0,
                );
            }
        }
    }
}

extern "C" fn on_rsp_qry_notice_callback(
    user_data: *mut c_void,
    notice: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: c_int,
) {
    unsafe {
        if let Some(api) = (user_data as *mut TraderApi).as_mut() {
            if let Some(ref mut handler) = api.handler {
                let parsed_notice = if !notice.is_null() {
                    let notice_ptr = notice as *const NoticeField;
                    Some((*notice_ptr).clone())
                } else {
                    None
                };

                let parsed_rsp_info = if !rsp_info.is_null() {
                    let info_ptr = rsp_info as *const RspInfoField;
                    Some((*info_ptr).clone())
                } else {
                    None
                };

                handler.on_rsp_qry_notice(parsed_notice, parsed_rsp_info, request_id, is_last != 0);
            }
        }
    }
}

impl Drop for TraderApi {
    fn drop(&mut self) {
        self.release();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        // 测试版本获取函数的调用
        match TraderApi::get_version() {
            Ok(version) => eprintln!("版本: {}", version),
            Err(e) => eprintln!("获取版本失败: {}", e),
        }
    }
}
