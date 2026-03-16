# 微观需求: 管理端重新加载配置 (IDEA-007)

## 元数据
- **ID**: REQ-MICRO-007 (覆盖 IDEA-007)
- **模块**: UI1 (AdminApp) / S3 (Config) / S1 (ServerMgr)
- **优先级**: Medium
- **关联**: `docs/design/api_specs/admin_commands.md`

## 1. 概述
管理员修改 `settings.toml` 后，希望无需退出应用即可让新配置生效。
本需求提供一个“重新加载配置”入口：后端重新读取配置文件并更新内存中的 Settings，前端在界面提示变更项，并在需要时引导管理员重启 HTTP 服务以应用端口等变更。

## 2. 范围

### 2.1 In Scope
- 重新读取 `settings.toml` 并更新内存 Settings。
- 在管理端显示“配置已更新”提示。
- 若 `server_port` 发生变化：
  - 服务未运行：下次启动服务使用新端口。
  - 服务运行中：提示“需重启服务后生效”，并提供“一键重启服务”。
- 若 Wi-Fi 配置发生变化（`wifi_ssid` / `wifi_password` / `wifi_encryption`）：同样提示是否需要重启服务（取决于实现是否支持运行时读取；本期按“重启服务后生效”处理）。

### 2.2 Out of Scope
- `db_path` 运行时切换数据库连接（本期不支持）。若检测到变更，只提示“需要重启应用后生效”。
- 自动编辑/生成 `settings.toml`（本期只读）。

## 3. 后端设计 (Tauri Command)

### 3.1 新增 Command: `reload_settings`

```rust
#[derive(serde::Serialize)]
pub struct ReloadSettingsResponse {
    pub settings: crate::config::Settings,
    pub changed: ReloadSettingsChanged,
}

#[derive(serde::Serialize)]
pub struct ReloadSettingsChanged {
    pub server_port: bool,
    pub db_path: bool,
    pub wifi: bool,
}

#[tauri::command]
pub async fn reload_settings(state: tauri::State<'_, AppState>) -> Result<ReloadSettingsResponse, String>
```

### 3.2 行为
1. 读取并解析 `settings.toml`（复用 `Settings::load_or_create()` 或新增仅 load 的方法）。
2. 与当前内存中的 Settings 做字段级 diff，生成 `changed`。
3. 用新值覆盖内存 Settings（注意多线程安全：AppState.settings 需要可变/可锁）。
4. 返回 `ReloadSettingsResponse`。

### 3.3 错误与提示文案
- 文件读取失败/解析失败：返回 `Err("重新加载配置失败: <原因>")`（原因可保留原始错误信息）。

## 4. 前端设计 (Admin UI)

### 4.1 入口位置
在 `src/components/ServerControl.vue` 的操作区新增按钮：`重新加载配置`。

### 4.2 交互逻辑
1. 点击后调用 `adminApi.reloadSettings()`。
2. 成功后 toast/alert 提示：
   - 无关键变更：`"配置已重新加载"`。
   - `server_port` 或 `wifi` 变更：
     - 若服务未运行：`"配置已更新，下次启动服务生效"`。
     - 若服务运行中：`"配置已更新，需重启服务后生效"`，并展示 `重启服务` 按钮（执行 `stop_server` -> `start_server`，并刷新访问链接）。
   - `db_path` 变更：`"数据库路径已变更，需要重启应用后生效"`。
3. 失败提示：`"重新加载失败: ..."`。

### 4.3 API 封装
在 `src/api/admin.ts` 新增：

```ts
reloadSettings(): Promise<ReloadSettingsResponse>
```

## 5. 验收标准

### AC-1 基本重载
Given 管理员修改了 `settings.toml`
When 点击“重新加载配置”
Then 弹出提示“配置已重新加载”且不需要退出应用

### AC-2 端口变更生效策略
Given 服务未运行且 `server_port` 被修改
When 点击“重新加载配置”后再点击“启动服务”
Then HTTP 服务使用新端口监听

Given 服务运行中且 `server_port` 被修改
When 点击“重新加载配置”
Then 提示“需重启服务后生效”并提供“重启服务”入口

### AC-3 db_path 变更提示
Given `db_path` 被修改
When 点击“重新加载配置”
Then 提示“需要重启应用后生效”，且不会在本次会话中切换数据库

## 6. 风险与备注
- 运行时更新 Settings 需要考虑并发读写；建议在 AppState 中对 Settings 加锁。
- 若未来希望 Wi-Fi 配置无需重启服务即可生效，需要将 Web 端读取 Settings 的方式改为共享状态（本期不强制）。
