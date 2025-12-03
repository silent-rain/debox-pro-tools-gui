# Account Friend Management 前端页面更新文档

## 功能说明

将 `mobile-web/src/pages/account-friend-management/index.tsx` 页面从关注人管理完全修改为好友管理功能。

## 修改内容

### 1. 主页面文件修改
**文件**: `mobile-web/src/pages/account-friend-management/index.tsx`

**主要变更**:
- ✅ 导入 `AccountFriendList` 替代 `AccountFollowList`
- ✅ 更新状态变量名称：`followsUpdateState` → `friendsUpdateState`
- ✅ 更新函数名称和注释：
  - `handleSyncFollows` → `handleSyncFriends`
  - `handleRefreshFollows` → `handleRefreshFriends`
  - "同步关注人" → "同步好友"
- ✅ 更新组件属性：
  - `followsUpdateState` → `friendsUpdateState`
- ✅ 更新页面类名：`account-follow-management` → `account-friend-management`

### 2. 新增好友列表组件
**文件**: `mobile-web/src/pages/account-friend-management/components/AccountFriendList.tsx`

**功能特性**:
- ✅ 使用好友相关的 API (`DeboxAccountFriendApi`)
- ✅ 使用好友相关的类型定义 (`DeboxAccountFriend`, `GetDeboxAccountFriendsReq`)
- ✅ 实现好友列表的展示和管理
- ✅ 支持好友状态的开关切换
- ✅ 提供加载状态和空状态处理

**核心方法**:
- `fetchAccountFriends()` - 获取账号好友列表
- `updateAccountFriendStatus()` - 更新好友状态

### 3. 新增好友列表样式
**文件**: `mobile-web/src/pages/account-friend-management/components/AccountFriendList.module.scss`

**样式定义**:
- `.friendList` - 好友列表容器样式
- `.friendAvatar` - 好友头像尺寸设置

## 页面功能

### 🎯 核心功能
1. **账号选择**: 下拉选择要管理的 DeBox 账号
2. **刷新按钮**: 手动刷新好友列表
3. **同步按钮**: 从 DeBox 同步好友数据
4. **好友列表**: 展示当前账号的所有好友
5. **状态管理**: 开关控制好友的启用/禁用状态

### 🔄 数据流程
1. 用户选择账号
2. 点击"同步好友"调用后端 API
3. 从 DeBox 获取最新好友数据
4. 更新本地数据库
5. 刷新页面显示好友列表

### 📱 用户交互
- **下拉选择**: 账号选择器
- **按钮操作**: 刷新和同步功能
- **开关切换**: 好友状态管理
- **列表展示**: 好友信息展示

## API 接口

### 使用的接口
- `DeboxAccountFriendApi.syncFriends()` - 同步好友列表
- `DeboxAccountFriendApi.list()` - 获取好友列表
- `DeboxAccountFriendApi.updateStatus()` - 更新好友状态

### 请求参数
```typescript
// 同步好友
{
  account_ids: [accountId]
}

// 获取好友列表
{
  page: 0,
  page_size: 0,
  all: true,
  status: true,
  account_ids: [accountId]
}

// 更新好友状态
{
  id: friendId,
  status: boolean
}
```

## 组件结构

```
account-friend-management/
├── index.tsx                    # 主页面组件
├── index.module.scss           # 主页面样式
└── components/
    ├── AccountFriendList.tsx   # 好友列表组件
    └── AccountFriendList.module.scss # 好友列表样式
```

## 与关注人管理的差异

| 项目 | 关注人管理 | 好友管理 |
|------|------------|----------|
| 组件名称 | AccountFollowList | AccountFriendList |
| API 类 | DeboxAccountFollowApi | DeboxAccountFriendApi |
| 类型定义 | DeboxAccountFollow | DeboxAccountFriend |
| 同步接口 | syncFollows | syncFriends |
| 页面类名 | account-follow-management | account-friend-management |
| 状态变量 | followsUpdateState | friendsUpdateState |

## 兼容性说明

- ✅ 完全兼容现有的账号选择组件
- ✅ 复用通用的样式和布局
- ✅ 遵循项目统一的设计规范
- ✅ 保持与关注人管理相同的交互模式

## 使用示例

```typescript
// 页面使用
import FriendManagement from '@/pages/account-friend-management';

// 在路由中使用
<Route path="/account-friend-management" component={FriendManagement} />
```

## 注意事项

1. **权限控制**: 需要用户登录才能访问
2. **数据同步**: 同步操作会覆盖本地数据
3. **状态管理**: 好友状态变更会实时更新
4. **错误处理**: API 调用失败会显示错误信息
5. **加载状态**: 数据加载时显示加载指示器

---

**修改状态**: ✅ 完成  
**组件状态**: ✅ 新建完成  
**样式状态**: ✅ 新建完成  
**文档状态**: ✅ 完整