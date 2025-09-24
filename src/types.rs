//! CTP数据类型定义
//!
//! 对应CTP C++库中的数据类型，提供安全的Rust封装

use crate::encoding::GbkConverter;
use crate::error::CtpResult;

/// 交易员代码类型 (21字符)
pub type TraderIdType = [u8; 21];

/// 投资者代码类型 (13字符)
pub type InvestorIdType = [u8; 13];

/// 经纪公司代码类型 (11字符)
pub type BrokerIdType = [u8; 11];

/// 经纪公司简称类型 (9字符)
pub type BrokerAbbrType = [u8; 9];

/// 经纪公司名称类型 (81字符)
pub type BrokerNameType = [u8; 81];

/// 合约代码类型 (31字符)
pub type InstrumentIdType = [u8; 31];

/// 密码类型 (41字符)
pub type PasswordType = [u8; 41];

/// 用户代码类型 (16字符)
pub type UserIdType = [u8; 16];

/// 产品信息类型 (11字符)
pub type ProductInfoType = [u8; 11];

/// 币种代码类型 (4字符)
pub type CurrencyIdType = [u8; 4];

/// 业务类型 (1字符)
pub type BizTypeType = [u8; 1];

/// 投资者账户代码类型 (13字符)
pub type AccountIdType = [u8; 13];

/// 交易所代码类型 (9字符)
pub type ExchangeIdType = [u8; 9];

/// 投资单元代码类型 (17字符)
pub type InvestUnitIdType = [u8; 17];

/// 协议信息类型 (11字符)
pub type ProtocolInfoType = [u8; 11];

/// Mac地址类型 (21字符)
pub type MacAddressType = [u8; 21];

/// 认证码类型 (17字符)
pub type AuthCodeType = [u8; 17];

/// 应用单元代码类型 (21字符)
pub type AppIdType = [u8; 21];

/// 客户端IP地址类型 (16字符)
pub type IpAddressType = [u8; 16];

/// 客户端IP端口类型 (6字符)
pub type IpPortType = [u8; 6];

/// 第一阶段新增类型定义

/// 报单编号类型 (21字符)
pub type OrderSysIdType = [u8; 21];

/// 时间类型 (9字符)
pub type TimeType = [u8; 9];

/// 成交编号类型 (21字符)
pub type TradeIdType = [u8; 21];

/// 合约在交易所的代码类型 (31字符)
pub type ExchangeInstIdType = [u8; 31];

/// 报单操作引用类型 (13字符)
pub type OrderActionRefType = [u8; 13];

/// 操作标志类型
pub type ActionFlagType = u8;

/// IP地址类型 (16字符)
pub type IPAddressType = [u8; 16];

/// 报单引用类型 (13字符)
pub type OrderRefType = [u8; 13];

/// 请求编号类型
pub type RequestIdType = i32;

/// 前置编号类型
pub type FrontIdType = i32;

/// 会话编号类型
pub type SessionIdType = i32;

/// 业务单元类型
pub type BusinessUnitType = [u8; 21];
/// 客户代码类型
pub type ClientIdType = [u8; 11];
/// 预埋单编号类型
pub type ParkedOrderIdType = [u8; 13];
/// 交易日类型
pub type TradingDayType = [u8; 9];
/// 银行代码类型
pub type BankIdType = [u8; 4];
/// 银行分支机构代码类型
pub type BankBrchIdType = [u8; 5];
/// 银行名称类型
pub type BankNameType = [u8; 101];

/// 价格类型
pub type PriceType = f64;

/// 日期类型 (9字符)
pub type DateType = [u8; 9];

/// 安装编号类型
pub type InstallIdType = i32;

/// 本地报单编号类型 (13字符)
pub type OrderLocalIdType = [u8; 13];

/// 会员代码类型 (11字符)
pub type ParticipantIdType = [u8; 11];

/// 报单操作状态类型
pub type OrderActionStatusType = u8;

/// 错误信息类型 (81字符)
pub type ErrorMsgType = [u8; 81];

/// 营业部编号类型 (9字符)
pub type BranchIdType = [u8; 9];

/// 数量类型
pub type VolumeType = i32;

/// 恢复类型枚举
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResumeType {
    /// 重新开始
    Restart = 0,
    /// 从本交易日开始传回
    Resume = 1,
    /// 从上次收到的续传
    Quick = 2,
    /// 只传送登录后的流内容
    None = 3,
}

impl Default for ResumeType {
    fn default() -> Self {
        ResumeType::Restart
    }
}

/// 字符串转换特质
///
/// 为固定长度的字节数组提供UTF-8字符串转换功能
pub trait StringConvert {
    /// 将字节数组转换为UTF-8字符串
    fn to_utf8_string(&self) -> CtpResult<String>;

    /// 从UTF-8字符串创建字节数组
    fn from_utf8_string(s: &str) -> CtpResult<Self>
    where
        Self: Sized;
}

macro_rules! impl_string_convert {
    ($type:ty, $size:expr) => {
        impl StringConvert for $type {
            fn to_utf8_string(&self) -> CtpResult<String> {
                GbkConverter::fixed_bytes_to_utf8(self)
            }

            fn from_utf8_string(s: &str) -> CtpResult<Self> {
                GbkConverter::utf8_to_fixed_bytes::<$size>(s)
            }
        }
    };
}

