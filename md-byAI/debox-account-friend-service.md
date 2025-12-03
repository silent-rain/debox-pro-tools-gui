# DeBox 账号好友服务层

## 功能说明

基于 DeBox 账号关注人的实现，完整实现了 DeBox 账号好友的基础增删改查功能。包括好友列表查询、好友信息管理、状态更新、批量操作以及与 DeBox API 的集成。

## 接口路由

### 基础 CRUD 接口

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/debox/debox-account-friends/` | 添加好友 |
| POST | `/debox/debox-account-friends/list` | 获取好友列表 |
| GET | `/debox/debox-account-friends/{id}` | 获取好友详情 |
| DELETE | `/debox/debox-account-friends/{id}` | 删除好友 |
| PUT | `/debox/debox-account-friends/update` | 更新好友信息 |
| PUT | `/debox/debox-account-friends/update-status` | 更新好友状态 |

### 扩展接口

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/debox/debox-account-friends/sync-friends` | 同步好友列表 |
| POST | `/debox/debox-account-friends/batch-friends` | 批量添加好友 |
| POST | `/debox/debox-account-friends/debox-user-search` | 用户搜索 |

## 文件结构

### 层次结构
```
apps/crates/service_hub/debox/src/
├── enums/
│   └── debox_account_friend.rs          # 枚举定义
├── dto/
│   └── debox_account_friend.rs          # 数据传输对象
├── dao/
│   └── debox_account_friend.rs          # 数据访问层
├── service/
│   └── debox_account_friend.rs          # 业务逻辑层
├── controller/
│   └── debox_account_friend.rs          # 控制器层
└── router/
    └── debox_account_friend.rs          # 路由层
```

## 核心功能

### 1. 数据传输对象 (DTO)

#### 请求参数
- `GetDeboxAccountFriendsReq`: 查询好友列表
- `CreateDeboxAccountFriendReq`: 创建好友
- `UpdateDeboxAccountFriendReq`: 更新好友信息
- `UpdateDeboxAccountFriendStatusReq`: 更新好友状态
- `DeleteDeboxAccountFriendReq`: 删除好友
- `SyncDeboxAccountFriendsReq`: 同步好友列表
- `BatchAccountFriendsReq`: 批量添加好友
- `DeboxUserSearchReq`: 用户搜索

#### 响应参数
- `GetDeboxAccountFriendsResp`: 好友列表响应
- `GetDeboxAccountFriendResp`: 好友详情响应
- 其他操作响应结构体

### 2. 数据访问层 (DAO)

#### 基础 CRUD
```rust
// 列表查询
pub async fn list(&self, user_id: i32, req: GetDeboxAccountFriendsReq) -> Result<(Vec<Model>, u64), DbErr>

// 详情查询
pub async fn info(&self, id: i32, user_id: i32) -> Result<Option<Model>, DbErr>

// 创建
pub async fn create(&self, active_model: ActiveModel) -> Result<Model, DbErr>

// 更新
pub async fn update(&self, id: i32, user_id: i32, active_model: ActiveModel) -> Result<u64, DbErr>

// 状态更新
pub async fn update_status(&self, id: i32, user_id: i32, status: bool) -> Result<u64, DbErr>

// 删除
pub async fn delete(&self, id: i32, user_id: i32) -> Result<u64, DbErr>
```

#### 扩展功能
```rust
// 根据 DeBox 用户ID查询
pub async fn friend_by_debox_user_id(&self, user_id: i32, account_id: i32, debox_user_id: String) -> Result<Option<Model>, DbErr>

// 获取指定账号的好友列表
pub async fn friends_by_account_id(&self, user_id: i32, account_id: i32) -> Result<Vec<Model>, DbErr>

// 清空指定账号的好友列表
pub async fn delete_by_account_id(&self, user_id: i32, account_id: i32) -> Result<u64, DbErr>

// 批量插入
pub async fn creates(&self, active_models: Vec<ActiveModel>) -> Result<(), DbErr>
```

### 3. 业务逻辑层 (Service)

#### 基础业务逻辑
- **列表查询**: 支持分页、排序、时间范围过滤、账号过滤
- **详情查询**: 根据ID和用户ID查询好友信息
- **创建好友**: 检查重复性，创建好友记录
- **更新好友**: 更新好友基本信息
- **状态管理**: 启用/禁用好友状态
- **删除好友**: 软删除好友记录

