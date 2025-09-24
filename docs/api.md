# API 文档

## 概述

CTP Rust SDK 提供了一套完整的类型安全API，用于与CTP期货交易系统进行交互。本文档详细介绍了所有可用的API接口。

## 模块结构

### 核心模块

- [`api`](api/index.html) - 高级API接口
  - [`MdApi`](api/struct.MdApi.html) - 行情API
  - [`TraderApi`](api/struct.TraderApi.html) - 交易API
- [`types`](types/index.html) - 数据类型定义
- [`encoding`](encoding/index.html) - 编码转换工具
- [`error`](error/index.html) - 错误处理

## 行情API (MdApi)

### 创建和初始化

```rust
use deepissue_ctp_rust::api::{MdApi, CtpApi};

// 创建行情API实例
let mut md_api = MdApi::new(
    Some("./flow"),  // 流文件存储目录
    false,           // 是否使用UDP
    false            // 是否使用组播
)?;

// 注册前置机地址
md_api.register_front("tcp://180.168.146.187:10131")?;

// 初始化API
md_api.init()?;
```

### 主要方法

#### `new(flow_path, is_using_udp, is_multicast) -> CtpResult<MdApi>`

创建新的行情API实例。

**参数:**
- `flow_path: Option<&str>` - 流文件存储目录，None表示使用当前目录
- `is_using_udp: bool` - 是否使用UDP协议接收多播数据
- `is_multicast: bool` - 是否使用组播方式

**返回:** 新的MdApi实例

#### `register_spi<T>(handler: T) -> CtpResult<()>`

注册SPI回调处理器。

**参数:**
- `handler: T` - 实现了`MdSpiHandler`特质的回调处理器

#### `req_user_login(req: &ReqUserLoginField) -> CtpResult<i32>`

发送用户登录请求。

**参数:**
- `req: &ReqUserLoginField` - 登录请求结构体

**返回:** 请求ID

#### `subscribe_market_data(instrument_ids: &[&str]) -> CtpResult<()>`

订阅行情数据。

**参数:**
- `instrument_ids: &[&str]` - 合约代码数组

#### `unsubscribe_market_data(instrument_ids: &[&str]) -> CtpResult<()>`

退订行情数据。

**参数:**
- `instrument_ids: &[&str]` - 合约代码数组

### SPI回调接口

实现`MdSpiHandler`特质来处理行情事件：

```rust
use deepissue_ctp_rust::api::md_api::{MdSpiHandler, DepthMarketDataField};

struct MyMdHandler;

impl MdSpiHandler for MyMdHandler {
    fn on_front_connected(&mut self) {
        println!("行情前置已连接");
    }
    
    fn on_front_disconnected(&mut self, reason: i32) {
        println!("行情前置已断开，原因: {}", reason);
    }
    
    fn on_rsp_user_login(
        &mut self,
        user_login: Option<RspUserLoginField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        // 处理登录响应
    }
    
    fn on_rtn_depth_market_data(&mut self, market_data: DepthMarketDataField) {
        // 处理深度行情数据
        println!("合约: {}, 最新价: {}", 
                 extract_string(&market_data.instrument_id),
                 market_data.last_price);
    }
}
```

#### 主要回调方法

- `on_front_connected()` - 前置机连接成功
- `on_front_disconnected(reason: i32)` - 前置机连接断开
- `on_heart_beat_warning(time_lapse: i32)` - 心跳超时警告
- `on_rsp_user_login()` - 登录响应
- `on_rsp_user_logout()` - 登出响应
- `on_rtn_depth_market_data()` - 深度行情通知
- `on_rsp_sub_market_data()` - 订阅行情响应
- `on_rsp_unsub_market_data()` - 退订行情响应

## 交易API (TraderApi)

### 创建和初始化

```rust
use deepissue_ctp_rust::api::{TraderApi, CtpApi};

// 创建交易API实例
let mut trader_api = TraderApi::new(Some("./flow"))?;

// 注册前置机地址
trader_api.register_front("tcp://180.168.146.187:10130")?;

// 初始化API
trader_api.init()?;
```

### 主要方法

#### `new(flow_path: Option<&str>) -> CtpResult<TraderApi>`

创建新的交易API实例。

**参数:**
- `flow_path: Option<&str>` - 流文件存储目录

#### `req_authenticate(req: &ReqAuthenticateField) -> CtpResult<i32>`

发送客户端认证请求。