// 为所有固定长度类型实现字符串转换
impl_string_convert!([u8; 21], 21); // TraderIdType, MacAddressType, AppIdType, OrderSysIdType, TradeIdType
impl_string_convert!([u8; 13], 13); // InvestorIdType, AccountIdType, OrderActionRefType, OrderRefType
impl_string_convert!([u8; 11], 11); // BrokerIdType
impl_string_convert!([u8; 9], 9); // BrokerAbbrType, ExchangeIdType, TimeType
impl_string_convert!([u8; 81], 81); // BrokerNameType
impl_string_convert!([u8; 31], 31); // InstrumentIdType, ExchangeInstIdType
impl_string_convert!([u8; 41], 41); // PasswordType
impl_string_convert!([u8; 17], 17); // AuthCodeType, InvestUnitIdType
impl_string_convert!([u8; 16], 16); // UserIdType, IpAddressType, IPAddressType
impl_string_convert!([u8; 6], 6); // IpPortType
impl_string_convert!([u8; 4], 4); // CurrencyIdType
impl_string_convert!([u8; 1], 1); // BizTypeType

/// 用户登录请求
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ReqUserLoginField {
    /// 交易日
    pub trading_day: [u8; 9],
    /// 经纪公司代码
    pub broker_id: BrokerIdType,
    /// 用户代码
    pub user_id: UserIdType,
    /// 密码
    pub password: PasswordType,
    /// 用户端产品信息
    pub user_product_info: ProductInfoType,
    /// 接口端产品信息
    pub interface_product_info: ProductInfoType,
    /// 协议信息
    pub protocol_info: ProtocolInfoType,
    /// Mac地址
    pub mac_address: MacAddressType,
    /// 动态密码
    pub one_time_password: PasswordType,
    /// 客户端IP地址
    pub client_ip_address: IpAddressType,
    /// 客户端IP端口
    pub client_ip_port: IpPortType,
    /// 登录备注
    pub login_remark: [u8; 36],
}

impl Default for ReqUserLoginField {
    fn default() -> Self {
        Self {
            trading_day: [0; 9],
            broker_id: [0; 11],
            user_id: [0; 16],
            password: [0; 41],
            user_product_info: [0; 11],
            interface_product_info: [0; 11],
            protocol_info: [0; 11],
            mac_address: [0; 21],
            one_time_password: [0; 41],
            client_ip_address: [0; 16],
            client_ip_port: [0; 6],
            login_remark: [0; 36],
        }
    }
}

impl ReqUserLoginField {
    /// 创建登录请求
    pub fn new(broker_id: &str, user_id: &str, password: &str) -> CtpResult<Self> {
        let mut req = Self::default();

        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.user_id = UserIdType::from_utf8_string(user_id)?;
        req.password = PasswordType::from_utf8_string(password)?;

        Ok(req)
    }

    /// 设置产品信息
    pub fn with_product_info(mut self, product_info: &str) -> CtpResult<Self> {
        self.user_product_info = ProductInfoType::from_utf8_string(product_info)?;
        self.interface_product_info = ProductInfoType::from_utf8_string(product_info)?;
        Ok(self)
    }

    /// 设置认证码
    pub fn with_auth_code(mut self, auth_code: &str) -> CtpResult<Self> {
        self.one_time_password = PasswordType::from_utf8_string(auth_code)?;
        Ok(self)
    }

    /// 设置Mac地址
    pub fn with_mac_address(mut self, mac_address: &str) -> CtpResult<Self> {
        self.mac_address = MacAddressType::from_utf8_string(mac_address)?;
        Ok(self)
    }

    /// 设置客户端IP
    pub fn with_client_ip(mut self, ip: &str, port: &str) -> CtpResult<Self> {
        self.client_ip_address = IpAddressType::from_utf8_string(ip)?;
        self.client_ip_port = IpPortType::from_utf8_string(port)?;
        Ok(self)
    }
}

/// 用户登录响应
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RspUserLoginField {
    /// 交易日
    pub trading_day: [u8; 9],
    /// 登录成功时间
    pub login_time: [u8; 9],
    /// 经纪公司代码
    pub broker_id: BrokerIdType,
    /// 用户代码
    pub user_id: UserIdType,
    /// 交易系统名称
    pub system_name: [u8; 41],
    /// 前置编号
    pub front_id: i32,
    /// 会话编号
    pub session_id: i32,
    /// 最大报单引用
    pub max_order_ref: [u8; 13],
    /// 上期所时间
    pub shfe_time: [u8; 9],
    /// 大商所时间
    pub dce_time: [u8; 9],
    /// 郑商所时间
    pub czce_time: [u8; 9],
    /// 中金所时间
    pub ffex_time: [u8; 9],
    /// 能源中心时间
    pub ine_time: [u8; 9],
}

impl Default for RspUserLoginField {
    fn default() -> Self {
        Self {
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
        }
    }
}

/// 响应信息
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RspInfoField {
    /// 错误代码
    pub error_id: i32,
    /// 错误信息
    pub error_msg: [u8; 81],
}

impl Default for RspInfoField {
    fn default() -> Self {
        Self {
            error_id: 0,
            error_msg: [0; 81],
        }
    }
}

