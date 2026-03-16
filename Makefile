.PHONY: all dev build build-windows install clean help

# Windows 编译目标架构
WIN_TARGET = x86_64-pc-windows-gnu

# 默认目标：显示帮助
help:
	@echo "可用命令 (Available commands):"
	@echo "  make dev            - 启动 Tauri 开发服务器 (Start Tauri dev server)"
	@echo "  make web-dev        - 启动 Web 前端开发服务器 (Start Web frontend dev server)"
	@echo "  make build          - 构建 Linux 原生版本 (Build for Linux)"
	@echo "  make build-windows  - 构建 Windows .exe 版本 (Build for Windows)"
	@echo "  make install        - 安装所有 NPM 依赖 (Install all dependencies)"
	@echo "  make clean          - 清理构建产物 (Clean build artifacts)"

# 编译 Web Client 静态页面 (Build Web Client static pages)
build-web:
	cd web-client && npm run build

# 启动开发环境 (Tauri)
dev: build-web
	npm run tauri dev

# 启动 Web 前端开发环境
web-dev:
	cd web-client && npm run dev

# 构建 Linux 版本
build: build-web
	npm run tauri build

# 构建 Windows 版本 (.exe)
build-windows: build-web
	npm run tauri build -- --target $(WIN_TARGET)

# 安装依赖
install:
	npm install
	cd web-client && npm install

# 清理构建文件
clean:
	rm -rf src-tauri/target
	rm -rf dist