**参数:**
- `req: &ReqAuthenticateField` - 认证请求结构体

**返回:** 请求ID

#### `req_user_login(req: &ReqUserLoginField) -> CtpResult<i32>`

发送用户登录请求。

### SPI回调接口

实现`TraderSpiHandler`特质来处理交易事件：

```rust
use deepissue_ctp_rust::api::trader_api::{TraderSpiHandler, OrderField, TradeField};

struct MyTraderHandler;

impl TraderSpiHandler for MyTraderHandler {
    fn on_front_connected(&mut self) {
        println!("交易前置已连接");
    }
    
    fn on_rsp_authenticate(
        &mut self,
        rsp_authenticate: Option<RspAuthenticateField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        // 处理认证响应
    }
    
    fn on_rtn_order(&mut self, order: OrderField) {
        // 处理报单回报
        println!("报单状态: {}", order.order_status);
    }
    
    fn on_rtn_trade(&mut self, trade: TradeField) {
        // 处理成交回报
        println!("成交价格: {}, 成交数量: {}", trade.price, trade.volume);
    }
}
```

#### 主要回调方法

- `on_front_connected()` - 前置机连接成功
- `on_front_disconnected(reason: i32)` - 前置机连接断开
- `on_rsp_authenticate()` - 认证响应
- `on_rsp_user_login()` - 登录响应
- `on_rtn_order()` - 报单通知
- `on_rtn_trade()` - 成交通知
- `on_rsp_order_insert()` - 报单录入响应
- `on_rsp_order_action()` - 报单操作响应

## 数据类型

### 基础类型

```rust
// 固定长度字符串类型
pub type BrokerIdType = [u8; 11];      // 经纪公司代码
pub type UserIdType = [u8; 16];        // 用户代码
pub type InstrumentIdType = [u8; 31];  // 合约代码
pub type PasswordType = [u8; 41];      // 密码
```

### 请求结构体

#### `ReqUserLoginField` - 用户登录请求

```rust
pub struct ReqUserLoginField {
    pub trading_day: [u8; 9],                    // 交易日
    pub broker_id: BrokerIdType,                 // 经纪公司代码
    pub user_id: UserIdType,                     // 用户代码
    pub password: PasswordType,                  // 密码
    pub user_product_info: ProductInfoType,      // 用户端产品信息
    pub interface_product_info: ProductInfoType, // 接口端产品信息
    pub protocol_info: ProtocolInfoType,         // 协议信息
    pub mac_address: MacAddressType,             // Mac地址
    pub one_time_password: PasswordType,         // 动态密码
    pub client_ip_address: IpAddressType,        // 客户端IP地址
    pub client_ip_port: IpPortType,              // 客户端IP端口
    pub login_remark: [u8; 36],                  // 登录备注
}
```

**构造方法:**

```rust
// 基本构造
let req = ReqUserLoginField::new("9999", "investor1", "password")?;

// 链式构造
let req = ReqUserLoginField::new("9999", "investor1", "password")?
    .with_product_info("MyApp")?
    .with_auth_code("AUTH123")?
    .with_mac_address("00:11:22:33:44:55")?
    .with_client_ip("192.168.1.100", "8080")?;
```

### 响应结构体

#### `RspUserLoginField` - 用户登录响应

```rust
pub struct RspUserLoginField {
    pub trading_day: [u8; 9],        // 交易日
    pub login_time: [u8; 9],         // 登录成功时间
    pub broker_id: BrokerIdType,     // 经纪公司代码
    pub user_id: UserIdType,         // 用户代码
    pub system_name: [u8; 41],       // 交易系统名称
    pub front_id: i32,               // 前置编号
    pub session_id: i32,             // 会话编号
    pub max_order_ref: [u8; 13],     // 最大报单引用
    pub shfe_time: [u8; 9],          // 上期所时间
    pub dce_time: [u8; 9],           // 大商所时间
    pub czce_time: [u8; 9],          // 郑商所时间
    pub ffex_time: [u8; 9],          // 中金所时间
    pub ine_time: [u8; 9],           // 能源中心时间
}
```

#### `RspInfoField` - 响应信息

```rust
pub struct RspInfoField {
    pub error_id: i32,         // 错误代码
    pub error_msg: [u8; 81],   // 错误信息
}

impl RspInfoField {
    pub fn is_success(&self) -> bool;           // 是否成功
    pub fn get_error_msg(&self) -> CtpResult<String>; // 获取错误信息
}
```

