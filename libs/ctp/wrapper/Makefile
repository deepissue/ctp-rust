# CTP Wrapper 统一 Makefile
# 自动检测平台并使用相应的配置

# 自动检测操作系统
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    DETECTED_OS = linux
endif
ifeq ($(UNAME_S),Darwin)
    DETECTED_OS = macos
endif

# 允许用户覆盖检测的系统类型
OS ?= $(DETECTED_OS)

# 平台特定配置
ifeq ($(OS),linux)
    # Linux 配置
    PLATFORM_DEFINE = -DCTP_PLATFORM_LINUX
    LIB_DIR = ../linux/lib
    SHARED_EXT = so
    CXX = g++
    PLATFORM_CXXFLAGS =
    LINKER_FLAGS = -L$(LIB_DIR) -lthostmduserapi_se -lthosttraderapi_se -lpthread -ldl
    INCLUDES = -I../linux/include
else ifeq ($(OS),macos)
    # macOS 配置
    PLATFORM_DEFINE = -DCTP_PLATFORM_MACOS
    LIB_DIR = ../mac64/lib
    SHARED_EXT = dylib
    CXX = clang++
    PLATFORM_CXXFLAGS = -mmacosx-version-min=14.0
    LINKER_FLAGS = -L$(LIB_DIR) -lthostmduserapi_se -lthosttraderapi_se
    INCLUDES = -I../mac64/include
else
    $(error 不支持的操作系统: $(OS). 支持的系统: linux, macos)
endif

# 通用编译器标志
CXXFLAGS = -fPIC -std=c++11 -O2 -Wall $(PLATFORM_DEFINE) $(PLATFORM_CXXFLAGS)

# 源文件和目标文件
SOURCES = spi_bridge.cpp ctp_wrapper.cpp logger.cpp
OBJECTS = $(SOURCES:.cpp=.o)
SHARED_LIB = libctp_wrapper.$(SHARED_EXT)
STATIC_LIB = libctp_wrapper.a

# 伪目标
.PHONY: all clean shared static test help info

# 默认目标
all: info shared

# 显示构建信息
info:
	@echo "🔧 CTP Wrapper 构建信息"
	@echo "======================="
	@echo "检测到的系统: $(DETECTED_OS)"
	@echo "目标系统: $(OS)"
	@echo "编译器: $(CXX)"
	@echo "库目录: $(LIB_DIR)"
	@echo "共享库扩展名: $(SHARED_EXT)"
	@echo "平台定义: $(PLATFORM_DEFINE)"
	@echo ""

# 编译共享库
shared: clean $(SHARED_LIB)
	rm -f *.o

$(SHARED_LIB): $(OBJECTS)
	@echo "🔗 链接共享库 [$(OS)]: $@"
	$(CXX) -shared $(OBJECTS) $(LINKER_FLAGS) -o $@
	@echo "✅ 共享库编译完成: $@"

# 编译静态库
static: $(STATIC_LIB)

$(STATIC_LIB): $(OBJECTS)
	@echo "📦 创建静态库 [$(OS)]: $@"
	ar rcs $@ $(OBJECTS)
	@echo "✅ 静态库编译完成: $@"

# 编译目标文件
%.o: %.cpp
	@echo "🔨 编译 [$(OS)]: $<"
	$(CXX) $(CXXFLAGS) $(INCLUDES) -c $< -o $@

# 清理
clean:
	@echo "🧹 清理编译产物 [$(OS)]..."
	rm -f *.o *.$(SHARED_EXT) *.a
	rm -f ../common/*.o
	@echo "✅ 清理完成"

# 测试编译结果
test: $(SHARED_LIB)
	@echo "🧪 测试库文件 [$(OS)]: $(SHARED_LIB)"
	file $(SHARED_LIB)
ifeq ($(OS),macos)
	@echo "📋 依赖库信息:"
	otool -L $(SHARED_LIB)
else
	@echo "📋 依赖库信息:"
	ldd $(SHARED_LIB) || true
endif
	@echo "✅ 测试完成"

# 安装到系统目录
install: $(SHARED_LIB)
	@echo "📦 安装库文件 [$(OS)]..."
ifeq ($(OS),macos)
	sudo cp $(SHARED_LIB) /usr/local/lib/
	sudo install_name_tool -id /usr/local/lib/$(SHARED_LIB) /usr/local/lib/$(SHARED_LIB)
else
	sudo cp $(SHARED_LIB) /usr/local/lib/
	sudo ldconfig
endif
	@echo "✅ 安装完成"

# 交叉编译支持
linux:
	$(MAKE) OS=linux

macos:
	$(MAKE) OS=macos

# 帮助信息
help:
	@echo "CTP Wrapper 统一构建系统"
	@echo "========================"
	@echo ""
	@echo "基本用法:"
	@echo "  make              - 显示信息并编译共享库 (自动检测平台)"
	@echo "  make shared       - 编译共享库"
	@echo "  make static       - 编译静态库"
	@echo "  make test         - 测试编译结果"
	@echo "  make clean        - 清理编译产物"
	@echo "  make install      - 安装到系统目录"
	@echo ""
	@echo "平台特定构建:"
	@echo "  make linux        - 强制使用 Linux 配置编译"
	@echo "  make macos        - 强制使用 macOS 配置编译"
	@echo "  make OS=linux     - 指定目标系统为 Linux"
	@echo "  make OS=macos     - 指定目标系统为 macOS"
	@echo ""
	@echo "实用工具:"
	@echo "  make info         - 显示构建配置信息"
	@echo "  make help         - 显示此帮助信息"
	@echo ""
	@echo "当前检测系统: $(DETECTED_OS)"
	@echo "支持的系统: linux, macos"