impl RspInfoField {
    /// 获取错误信息的UTF-8字符串
    pub fn get_error_msg(&self) -> CtpResult<String> {
        GbkConverter::fixed_bytes_to_utf8(&self.error_msg)
    }

    /// 检查是否成功（错误代码为0）
    pub fn is_success(&self) -> bool {
        self.error_id == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_convert() {
        let test_str = "测试用户";
        let user_id = UserIdType::from_utf8_string(test_str).unwrap();
        let converted = user_id.to_utf8_string().unwrap();
        assert_eq!(converted.trim_end_matches('\0'), test_str);
    }

    #[test]
    fn test_login_request_creation() {
        let req = ReqUserLoginField::new("9999", "investor1", "123456").unwrap();

        assert_eq!(
            req.broker_id
                .to_utf8_string()
                .unwrap()
                .trim_end_matches('\0'),
            "9999"
        );
        assert_eq!(
            req.user_id.to_utf8_string().unwrap().trim_end_matches('\0'),
            "investor1"
        );
        assert_eq!(
            req.password
                .to_utf8_string()
                .unwrap()
                .trim_end_matches('\0'),
            "123456"
        );
    }

    #[test]
    fn test_rsp_info_success() {
        let mut rsp = RspInfoField::default();
        assert!(rsp.is_success());

        rsp.error_id = -1;
        assert!(!rsp.is_success());
    }
}

// 查询资金账户字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryTradingAccountField {
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 币种代码
    pub currency_id: CurrencyIdType,
    // 业务类型
    pub biz_type: BizTypeType,
    // 投资者账户代码
    pub account_id: AccountIdType,
}

impl Default for QryTradingAccountField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryTradingAccountField {
    pub fn new(broker_id: &str, investor_id: &str) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        // 不设置币种代码，让CTP服务器使用默认值
        Ok(req)
    }

    pub fn with_currency(mut self, currency_id: &str) -> CtpResult<Self> {
        self.currency_id = CurrencyIdType::from_utf8_string(currency_id)?;
        Ok(self)
    }
}

// 查询投资者持仓字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryInvestorPositionField {
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 合约代码（为空表示查询所有）
    pub instrument_id: InstrumentIdType,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 投资单元代码
    pub invest_unit_id: InvestUnitIdType,
}

impl Default for QryInvestorPositionField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryInvestorPositionField {
    pub fn new(broker_id: &str, investor_id: &str) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        Ok(req)
    }

    pub fn with_instrument_id(mut self, instrument_id: &str) -> CtpResult<Self> {
        self.instrument_id = InstrumentIdType::from_utf8_string(instrument_id)?;
        Ok(self)
    }
}

// 第一阶段新增结构体

// 查询报单字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryOrderField {
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 合约代码
    pub instrument_id: InstrumentIdType,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 报单编号
    pub order_sys_id: OrderSysIdType,
    // 开始时间
    pub insert_time_start: TimeType,
    // 结束时间
    pub insert_time_end: TimeType,
    // 投资单元代码
    pub invest_unit_id: InvestUnitIdType,
}

impl Default for QryOrderField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryOrderField {
    pub fn new(broker_id: &str, investor_id: &str) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        Ok(req)
    }

    pub fn with_instrument_id(mut self, instrument_id: &str) -> CtpResult<Self> {
        self.instrument_id = InstrumentIdType::from_utf8_string(instrument_id)?;
        Ok(self)
    }
}

// 查询成交字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryTradeField {
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 合约代码
    pub instrument_id: InstrumentIdType,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 成交编号
    pub trade_id: TradeIdType,
    // 开始时间
    pub trade_time_start: TimeType,
    // 结束时间
    pub trade_time_end: TimeType,
    // 投资单元代码
    pub invest_unit_id: InvestUnitIdType,
}

impl Default for QryTradeField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryTradeField {
    pub fn new(broker_id: &str, investor_id: &str) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        Ok(req)
    }

    pub fn with_instrument_id(mut self, instrument_id: &str) -> CtpResult<Self> {
        self.instrument_id = InstrumentIdType::from_utf8_string(instrument_id)?;
        Ok(self)
    }
}

// 查询合约字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryInstrumentField {
    // 合约代码
    pub instrument_id: InstrumentIdType,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 合约在交易所的代码
    pub exchange_inst_id: ExchangeInstIdType,
    // 产品代码
    pub product_id: InstrumentIdType,
}

impl Default for QryInstrumentField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryInstrumentField {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_instrument_id(mut self, instrument_id: &str) -> CtpResult<Self> {
        self.instrument_id = InstrumentIdType::from_utf8_string(instrument_id)?;
        Ok(self)
    }

    pub fn with_exchange_id(mut self, exchange_id: &str) -> CtpResult<Self> {
        self.exchange_id = ExchangeIdType::from_utf8_string(exchange_id)?;
        Ok(self)
    }
}

// 报单操作字段（撤单使用）
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputOrderActionField {
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 报单操作引用
    pub order_action_ref: OrderActionRefType,
    // 报单引用
    pub order_ref: OrderRefType,
    // 请求编号
    pub request_id: RequestIdType,
    // 前置编号
    pub front_id: FrontIdType,
    // 会话编号
    pub session_id: SessionIdType,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 报单编号
    pub order_sys_id: OrderSysIdType,
    // 操作标志
    pub action_flag: ActionFlagType,
    // 价格
    pub limit_price: PriceType,
    // 数量变化
    pub volume_change: VolumeType,
    // 用户代码
    pub user_id: UserIdType,
    // 合约代码
    pub instrument_id: InstrumentIdType,
    // 投资单元代码
    pub invest_unit_id: InvestUnitIdType,
    // IP地址
    pub ip_address: IPAddressType,
    // Mac地址
    pub mac_address: MacAddressType,
}

