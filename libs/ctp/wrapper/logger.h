#ifndef DEBUG_LOGGER_H
#define DEBUG_LOGGER_H

#include <iostream>
#include <fstream>
#include <mutex>
#include <thread>
#include <chrono>
#include <cstdarg>
#include <cstdio>
#include <string>
#include <sstream>

#ifdef _WIN32
    #include <windows.h>
#elif defined(__APPLE__) || defined(__linux__)
    #include <pthread.h>
    #include <unistd.h>
#endif

class DebugLogger {
public:
    static DebugLogger& getInstance();
    void init(bool enable_debug = false, const char* log_file = nullptr);
    void debug(const char* file, int line, const char* func, const char* format, ...);
    bool isEnabled() const { return enabled_; }
    void cleanup();

private:
    DebugLogger() = default;
    ~DebugLogger();
    
    std::mutex mutex_;
    bool enabled_ = false;
    std::ofstream file_stream_;
    bool use_file_ = false;
    
    std::string getCurrentTimestamp();
    std::string getThreadId();
    std::string extractFileName(const char* full_path);
};

// 便捷宏定义
#define CTP_DEBUG_INIT(enable, file) DebugLogger::getInstance().init(enable, file)
#define CTP_DEBUG_CLEANUP() DebugLogger::getInstance().cleanup()
#define CTP_DEBUG(format, ...) do { \
    if (DebugLogger::getInstance().isEnabled()) { \
        DebugLogger::getInstance().debug(__FILE__, __LINE__, __FUNCTION__, format, ##__VA_ARGS__); \
    } \
} while(0)

// C接口配置结构
typedef struct {
    int enable_debug;           // 0=关闭, 1=开启
    const char* log_file_path;  // 日志文件路径，NULL=控制台输出
    int max_file_size_mb;       // 最大文件大小（MB）- 预留功能
    int max_backup_files;       // 最大备份文件数 - 预留功能
} CtpLogConfig;

#ifdef __cplusplus
extern "C" {
#endif

// C接口函数
void CTP_InitializeDebugLogging(const CtpLogConfig* config);
void CTP_CleanupDebugLogging();

#ifdef __cplusplus
}
#endif

#endif // DEBUG_LOGGER_H