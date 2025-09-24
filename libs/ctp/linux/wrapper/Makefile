# CTP Wrapper Makefile
# 支持 macOS 和 Linux 平台

# Linux特定配置
OS = linux
LIB_DIR = ../lib
SHARED_EXT = so
LINKER_FLAGS = -L$(LIB_DIR) -lthostmduserapi_se -lthosttraderapi_se -lpthread -ldl

# 编译器和标志
CXX = g++
CXXFLAGS = -fPIC -std=c++11 -O2 -Wall
INCLUDES = -I../include -I../../common

# 源文件
SOURCES = spi_bridge.cpp ctp_wrapper.cpp ../../common/debug_logger.cpp
OBJECTS = $(SOURCES:.cpp=.o)

# 目标文件
SHARED_LIB = libctp_wrapper.$(SHARED_EXT)
STATIC_LIB = libctp_wrapper.a

.PHONY: all clean shared static test help

# 默认目标
all: shared

# 编译共享库
shared: $(SHARED_LIB)

$(SHARED_LIB): $(OBJECTS)
	@echo "🔗 链接共享库: $@"
	$(CXX) -shared $(OBJECTS) $(LINKER_FLAGS) -o $@
	@echo "✅ 共享库编译完成"

# 编译静态库
static: $(STATIC_LIB)

$(STATIC_LIB): $(OBJECTS)
	@echo "📦 创建静态库: $@"
	ar rcs $@ $(OBJECTS)
	@echo "✅ 静态库编译完成"

# 编译目标文件
%.o: %.cpp
	@echo "🔨 编译: $<"
	$(CXX) $(CXXFLAGS) $(INCLUDES) -c $< -o $@

# 清理
clean:
	@echo "🧹 清理编译产物..."
	rm -f *.o *.$(SHARED_EXT) *.a
	@echo "✅ 清理完成"

# 测试编译
test: $(SHARED_LIB)
	@echo "🧪 测试库文件..."
	file $(SHARED_LIB)
ifeq ($(OS),macos)
	otool -L $(SHARED_LIB)
else
	ldd $(SHARED_LIB)
endif

# 安装 (复制到系统库目录)
install: $(SHARED_LIB)
	@echo "📦 安装库文件..."
ifeq ($(OS),macos)
	sudo cp $(SHARED_LIB) /usr/local/lib/
	sudo install_name_tool -id /usr/local/lib/$(SHARED_LIB) /usr/local/lib/$(SHARED_LIB)
else
	sudo cp $(SHARED_LIB) /usr/local/lib/
	sudo ldconfig
endif
	@echo "✅ 安装完成"

# 帮助信息
help:
	@echo "CTP Wrapper 编译帮助"
	@echo "===================="
	@echo "make            - 编译共享库 (默认)"
	@echo "make shared     - 编译共享库"
	@echo "make static     - 编译静态库"
	@echo "make all        - 编译共享库"
	@echo "make clean      - 清理编译产物"
	@echo "make test       - 测试编译结果"
	@echo "make install    - 安装到系统目录"
	@echo "make help       - 显示此帮助"
	@echo ""
	@echo "当前平台: $(OS)"
	@echo "库目录: $(LIB_DIR)"
	@echo "共享库扩展名: $(SHARED_EXT)"