impl Default for InputOrderActionField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl InputOrderActionField {
    pub fn new(
        broker_id: &str,
        investor_id: &str,
        action_flag: u8,
        order_ref: &str,
        front_id: i32,
        session_id: i32,
        exchange_id: &str,
        order_sys_id: &str,
    ) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.action_flag = action_flag;
        req.order_ref = OrderRefType::from_utf8_string(order_ref)?;
        req.front_id = front_id;
        req.session_id = session_id;
        req.exchange_id = ExchangeIdType::from_utf8_string(exchange_id)?;
        req.order_sys_id = OrderSysIdType::from_utf8_string(order_sys_id)?;
        Ok(req)
    }
}

// 报单操作回报字段（用于错误回报）
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OrderActionField {
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 报单操作引用
    pub order_action_ref: OrderActionRefType,
    // 报单引用
    pub order_ref: OrderRefType,
    // 请求编号
    pub request_id: RequestIdType,
    // 前置编号
    pub front_id: FrontIdType,
    // 会话编号
    pub session_id: SessionIdType,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 报单编号
    pub order_sys_id: OrderSysIdType,
    // 操作标志
    pub action_flag: ActionFlagType,
    // 价格
    pub limit_price: PriceType,
    // 数量变化
    pub volume_change: VolumeType,
    // 操作日期
    pub action_date: DateType,
    // 操作时间
    pub action_time: TimeType,
    // 交易所交易员代码
    pub trader_id: TraderIdType,
    // 安装编号
    pub install_id: InstallIdType,
    // 本地报单编号
    pub order_local_id: OrderLocalIdType,
    // 操作本地编号
    pub action_local_id: OrderLocalIdType,
    // 会员代码
    pub participant_id: ParticipantIdType,
    // 客户代码
    pub client_id: ClientIdType,
    // 业务单元
    pub business_unit: BusinessUnitType,
    // 报单操作状态
    pub order_action_status: OrderActionStatusType,
    // 用户代码
    pub user_id: UserIdType,
    // 状态信息
    pub status_msg: ErrorMsgType,
    // 合约代码
    pub instrument_id: InstrumentIdType,
    // 营业部编号
    pub branch_id: BranchIdType,
    // 投资单元代码
    pub invest_unit_id: InvestUnitIdType,
    // IP地址
    pub ip_address: IPAddressType,
    // Mac地址
    pub mac_address: MacAddressType,
}

impl Default for OrderActionField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 第二阶段新增结构体

// 查询合约保证金率字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryInstrumentMarginRateField {
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 合约代码
    pub instrument_id: InstrumentIdType,
    // 投机套保标志
    pub hedge_flag: u8,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 投资单元代码
    pub invest_unit_id: InvestUnitIdType,
}

impl Default for QryInstrumentMarginRateField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryInstrumentMarginRateField {
    pub fn new(broker_id: &str, investor_id: &str, instrument_id: &str) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.instrument_id = InstrumentIdType::from_utf8_string(instrument_id)?;
        Ok(req)
    }

    pub fn with_hedge_flag(mut self, hedge_flag: u8) -> Self {
        self.hedge_flag = hedge_flag;
        self
    }

    pub fn with_exchange_id(mut self, exchange_id: &str) -> CtpResult<Self> {
        self.exchange_id = ExchangeIdType::from_utf8_string(exchange_id)?;
        Ok(self)
    }
}

// 查询合约手续费率字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryInstrumentCommissionRateField {
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 合约代码
    pub instrument_id: InstrumentIdType,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 投资单元代码
    pub invest_unit_id: InvestUnitIdType,
}

impl Default for QryInstrumentCommissionRateField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryInstrumentCommissionRateField {
    pub fn new(broker_id: &str, investor_id: &str, instrument_id: &str) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.instrument_id = InstrumentIdType::from_utf8_string(instrument_id)?;
        Ok(req)
    }

    pub fn with_exchange_id(mut self, exchange_id: &str) -> CtpResult<Self> {
        self.exchange_id = ExchangeIdType::from_utf8_string(exchange_id)?;
        Ok(self)
    }
}

// 查询交易所字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryExchangeField {
    // 交易所代码
    pub exchange_id: ExchangeIdType,
}

impl Default for QryExchangeField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryExchangeField {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_exchange_id(mut self, exchange_id: &str) -> CtpResult<Self> {
        self.exchange_id = ExchangeIdType::from_utf8_string(exchange_id)?;
        Ok(self)
    }
}

// 查询产品字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryProductField {
    // 产品代码
    pub product_id: InstrumentIdType,
    // 产品类型
    pub product_class: u8,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
}

impl Default for QryProductField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryProductField {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_product_id(mut self, product_id: &str) -> CtpResult<Self> {
        self.product_id = InstrumentIdType::from_utf8_string(product_id)?;
        Ok(self)
    }

