use ctp_rust::{CtpConfig, CtpResult};

fn main() -> CtpResult<()> {
    println!("Testing CTP Rust SDK integration...");
    
    // Test basic imports
    let version = ctp_rust::VERSION;
    println!("CTP Rust SDK version: {}", version);
    
    // Test configuration loading (this should not fail even without env vars)
    match CtpConfig::from_env() {
        Ok(config) => {
            println!("Configuration loaded successfully:");
            println!("  Broker ID: {}", config.broker_id);
            println!("  Flow Path: {}", config.flow_path);
        }
        Err(e) => {
            println!("Configuration loading failed (expected without env vars): {}", e);
        }
    }
    
    println!("Integration test completed successfully!");
    Ok(())
}
