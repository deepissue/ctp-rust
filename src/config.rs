use dotenvy::dotenv;
use std::env;

/// CTP配置信息 (别名，用于异步API)
#[derive(Debug, Clone)]
pub struct CtpConfig {
    pub md_front_address: String,
    pub trader_front_address: String,
    pub broker_id: String,
    pub investor_id: String,
    pub password: String,
    pub flow_path: String,
    pub instruments: Vec<String>,
    pub app_id: String,
    pub auth_code: String,
    pub product_info: String,
}

impl CtpConfig {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let md_front_address = env::var("MD_FRONT_ADDRESS")
            .unwrap_or_else(|_| "tcp://121.37.80.177:20004".to_string());
        let trader_front_address = env::var("TRADER_FRONT_ADDRESS")
            .unwrap_or_else(|_| "tcp://121.37.80.177:20002".to_string());
        let broker_id = env::var("BROKER_ID").unwrap_or_else(|_| "9999".to_string());
        let investor_id =
            env::var("INVESTOR_ID").map_err(|_| "请在.env文件中设置INVESTOR_ID环境变量")?;
        let password = env::var("PASSWORD").map_err(|_| "请在.env文件中设置PASSWORD环境变量")?;
        let flow_path = env::var("FLOW_PATH").unwrap_or_else(|_| "./flow".to_string());
        let instruments_str = env::var("INSTRUMENTS").unwrap_or_else(|_| "rb2601".to_string());
        let instruments: Vec<String> = instruments_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let app_id = env::var("APP_ID").unwrap_or_else(|_| "".to_string());
        let auth_code = env::var("AUTH_CODE").unwrap_or_else(|_| "".to_string());
        let product_info = env::var("PRODUCT_INFO").unwrap_or_else(|_| "".to_string());

        Ok(CtpConfig {
            md_front_address,
            trader_front_address,
            broker_id,
            investor_id,
            password,
            flow_path,
            instruments,
            app_id,
            auth_code,
            product_info,
        })
    }
}