    pub fn with_exchange_id(mut self, exchange_id: &str) -> CtpResult<Self> {
        self.exchange_id = ExchangeIdType::from_utf8_string(exchange_id)?;
        Ok(self)
    }

    pub fn with_product_class(mut self, product_class: u8) -> Self {
        self.product_class = product_class;
        self
    }
}

// 结算信息确认字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SettlementInfoConfirmField {
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 确认日期
    pub confirm_date: [u8; 9],
    // 确认时间
    pub confirm_time: [u8; 9],
    // 结算编号
    pub settlement_id: i32,
    // 投资者账户代码
    pub account_id: AccountIdType,
    // 币种代码
    pub currency_id: CurrencyIdType,
}

impl Default for SettlementInfoConfirmField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl SettlementInfoConfirmField {
    pub fn new(broker_id: &str, investor_id: &str) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        Ok(req)
    }

    pub fn with_account_id(mut self, account_id: &str) -> CtpResult<Self> {
        self.account_id = AccountIdType::from_utf8_string(account_id)?;
        Ok(self)
    }
}

// 预埋单字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ParkedOrderField {
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 合约代码
    pub instrument_id: InstrumentIdType,
    // 报单引用
    pub order_ref: OrderRefType,
    // 用户代码
    pub user_id: UserIdType,
    // 报单价格条件
    pub order_price_type: u8,
    // 买卖方向
    pub direction: u8,
    // 组合开平标志
    pub comb_offset_flag: [u8; 5],
    // 组合投机套保标志
    pub comb_hedge_flag: [u8; 5],
    // 价格
    pub limit_price: PriceType,
    // 数量
    pub volume_total_original: VolumeType,
    // 有效期类型
    pub time_condition: u8,
    // GTD日期
    pub gtd_date: [u8; 9],
    // 成交量类型
    pub volume_condition: u8,
    // 最小成交量
    pub min_volume: VolumeType,
    // 触发条件
    pub contingent_condition: u8,
    // 止损价
    pub stop_price: PriceType,
    // 强平原因
    pub force_close_reason: u8,
    // 自动挂起标志
    pub is_auto_suspend: i32,
    // 业务单元
    pub business_unit: [u8; 21],
    // 请求编号
    pub request_id: RequestIdType,
    // 用户强平标志
    pub user_force_close: i32,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 预埋报单编号
    pub parked_order_id: [u8; 13],
    // 用户类型
    pub user_type: u8,
    // 预埋单状态
    pub status: u8,
    // 错误代码
    pub error_id: i32,
    // 错误信息
    pub error_msg: [u8; 81],
    // 互换单标志
    pub is_swap_order: i32,
    // 投资单元代码
    pub invest_unit_id: InvestUnitIdType,
    // 资金账号
    pub account_id: AccountIdType,
    // 币种代码
    pub currency_id: CurrencyIdType,
    // 客户代码
    pub client_id: [u8; 11],
    // IP地址
    pub ip_address: IPAddressType,
    // Mac地址
    pub mac_address: MacAddressType,
}

impl Default for ParkedOrderField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl ParkedOrderField {
    pub fn new(
        broker_id: &str,
        investor_id: &str,
        instrument_id: &str,
        order_ref: &str,
        direction: u8,
        limit_price: f64,
        volume: i32,
    ) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.instrument_id = InstrumentIdType::from_utf8_string(instrument_id)?;
        req.order_ref = OrderRefType::from_utf8_string(order_ref)?;
        req.direction = direction;
        req.limit_price = limit_price;
        req.volume_total_original = volume;
        Ok(req)
    }
}

// 预埋单操作字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ParkedOrderActionField {
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 报单操作引用
    pub order_action_ref: OrderActionRefType,
    // 报单引用
    pub order_ref: OrderRefType,
    // 请求编号
    pub request_id: RequestIdType,
    // 前置编号
    pub front_id: FrontIdType,
    // 会话编号
    pub session_id: SessionIdType,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 报单编号
    pub order_sys_id: OrderSysIdType,
    // 操作标志
    pub action_flag: ActionFlagType,
    // 价格
    pub limit_price: PriceType,
    // 数量变化
    pub volume_change: VolumeType,
    // 用户代码
    pub user_id: UserIdType,
    // 合约代码
    pub instrument_id: InstrumentIdType,
    // 预埋撤单单编号
    pub parked_order_action_id: [u8; 13],
    // 用户类型
    pub user_type: u8,
    // 预埋撤单状态
    pub status: u8,
    // 错误代码
    pub error_id: i32,
    // 错误信息
    pub error_msg: [u8; 81],
    // 投资单元代码
    pub invest_unit_id: InvestUnitIdType,
    // IP地址
    pub ip_address: IPAddressType,
    // Mac地址
    pub mac_address: MacAddressType,
}

impl Default for ParkedOrderActionField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl ParkedOrderActionField {
    pub fn new(
        broker_id: &str,
        investor_id: &str,
        action_flag: u8,
        order_ref: &str,
        front_id: i32,
        session_id: i32,
        exchange_id: &str,
        order_sys_id: &str,
    ) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.action_flag = action_flag;
        req.order_ref = OrderRefType::from_utf8_string(order_ref)?;
        req.front_id = front_id;
        req.session_id = session_id;
        req.exchange_id = ExchangeIdType::from_utf8_string(exchange_id)?;
        req.order_sys_id = OrderSysIdType::from_utf8_string(order_sys_id)?;
        Ok(req)
    }
}

