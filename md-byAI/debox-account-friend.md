# DeBox 账号好友表

## 功能说明

DeBox 账号好友表用于存储用户的好友关系信息，包括好友的基本信息、状态和时间戳等。该表与用户表和 DeBox 账号表建立外键关联，确保数据的一致性。

## 表结构

### 字段说明

| 字段名 | 类型 | 说明 | 约束 |
|--------|------|------|------|
| id | i32 | 好友ID | 主键，自增 |
| user_id | i32 | 用户ID | 外键，关联用户表 |
| account_id | i32 | 账号ID | 外键，关联DeBox账号表 |
| debox_user_id | String | DeBox 用户ID | 长度30，默认空字符串 |
| name | String | 用户名称 | 长度50，非空 |
| avatar | Option<String> | 账号头像 | 长度250，可选 |
| desc | Option<String> | 描述信息 | 长度200，可选 |
| status | bool | 状态 | false:停用，true:正常，默认false |
| created_at | DateTime | 创建时间 | 非空，默认当前时间 |
| updated_at | DateTime | 更新时间 | 非空，自动更新 |

### 索引

- 复合索引：(user_id, account_id) - 用于快速查询用户的好友关系

## 文件结构

### Migration 文件
- 路径：`apps/crates/core/migration/src/debox/debox_account_friend.rs`
- 表名：`t_debox_account_friend`
- 包含创建表、索引和外键约束的迁移逻辑

### Entity 文件
- 路径：`apps/crates/core/entity/src/debox/debox_account_friend.rs`
- 实体名：`DeboxAccountFriend`
- 包含完整的实体定义和关联关系

## 关联关系

### 与 UserBase 的关联
```rust
impl Related<user_base::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserBase.def()
    }
}
```

### 与 DeboxAccount 的关联
```rust
impl Related<debox_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DeboxAccount.def()
    }
}
```

## 使用示例

### 1. 查询用户的所有好友
```rust
use sea_orm::*;

// 查询用户ID为1的所有好友
let friends = DeboxAccountFriend::find()
    .filter(Condition::all().add(debox_account_friend::Column::UserId.eq(1)))
    .filter(debox_account_friend::Column::Status.eq(true))
    .all(db)
    .await?;
```

### 2. 创建好友关系
```rust
use sea_orm::*;
use chrono::Local;

let new_friend = debox_account_friend::ActiveModel {
    user_id: Set(1),
    account_id: Set(2),
    debox_user_id: Set("debox_user_123".to_string()),
    name: Set("张三".to_string()),
    avatar: Set(Some("https://example.com/avatar.jpg".to_string())),
    desc: Set(Some("这是我的好朋友".to_string())),
    status: Set(true),
    created_at: Set(Local::now().naive_local()),
    updated_at: Set(Local::now().naive_local()),
    ..Default::default()
};

let result = new_friend.insert(db).await?;
```

### 3. 更新好友信息
```rust
use sea_orm::*;

let friend = DeboxAccountFriend::find_by_id(1).one(db).await?;
if let Some(friend) = friend {
    let mut active_friend: debox_account_friend::ActiveModel = friend.into();
    active_friend.name = Set("李四".to_string());
    active_friend.status = Set(true);
    
    let updated_friend = active_friend.update(db).await?;
}
```

### 4. 删除好友关系
```rust
use sea_orm::*;

let result = DeboxAccountFriend::delete_by_id(1).exec(db).await?;
```

## 兼容性说明

### 数据库兼容性
- **MySQL**: 完全支持，包括 `ON UPDATE CURRENT_TIMESTAMP` 功能
- **PostgreSQL**: 完全支持
- **SQLite**: 支持，但 `updated_at` 字段仅使用 `DEFAULT CURRENT_TIMESTAMP`

### 外键约束
- `user_id` 关联 `user_base.id`，级联更新和删除
- `account_id` 关联 `debox_account.id`，级联更新和删除

### 注意事项

1. **唯一性约束**: 建议在应用层确保 `(user_id, account_id)` 的唯一性，避免重复的好友关系
2. **状态管理**: 使用 `status` 字段进行软删除，而不是物理删除
3. **时间戳**: `updated_at` 字段会在更新时自动设置为当前时间
4. **外键级联**: 删除用户或账号时会自动删除相关的好友关系，请谨慎操作

## 模块更新

本次更新同时修改了以下模块文件：

1. **Migration 模块**: `apps/crates/core/migration/src/debox/mod.rs`
   - 添加了 `pub mod debox_account_friend;`

2. **Entity 模块**: `apps/crates/core/entity/src/debox/mod.rs`
   - 添加了 `pub mod debox_account_friend;`
   - 添加了 `pub use debox_account_friend::Entity as DeboxAccountFriend;`

确保在编译前运行数据库迁移以创建相应的表结构。