### 行情数据结构

#### `DepthMarketDataField` - 深度行情数据

```rust
pub struct DepthMarketDataField {
    pub trading_day: [u8; 9],           // 交易日
    pub instrument_id: [u8; 31],        // 合约代码
    pub exchange_id: [u8; 9],           // 交易所代码
    pub last_price: f64,                // 最新价
    pub pre_settlement_price: f64,      // 上次结算价
    pub pre_close_price: f64,           // 昨收盘
    pub open_price: f64,                // 今开盘
    pub highest_price: f64,             // 最高价
    pub lowest_price: f64,              // 最低价
    pub volume: i32,                    // 数量
    pub turnover: f64,                  // 成交金额
    pub open_interest: f64,             // 持仓量
    pub upper_limit_price: f64,         // 涨停板价
    pub lower_limit_price: f64,         // 跌停板价
    pub update_time: [u8; 9],           // 最后修改时间
    pub update_millisec: i32,           // 最后修改毫秒
    pub bid_price1: f64,                // 申买价一
    pub bid_volume1: i32,               // 申买量一
    pub ask_price1: f64,                // 申卖价一
    pub ask_volume1: i32,               // 申卖量一
    // ... 五档行情数据
    pub average_price: f64,             // 平均价格
    pub action_day: [u8; 9],            // 业务日期
}
```

## 编码转换

### `GbkConverter` - 编码转换器

```rust
use deepissue_ctp_rust::encoding::GbkConverter;

// UTF-8 转 GB18030
let gb_bytes = GbkConverter::utf8_to_gb18030("期货合约")?;

// GB18030 转 UTF-8  
let utf8_string = GbkConverter::gb18030_to_utf8(&gb_bytes)?;

// C字符串转换
let c_string = GbkConverter::utf8_to_gb18030_cstring("测试")?;
let back = unsafe { GbkConverter::cstring_to_utf8(c_string.as_ptr()) }?;

// 固定长度数组转换
let fixed_bytes: [u8; 20] = GbkConverter::utf8_to_fixed_bytes("测试")?;
let back = GbkConverter::fixed_bytes_to_utf8(&fixed_bytes)?;
```

### `StringConvert` 特质

为固定长度类型提供字符串转换：

```rust
use deepissue_ctp_rust::types::{StringConvert, InstrumentIdType};

// 从字符串创建
let instrument_id = InstrumentIdType::from_utf8_string("rb2501")?;

// 转换回字符串
let string_back = instrument_id.to_utf8_string()?;
```

## 错误处理

### `CtpError` - 错误类型

```rust
pub enum CtpError {
    FfiError(String),              // FFI调用错误
    EncodingError(String),         // 编码转换错误
    ConnectionError(String),       // 网络连接错误
    AuthenticationError(String),   // 登录认证错误
    BusinessError(i32, String),    // 业务逻辑错误
    InitializationError(String),   // 初始化错误
    TimeoutError(String),          // 超时错误
    InvalidParameterError(String), // 无效参数错误
    MemoryError(String),           // 内存错误
    Other(String),                 // 其他错误
}
```

### 错误处理示例

```rust
use deepissue_ctp_rust::error::{CtpError, CtpResult};

fn handle_result(result: CtpResult<()>) {
    match result {
        Ok(()) => println!("操作成功"),
        Err(CtpError::ConnectionError(msg)) => {
            println!("连接错误: {}", msg);
        }
        Err(CtpError::BusinessError(code, msg)) => {
            println!("业务错误 [{}]: {}", code, msg);
        }
        Err(e) => println!("其他错误: {}", e),
    }
}
```

## 完整示例

### 行情订阅完整示例