// 第二阶段响应字段定义

// 合约保证金率字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InstrumentMarginRateField {
    // 合约代码
    pub instrument_id: InstrumentIdType,
    // 投资者范围
    pub investor_range: u8,
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 投机套保标志
    pub hedge_flag: u8,
    // 多头保证金率
    pub long_margin_ratio_by_money: f64,
    // 多头保证金费
    pub long_margin_ratio_by_volume: f64,
    // 空头保证金率
    pub short_margin_ratio_by_money: f64,
    // 空头保证金费
    pub short_margin_ratio_by_volume: f64,
    // 是否相对交易所收取
    pub is_relative: i32,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 投资单元代码
    pub invest_unit_id: InvestUnitIdType,
}

impl Default for InstrumentMarginRateField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 合约手续费率字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InstrumentCommissionRateField {
    // 合约代码
    pub instrument_id: InstrumentIdType,
    // 投资者范围
    pub investor_range: u8,
    // 经纪公司代码
    pub broker_id: BrokerIdType,
    // 投资者代码
    pub investor_id: InvestorIdType,
    // 开仓手续费率
    pub open_ratio_by_money: f64,
    // 开仓手续费
    pub open_ratio_by_volume: f64,
    // 平仓手续费率
    pub close_ratio_by_money: f64,
    // 平仓手续费
    pub close_ratio_by_volume: f64,
    // 平今手续费率
    pub close_today_ratio_by_money: f64,
    // 平今手续费
    pub close_today_ratio_by_volume: f64,
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 业务类型
    pub biz_type: u8,
    // 投资单元代码
    pub invest_unit_id: InvestUnitIdType,
}

impl Default for InstrumentCommissionRateField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 交易所字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExchangeField {
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 交易所名称
    pub exchange_name: [u8; 61],
    // 交易所属性
    pub exchange_property: u8,
}

impl Default for ExchangeField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 产品字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ProductField {
    // 产品代码
    pub product_id: InstrumentIdType,
    // 产品名称
    pub product_name: [u8; 21],
    // 交易所代码
    pub exchange_id: ExchangeIdType,
    // 产品类型
    pub product_class: u8,
    // 合约数量乘数
    pub volume_multiple: i32,
    // 最小变动价位
    pub price_tick: f64,
    // 市价单最大下单量
    pub max_market_order_volume: i32,
    // 市价单最小下单量
    pub min_market_order_volume: i32,
    // 限价单最大下单量
    pub max_limit_order_volume: i32,
    // 限价单最小下单量
    pub min_limit_order_volume: i32,
    // 持仓类型
    pub position_type: u8,
    // 持仓日期类型
    pub position_date_type: u8,
    // 平仓处理类型
    pub close_deal_type: u8,
    // 交易币种类型
    pub trade_currency_id: CurrencyIdType,
    // 保证金币种类型
    pub margin_currency_id: CurrencyIdType,
}

impl Default for ProductField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 第三阶段请求和响应字段定义

// 执行宣告录入字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputExecOrderField {
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub instrument_id: InstrumentIdType,
    pub exec_order_ref: OrderRefType,
    pub user_id: UserIdType,
    pub volume: i32,
    pub request_id: i32,
    pub business_unit: BusinessUnitType,
    pub offset_flag: u8,
    pub hedge_flag: u8,
    pub action_type: u8,
    pub posidir: u8,
    pub reserve_position_flag: u8,
    pub close_flag: u8,
    pub exchange_id: ExchangeIdType,
    pub invest_unit_id: InvestUnitIdType,
    pub account_id: AccountIdType,
    pub currency_id: CurrencyIdType,
    pub client_id: ClientIdType,
    pub ip_address: [u8; 16],
    pub mac_address: [u8; 21],
}

impl Default for InputExecOrderField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl InputExecOrderField {
    pub fn new(
        broker_id: &str,
        investor_id: &str,
        instrument_id: &str,
        exec_order_ref: &str,
        user_id: &str,
        volume: i32,
        action_type: u8,
    ) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.instrument_id = InstrumentIdType::from_utf8_string(instrument_id)?;
        req.exec_order_ref = OrderRefType::from_utf8_string(exec_order_ref)?;
        req.user_id = UserIdType::from_utf8_string(user_id)?;
        req.volume = volume;
        req.action_type = action_type;
        Ok(req)
    }
}

// 执行宣告操作字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputExecOrderActionField {
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub exec_order_action_ref: OrderRefType,
    pub exec_order_ref: OrderRefType,
    pub request_id: i32,
    pub front_id: i32,
    pub session_id: i32,
    pub exchange_id: ExchangeIdType,
    pub exec_order_sys_id: OrderSysIdType,
    pub action_flag: u8,
    pub user_id: UserIdType,
    pub instrument_id: InstrumentIdType,
    pub invest_unit_id: InvestUnitIdType,
    pub ip_address: [u8; 16],
    pub mac_address: [u8; 21],
}

