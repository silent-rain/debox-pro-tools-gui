# DeBox Account Friend Sync Friends 迁移文档

## 功能说明

将 `debox_account_follow` 的 `sync_follows` 接口完整迁移到 `debox_account_friend` 中，实现同步好友列表的功能。

## 修改内容

### 1. DTO 层修改
**文件**: `apps/crates/service_hub/debox/src/dto/debox_account_friend.rs`

**新增结构体**:
```rust
/// 同步好友列表 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct SyncDeboxAccountFriendsReq {
    /// 账号ID
    pub account_ids: Vec<i32>,
}

/// 同步好友列表 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncDeboxAccountFriendsResp {}
```

### 2. Service 层修改
**文件**: `apps/crates/service_hub/debox/src/service/debox_account_friend.rs`

**新增依赖**:
- 添加 `tokio::time::{sleep, Duration}` 导入
- 添加 `SyncDeboxAccountFriendsReq` 导入
- 添加 `debox_account_dao` 依赖注入

**新增方法**:
- `sync_friends()` - 同步好友列表主方法
- `get_account()` - 获取账号信息
- `get_all_friends()` - 获取所有好友
- `batch_create()` - 批量添加好友

**核心逻辑**:
1. 遍历传入的账号ID列表
2. 为每个账号获取DeBox客户端
3. 使用 `RelationStatus::Friend` 获取好友列表
4. 分页处理，每页20条记录
5. 先删除原好友列表，再批量插入新数据
6. 每次请求间隔500ms，避免频繁调用

### 3. Controller 层修改
**文件**: `apps/crates/service_hub/debox/src/controller/debox_account_friend.rs`

**新增导入**:
- `SyncDeboxAccountFriendsReq`
- `SyncDeboxAccountFriendsResp`

**新增方法**:
```rust
/// 同步好友列表
pub async fn sync_friends(
    ctx: Context,
    Extension(provider): Extension<AInjectProvider>,
    Json(req): Json<SyncDeboxAccountFriendsReq>,
) -> Responder<SyncDeboxAccountFriendsResp>
```

### 4. Router 层修改
**文件**: `apps/crates/service_hub/debox/src/router/debox_account_friend.rs`

**新增路由**:
```rust
.route(
    "/sync-friends",
    post(DeboxAccountFriendController::sync_friends),
)
```

## 接口说明

### 请求信息
- **路径**: `POST /debox/debox-account-friends/sync-friends`
- **请求体**:
```json
{
    "account_ids": [1, 2, 3]
}
```
- **响应体**:
```json
{
    "code": 200,
    "message": "success",
    "data": {}
}
```

## 与 Follow 接口的差异

| 项目 | Follow | Friend |
|------|--------|--------|
| 路由路径 | `/sync-follows` | `/sync-friends` |
| 关系状态 | `RelationStatus::Follow` | `RelationStatus::Friend` |
| 数据表 | `t_debox_account_follow` | `t_debox_account_friend` |
| 错误信息 | "关注人" | "好友" |

## 使用示例

```typescript
// 前端调用示例
import { deboxAccountFriendApi } from '@/api/debox-account-friend';

try {
    await deboxAccountFriendApi.syncFriends({
        account_ids: [1, 2, 3]
    });
    console.log('同步好友成功');
} catch (error) {
    console.error('同步好友失败:', error);
}
```

## 注意事项

1. **权限控制**: 需要用户登录验证
2. **频率限制**: 每次API调用间隔500ms
3. **数据一致性**: 采用先删除后插入的策略
4. **错误处理**: 单个账号同步失败不会影响其他账号
5. **分页处理**: 自动处理分页，每页20条记录
6. **资源清理**: 同步完成后会清理原有的好友数据

## 兼容性说明

- 完全兼容现有的 `debox_account_friend` 接口设计
- 遵循项目统一的错误处理和响应格式
- 使用相同的依赖注入和服务架构模式