```rust
use deepissue_ctp_rust::*;
use deepissue_ctp_rust::api::{MdApi, CtpApi};
use deepissue_ctp_rust::api::md_api::{MdSpiHandler, DepthMarketDataField};
use deepissue_ctp_rust::types::{ReqUserLoginField, RspUserLoginField, RspInfoField};
use deepissue_ctp_rust::encoding::GbkConverter;

struct MarketDataHandler {
    logged_in: bool,
}

impl MarketDataHandler {
    fn new() -> Self {
        Self { logged_in: false }
    }
}

impl MdSpiHandler for MarketDataHandler {
    fn on_front_connected(&mut self) {
        println!("✓ 行情前置连接成功");
    }
    
    fn on_front_disconnected(&mut self, reason: i32) {
        println!("✗ 行情前置连接断开: {}", reason);
        self.logged_in = false;
    }
    
    fn on_rsp_user_login(
        &mut self,
        _user_login: Option<RspUserLoginField>,
        rsp_info: Option<RspInfoField>,
        request_id: i32,
        _is_last: bool,
    ) {
        println!("收到登录响应 (ID: {})", request_id);
        
        if let Some(rsp) = rsp_info {
            if rsp.is_success() {
                println!("✓ 登录成功");
                self.logged_in = true;
            } else {
                if let Ok(error_msg) = rsp.get_error_msg() {
                    println!("✗ 登录失败: {}", error_msg);
                }
            }
        }
    }
    
    fn on_rtn_depth_market_data(&mut self, market_data: DepthMarketDataField) {
        if let Ok(instrument_id) = GbkConverter::fixed_bytes_to_utf8(&market_data.instrument_id) {
            let instrument_id = instrument_id.trim_end_matches('\0');
            if !instrument_id.is_empty() {
                println!("📈 {} - 最新价: {:.2}, 成交量: {}", 
                         instrument_id, 
                         market_data.last_price, 
                         market_data.volume);
            }
        }
    }
    
    fn on_rsp_sub_market_data(
        &mut self,
        specific_instrument: Option<String>,
        rsp_info: Option<RspInfoField>,
        _request_id: i32,
        _is_last: bool,
    ) {
        if let Some(rsp) = rsp_info {
            if rsp.is_success() {
                if let Some(instrument) = specific_instrument {
                    println!("✓ 订阅成功: {}", instrument);
                }
            } else {
                if let Ok(error_msg) = rsp.get_error_msg() {
                    println!("✗ 订阅失败: {}", error_msg);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> CtpResult<()> {
    println!("🚀 启动CTP行情客户端");
    
    // 创建行情API
    let mut md_api = MdApi::new(Some("./flow"), false, false)?;
    println!("✓ 创建行情API成功");
    
    // 注册回调处理器
    let handler = MarketDataHandler::new();
    md_api.register_spi(handler)?;
    println!("✓ 注册SPI处理器成功");
    
    // 注册前置机地址
    md_api.register_front("tcp://180.168.146.187:10131")?;
    println!("✓ 注册前置机地址成功");
    
    // 初始化API
    md_api.init()?;
    println!("✓ 初始化API成功");
    
    // 等待连接建立
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // 创建登录请求
    let login_req = ReqUserLoginField::new("9999", "your_investor_id", "your_password")?
        .with_product_info("TestApp")?;
    
    // 发送登录请求
    let login_id = md_api.req_user_login(&login_req)?;
    println!("✓ 发送登录请求 (ID: {})", login_id);
    
    // 等待登录完成
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    // 订阅行情
    let instruments = ["rb2501", "i2501", "TA501"];
    md_api.subscribe_market_data(&instruments)?;
    println!("✓ 订阅行情: {:?}", instruments);
    
    // 保持运行状态
    println!("📊 开始接收行情数据...");
    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    
    println!("👋 程序结束");
    Ok(())
}
```

## 注意事项

1. **内存安全**: 所有字符串转换都经过安全检查
2. **线程安全**: API实例可以在多线程环境中安全使用
3. **错误处理**: 始终检查函数返回值
4. **资源释放**: API实例会在Drop时自动释放资源
5. **编码转换**: 字符串会自动在UTF-8和GB18030之间转换

## 常见问题

### Q: 如何处理连接断开？

A: 在`on_front_disconnected`回调中处理重连逻辑：

```rust
fn on_front_disconnected(&mut self, reason: i32) {
    println!("连接断开: {}", reason);
    // 实现重连逻辑
}
```

### Q: 如何处理中文字符串？

A: 使用提供的编码转换工具：

```rust
use deepissue_ctp_rust::encoding::GbkConverter;

let utf8_str = "期货合约";
let gb_bytes = GbkConverter::utf8_to_gb18030(utf8_str)?;
```

### Q: 如何获取详细错误信息？

A: 检查`RspInfoField`的内容：

```rust
if let Some(rsp) = rsp_info {
    if !rsp.is_success() {
        let error_msg = rsp.get_error_msg()?;
        println!("错误: [{}] {}", rsp.error_id, error_msg);
    }
}
```