impl Default for InputExecOrderActionField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl InputExecOrderActionField {
    pub fn new(
        broker_id: &str,
        investor_id: &str,
        exec_order_action_ref: &str,
        exec_order_ref: &str,
        user_id: &str,
        action_flag: u8,
    ) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.exec_order_action_ref = OrderRefType::from_utf8_string(exec_order_action_ref)?;
        req.exec_order_ref = OrderRefType::from_utf8_string(exec_order_ref)?;
        req.user_id = UserIdType::from_utf8_string(user_id)?;
        req.action_flag = action_flag;
        Ok(req)
    }
}

// 询价录入字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputForQuoteField {
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub instrument_id: InstrumentIdType,
    pub for_quote_ref: OrderRefType,
    pub user_id: UserIdType,
    pub exchange_id: ExchangeIdType,
    pub invest_unit_id: InvestUnitIdType,
    pub ip_address: [u8; 16],
    pub mac_address: [u8; 21],
}

impl Default for InputForQuoteField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl InputForQuoteField {
    pub fn new(
        broker_id: &str,
        investor_id: &str,
        instrument_id: &str,
        for_quote_ref: &str,
        user_id: &str,
    ) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.instrument_id = InstrumentIdType::from_utf8_string(instrument_id)?;
        req.for_quote_ref = OrderRefType::from_utf8_string(for_quote_ref)?;
        req.user_id = UserIdType::from_utf8_string(user_id)?;
        Ok(req)
    }
}

// 报价录入字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputQuoteField {
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub instrument_id: InstrumentIdType,
    pub quote_ref: OrderRefType,
    pub user_id: UserIdType,
    pub ask_price: f64,
    pub bid_price: f64,
    pub ask_volume: i32,
    pub bid_volume: i32,
    pub request_id: i32,
    pub business_unit: BusinessUnitType,
    pub ask_offset_flag: u8,
    pub bid_offset_flag: u8,
    pub ask_hedge_flag: u8,
    pub bid_hedge_flag: u8,
    pub ask_order_ref: OrderRefType,
    pub bid_order_ref: OrderRefType,
    pub for_quote_sys_id: OrderSysIdType,
    pub exchange_id: ExchangeIdType,
    pub invest_unit_id: InvestUnitIdType,
    pub account_id: AccountIdType,
    pub currency_id: CurrencyIdType,
    pub client_id: ClientIdType,
    pub ip_address: [u8; 16],
    pub mac_address: [u8; 21],
}

impl Default for InputQuoteField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl InputQuoteField {
    pub fn new(
        broker_id: &str,
        investor_id: &str,
        instrument_id: &str,
        quote_ref: &str,
        user_id: &str,
        ask_price: f64,
        bid_price: f64,
        ask_volume: i32,
        bid_volume: i32,
    ) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.instrument_id = InstrumentIdType::from_utf8_string(instrument_id)?;
        req.quote_ref = OrderRefType::from_utf8_string(quote_ref)?;
        req.user_id = UserIdType::from_utf8_string(user_id)?;
        req.ask_price = ask_price;
        req.bid_price = bid_price;
        req.ask_volume = ask_volume;
        req.bid_volume = bid_volume;
        Ok(req)
    }
}

// 报价操作字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputQuoteActionField {
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub quote_action_ref: OrderRefType,
    pub quote_ref: OrderRefType,
    pub request_id: i32,
    pub front_id: i32,
    pub session_id: i32,
    pub exchange_id: ExchangeIdType,
    pub quote_sys_id: OrderSysIdType,
    pub action_flag: u8,
    pub user_id: UserIdType,
    pub instrument_id: InstrumentIdType,
    pub invest_unit_id: InvestUnitIdType,
    pub client_id: ClientIdType,
    pub ip_address: [u8; 16],
    pub mac_address: [u8; 21],
}

impl Default for InputQuoteActionField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl InputQuoteActionField {
    pub fn new(
        broker_id: &str,
        investor_id: &str,
        quote_action_ref: &str,
        quote_ref: &str,
        user_id: &str,
        action_flag: u8,
    ) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.quote_action_ref = OrderRefType::from_utf8_string(quote_action_ref)?;
        req.quote_ref = OrderRefType::from_utf8_string(quote_ref)?;
        req.user_id = UserIdType::from_utf8_string(user_id)?;
        req.action_flag = action_flag;
        Ok(req)
    }
}

// 批量报单操作字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputBatchOrderActionField {
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub order_action_ref: OrderRefType,
    pub request_id: i32,
    pub front_id: i32,
    pub session_id: i32,
    pub exchange_id: ExchangeIdType,
    pub user_id: UserIdType,
    pub invest_unit_id: InvestUnitIdType,
    pub ip_address: [u8; 16],
    pub mac_address: [u8; 21],
}

impl Default for InputBatchOrderActionField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl InputBatchOrderActionField {
    pub fn new(
        broker_id: &str,
        investor_id: &str,
        order_action_ref: &str,
        user_id: &str,
        exchange_id: &str,
    ) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.order_action_ref = OrderRefType::from_utf8_string(order_action_ref)?;
        req.user_id = UserIdType::from_utf8_string(user_id)?;
        req.exchange_id = ExchangeIdType::from_utf8_string(exchange_id)?;
        Ok(req)
    }
}

// 删除预埋单字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RemoveParkedOrderField {
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub parked_order_id: ParkedOrderIdType,
    pub invest_unit_id: InvestUnitIdType,
}