#### DeBox API 集成
```rust
// 获取 DeBox 客户端
fn debox_client(&self, model: &debox_account::Model) -> Result<DeBoxClient, ErrorMsg>

// 获取关系列表（关注/粉丝/好友）
async fn relation_list(&self, client: &DeBoxClient, page: u64, status: RelationStatus, look_user_id: Option<u64>) -> Result<Vec<Relation>, ErrorMsg>

// 关注账号
async fn debox_follow_account(&self, client: &DeBoxClient, follow_id: &str) -> Result<(), ErrorMsg>
```

#### 高级功能
- **同步好友列表**: 从 DeBox API 同步好友数据到本地数据库
- **批量添加好友**: 支持从账号、群组、用户列表三种方式批量添加好友
- **用户搜索**: 集成 DeBox 用户搜索功能

### 4. 控制器层 (Controller)

实现了所有接口的 HTTP 处理逻辑，包括：
- 参数验证和转换
- 调用服务层方法
- 统一响应格式
- 错误处理

### 5. 路由层 (Router)

注册所有相关的 API 路由，支持 RESTful 风格的接口设计。

## 使用示例

### 1. 添加好友
```bash
POST /debox/debox-account-friends/
Content-Type: application/json

{
  "account_id": 1,
  "debox_user_id": "debox_user_123",
  "name": "张三",
  "avatar": "https://example.com/avatar.jpg",
  "desc": "我的好朋友",
  "status": true
}
```

### 2. 查询好友列表
```bash
POST /debox/debox-account-friends/list
Content-Type: application/json

{
  "page": 1,
  "page_size": 20,
  "account_ids": [1, 2],
  "status": true,
  "sorts": ["created_at:desc"]
}
```

### 3. 更新好友状态
```bash
PUT /debox/debox-account-friends/update-status
Content-Type: application/json

{
  "id": 1,
  "status": false
}
```

### 4. 同步好友列表
```bash
POST /debox/debox-account-friends/sync-friends
Content-Type: application/json

{
  "account_ids": [1, 2, 3]
}
```

### 5. 批量添加好友
```bash
POST /debox/debox-account-friends/batch-friends
Content-Type: application/json

{
  "account_id": 1,
  "target_account_id": 2,
  "friend_type": 0,
  "debox_user_ids": ["user1", "user2", "user3"]
}
```

## 枚举定义

### FriendType
```rust
pub enum FriendType {
    Account,  // 从账号的好友列表添加
    Group,    // 从群组成员添加
    User,     // 从指定用户ID列表添加
}
```

## 错误处理

所有操作都包含完整的错误处理机制：
- 数据库操作错误
- DeBox API 调用错误
- 参数验证错误
- 业务逻辑错误

## 性能优化

1. **批量操作**: 支持批量插入和批量添加好友
2. **分页查询**: 列表查询支持分页，避免一次性加载大量数据
3. **索引优化**: 在数据库层面建立了合适的索引
4. **缓存机制**: DeBox 客户端复用，减少重复创建开销

## 安全考虑

1. **用户隔离**: 所有操作都基于 user_id 进行隔离
2. **参数验证**: 使用 validator 进行严格的参数验证
3. **权限控制**: 确保用户只能操作自己的好友数据
4. **SQL 注入防护**: 使用 SeaORM 的参数化查询

## 模块更新

本次更新同时修改了以下模块文件：

1. **enums/mod.rs**: 添加了 `pub mod debox_account_friend;`
2. **dto/mod.rs**: 添加了 `pub mod debox_account_friend;`
3. **dao/mod.rs**: 添加了 `pub mod debox_account_friend;`
4. **service/mod.rs**: 添加了 `pub mod debox_account_friend;`
5. **controller/mod.rs**: 添加了 `pub mod debox_account_friend;`
6. **router/mod.rs**: 添加了 `pub mod debox_account_friend;` 并在路由注册中合并
7. **lib.rs**: 添加了所有相关模块的导出

## 兼容性说明

- 与现有的 DeBox 账号关注人功能完全兼容
- 遵循相同的代码风格和架构模式
- 支持相同的数据库连接池和事务管理
- 使用相同的错误处理和日志记录机制

## 注意事项

1. **数据一致性**: 批量操作时注意事务处理
2. **API 限制**: DeBox API 可能有调用频率限制
3. **存储空间**: 好友数据可能占用较多存储空间
4. **同步延迟**: 与 DeBox API 的同步可能存在延迟