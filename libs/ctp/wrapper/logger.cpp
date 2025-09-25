#include "logger.h"
#include <cstring>
#include <iomanip>

DebugLogger &DebugLogger::getInstance() {
  static DebugLogger instance;
  return instance;
}

DebugLogger::~DebugLogger() { cleanup(); }

void DebugLogger::init(bool enable_debug, const char *log_file) {
  std::lock_guard<std::mutex> lock(mutex_);

  enabled_ = enable_debug;
  if (!enabled_) {
    return;
  }

  // 如果之前打开了文件，先关闭
  if (file_stream_.is_open()) {
    file_stream_.close();
  }

  use_file_ = (log_file != nullptr && strlen(log_file) > 0);

  if (use_file_) {
    file_stream_.open(log_file, std::ios::out | std::ios::app);
    if (!file_stream_.is_open()) {
      std::cerr << "[CTP_DEBUG] 无法打开日志文件: " << log_file
                << ", 将使用控制台输出" << std::endl;
      use_file_ = false;
    }
  }
}

void DebugLogger::cleanup() {
  std::lock_guard<std::mutex> lock(mutex_);

  if (file_stream_.is_open()) {
    file_stream_.close();
  }

  enabled_ = false;
  use_file_ = false;
}

void DebugLogger::debug(const char *file, int line, const char *func,
                        const char *format, ...) {
  if (!enabled_) {
    return;
  }

  // 处理可变参数
  va_list args;
  va_start(args, format);

  // 先计算需要的缓冲区大小
  va_list args_copy;
  va_copy(args_copy, args);
  int len = vsnprintf(nullptr, 0, format, args_copy);
  va_end(args_copy);

  if (len < 0) {
    va_end(args);
    return;
  }

  // 分配缓冲区并格式化字符串
  std::string message(len + 1, '\0');
  vsnprintf(&message[0], len + 1, format, args);
  va_end(args);

  // 移除尾部的null字符
  message.resize(len);

  // 构建完整的日志消息
  std::ostringstream log_stream;
  log_stream << "[" << getCurrentTimestamp() << "] "
             << "[DEBUG] "
             // << "[Thread:" << getThreadId() << "] "
             << "[" << extractFileName(file) << ":" << line //<< ":" << func
             << "] " << message;

  std::string log_message = log_stream.str();

  // 线程安全地输出日志
  std::lock_guard<std::mutex> lock(mutex_);

  if (use_file_ && file_stream_.is_open()) {
    file_stream_ << log_message << std::endl;
    file_stream_.flush(); // 立即刷新以确保实时写入
    std::cout << log_message << std::endl;
    std::cout.flush();
  } else {
    std::cout << log_message << std::endl;
    std::cout.flush();
  }
}

std::string DebugLogger::getCurrentTimestamp() {
  auto now = std::chrono::system_clock::now();
  auto time_t = std::chrono::system_clock::to_time_t(now);
  auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(
                now.time_since_epoch()) %
            1000;

  std::ostringstream oss;
  oss << std::put_time(std::localtime(&time_t), "%Y-%m-%d %H:%M:%S");
  oss << "." << std::setfill('0') << std::setw(3) << ms.count();

  return oss.str();
}

std::string DebugLogger::getThreadId() {
#ifdef _WIN32
  return std::to_string(GetCurrentThreadId());
#elif defined(__APPLE__) || defined(__linux__)
  std::ostringstream oss;
  oss << std::this_thread::get_id();
  return oss.str();
#else
  return "unknown";
#endif
}

std::string DebugLogger::extractFileName(const char *full_path) {
  if (!full_path)
    return "unknown";

  // 跨平台路径分隔符处理
  const char *filename = full_path;
  const char *last_slash = nullptr;

  // 查找最后一个 '/' 或 '\'
  for (const char *p = full_path; *p; ++p) {
    if (*p == '/' || *p == '\\') {
      last_slash = p;
    }
  }

  if (last_slash) {
    filename = last_slash + 1;
  }

  return std::string(filename);
}

// C接口实现
extern "C" {
void CTP_InitializeDebugLogging(const CtpLogConfig *config) {
  if (!config) {
    return;
  }

  DebugLogger::getInstance().init(config->enable_debug != 0,
                                  config->log_file_path);
}

void CTP_CleanupDebugLogging() { DebugLogger::getInstance().cleanup(); }
}