impl Default for RemoveParkedOrderField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl RemoveParkedOrderField {
    pub fn new(broker_id: &str, investor_id: &str, parked_order_id: &str) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.parked_order_id = ParkedOrderIdType::from_utf8_string(parked_order_id)?;
        Ok(req)
    }
}

// 删除预埋撤单字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RemoveParkedOrderActionField {
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub parked_order_action_id: ParkedOrderIdType,
    pub invest_unit_id: InvestUnitIdType,
}

impl Default for RemoveParkedOrderActionField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl RemoveParkedOrderActionField {
    pub fn new(
        broker_id: &str,
        investor_id: &str,
        parked_order_action_id: &str,
    ) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.parked_order_action_id = ParkedOrderIdType::from_utf8_string(parked_order_action_id)?;
        Ok(req)
    }
}

// 查询最大报单数量字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryMaxOrderVolumeField {
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub instrument_id: InstrumentIdType,
    pub direction: u8,
    pub offset_flag: u8,
    pub hedge_flag: u8,
    pub max_volume: i32,
    pub exchange_id: ExchangeIdType,
    pub invest_unit_id: InvestUnitIdType,
}

impl Default for QryMaxOrderVolumeField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryMaxOrderVolumeField {
    pub fn new(
        broker_id: &str,
        investor_id: &str,
        instrument_id: &str,
        direction: u8,
    ) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        req.instrument_id = InstrumentIdType::from_utf8_string(instrument_id)?;
        req.direction = direction;
        Ok(req)
    }
}

// 查询行情字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryDepthMarketDataField {
    pub instrument_id: InstrumentIdType,
    pub exchange_id: ExchangeIdType,
}

impl Default for QryDepthMarketDataField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryDepthMarketDataField {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_instrument_id(mut self, instrument_id: &str) -> CtpResult<Self> {
        self.instrument_id = InstrumentIdType::from_utf8_string(instrument_id)?;
        Ok(self)
    }
}

// 查询投资者结算结果字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QrySettlementInfoField {
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub trading_day: TradingDayType,
    pub account_id: AccountIdType,
    pub currency_id: CurrencyIdType,
}

impl Default for QrySettlementInfoField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QrySettlementInfoField {
    pub fn new(broker_id: &str, investor_id: &str) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        Ok(req)
    }
}

// 查询转帐银行字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryTransferBankField {
    pub bank_id: BankIdType,
    pub bank_brch_id: BankBrchIdType,
}

impl Default for QryTransferBankField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryTransferBankField {
    pub fn new() -> Self {
        Self::default()
    }
}

// 查询投资者持仓明细字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryInvestorPositionDetailField {
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub instrument_id: InstrumentIdType,
    pub exchange_id: ExchangeIdType,
    pub invest_unit_id: InvestUnitIdType,
}

impl Default for QryInvestorPositionDetailField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryInvestorPositionDetailField {
    pub fn new(broker_id: &str, investor_id: &str) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        req.investor_id = InvestorIdType::from_utf8_string(investor_id)?;
        Ok(req)
    }
}

// 查询客户通知字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QryNoticeField {
    pub broker_id: BrokerIdType,
}

impl Default for QryNoticeField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl QryNoticeField {
    pub fn new(broker_id: &str) -> CtpResult<Self> {
        let mut req = Self::default();
        req.broker_id = BrokerIdType::from_utf8_string(broker_id)?;
        Ok(req)
    }
}

// 第三阶段响应字段定义

// 投资者结算结果字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SettlementInfoField {
    pub trading_day: TradingDayType,
    pub settlement_id: i32,
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub sequence_no: i32,
    pub content: [u8; 1001],
    pub account_id: AccountIdType,
    pub currency_id: CurrencyIdType,
}

impl Default for SettlementInfoField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 转帐银行字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TransferBankField {
    pub bank_id: BankIdType,
    pub bank_brch_id: BankBrchIdType,
    pub bank_name: BankNameType,
    pub is_active: i32,
}

impl Default for TransferBankField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 投资者持仓明细字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InvestorPositionDetailField {
    pub instrument_id: InstrumentIdType,
    pub broker_id: BrokerIdType,
    pub investor_id: InvestorIdType,
    pub hedge_flag: u8,
    pub direction: u8,
    pub open_date: TradingDayType,
    pub trade_id: TradeIdType,
    pub volume: i32,
    pub open_price: f64,
    pub trading_day: TradingDayType,
    pub settlement_id: i32,
    pub trade_type: u8,
    pub comb_instrument_id: InstrumentIdType,
    pub exchange_id: ExchangeIdType,
    pub close_profit_by_date: f64,
    pub close_profit_by_trade: f64,
    pub position_profit_by_date: f64,
    pub position_profit_by_trade: f64,
    pub margin: f64,
    pub exch_margin: f64,
    pub margin_rate_by_money: f64,
    pub margin_rate_by_volume: f64,
    pub last_settlement_price: f64,
    pub settlement_price: f64,
    pub close_volume: i32,
    pub close_amount: f64,
    pub time_first_volume: i32,
    pub invest_unit_id: InvestUnitIdType,
    pub spec_posidir: u8,
}

impl Default for InvestorPositionDetailField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// 客户通知字段
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NoticeField {
    pub broker_id: BrokerIdType,
    pub content: [u8; 501],
    pub url_link: [u8; 201],
}

impl Default for NoticeField {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
