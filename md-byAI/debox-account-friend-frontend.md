# DeBox 账号好友前端接口

## 功能说明

基于 DeBox 账号关注人的前端接口实现，完整创建了 DeBox 账号好友的前端接口。包括类型定义、枚举定义和 API 调用方法，与后端接口完全对应。

## 文件结构

```
mobile-web/src/
├── typings/
│   └── debox-account-friend.d.ts      # 类型定义
├── enums/
│   └── debox-account-friend.ts        # 枚举定义
└── api/
    └── debox-account-friend.ts        # API 接口
```

## 类型定义 (typings/debox-account-friend.d.ts)

### 核心数据类型

#### DeboxAccountFriend
```typescript
export interface DeboxAccountFriend {
  id: number;           // 好友ID
  user_id: number;      // 用户ID
  account_id: number;   // 账号ID
  debox_user_id: string; // DeBox 用户ID
  name: string;         // 用户名称
  avatar: string;       // 账号头像
  desc: string;         // 描述信息
  status: boolean;      // 状态
  created_at: string;   // 创建时间
  updated_at: string;   // 更新时间
}
```

### 请求/响应类型

#### 列表查询
```typescript
// 请求
export interface GetDeboxAccountFriendsReq {
  page?: number;           // 当前分页
  page_size?: number;      // 页面大小
  start_time?: string;     // 开始时间
  end_time?: string;       // 结束时间
  all: boolean;            // 返回所有数据
  account_ids: number[];   // 账号IDs
  status: boolean;         // 账号状态
  sorts?: string[];        // 排序字段, ["id:asc"]
}

// 响应
export interface GetDeboxAccountFriendsResp {
  data_list: DeboxAccountFriend[];
  total: number;
}
```

#### 详情查询
```typescript
// 请求
export interface GetDeboxAccountFriendReq {
  id: number;  // 账号好友ID
}

// 响应
export type GetDeboxAccountFriendResp = DeboxAccountFriend;
```

#### 创建好友
```typescript
// 请求
export interface CreateDeboxAccountFriendReq {
  account_id: number;      // 账号ID
  debox_user_id: string;   // DeBox 用户ID
  name: string;            // 好友名称
  avatar: string;          // 账号头像
  desc: string;            // 描述信息
  status: boolean;         // 状态
}

// 响应
export interface CreateDeboxAccountFriendResp {}
```

#### 更新好友
```typescript
// 请求
export interface UpdateDeboxAccountFriendReq {
  id: number;       // 账号好友ID
  name: string;     // 好友名称
  avatar: string;   // 账号头像
  desc: string;     // 描述信息
  status: boolean;  // 状态
}

// 响应
export interface UpdateDeboxAccountFriendResp {}
```

#### 状态更新
```typescript
// 请求
export interface UpdateDeboxAccountFriendStatusReq {
  id: number;       // 账号好友ID
  status: boolean;  // 状态
}

// 响应
export interface UpdateDeboxAccountFriendStatusResp {}
```

#### 删除好友
```typescript
// 请求
export interface DeleteDeboxAccountFriendReq {
  id: number;  // 账号好友ID
}

// 响应
export interface DeleteDeboxAccountFriendResp {}
```

#### 同步好友列表
```typescript
// 请求
export interface SyncDeboxAccountFriendsReq {
  account_ids: number[];  // 账号IDs
}

// 响应
export interface SyncDeboxAccountFriendsResp {}
```

#### 批量添加好友
```typescript
// 请求
export interface BatchAccountFriendsReq {
  account_id: number;           // 账号ID
  target_account_id?: number;   // 目标账号ID
  target_group_id?: number;     // 目标群组ID
  debox_user_ids?: string[];    // DeBox用户IDs
  friend_type: FriendType;      // 添加好友账号类型
}

// 响应
export interface BatchAccountFriendsResp {}
```

#### 用户搜索
```typescript
// 请求
export interface DeboxUserSearchReq {
  account_id: number;  // 账号ID
  search: string;      // 搜索关键词
  page: number;        // 当前分页
  size: number;        // 页面大小
}

// 响应
export interface DeboxUserSearchResp {
  data_list: UserSearch[];
}
```

#### 用户搜索结果
```typescript
export interface UserSearch {
  user_id: number;
  name: string;
  group_alias: string;
  alias_name: string;
  pic: string;
  chain_id: number;
  address: string;
  solana_address: string;
  tron_address: string;
  colors?: any;
  is_admin: number;
  is_builder: number;
  is_founder: number;
  is_role: number;
  identity?: any;
  user_label?: any;
  ext_text: string;
  icons: any[];
}
```

## 枚举定义 (enums/debox-account-friend.ts)

```typescript
// 添加好友类型
export enum FriendType {
  // 账号
  Account = 0,
  // 群组
  Group = 1,
  // 用户
  User = 2,
}
```

## API 接口 (api/debox-account-friend.ts)

### 基础 CRUD 接口

#### 获取好友列表
```typescript
list: async (data: GetDeboxAccountFriendsReq): Promise<GetDeboxAccountFriendsResp>
```

#### 获取好友详情
```typescript
info: async (data: GetDeboxAccountFriendReq): Promise<GetDeboxAccountFriendResp>
```

#### 创建好友
```typescript
create: async (data: CreateDeboxAccountFriendReq): Promise<CreateDeboxAccountFriendResp>
```

#### 更新好友信息
```typescript
update: async (data: UpdateDeboxAccountFriendReq): Promise<UpdateDeboxAccountFriendResp>
```

#### 更新好友状态
```typescript
updateStatus: async (data: UpdateDeboxAccountFriendStatusReq): Promise<UpdateDeboxAccountFriendStatusResp>
```

#### 删除好友
```typescript
delete: async (data: DeleteDeboxAccountFriendReq): Promise<DeleteDeboxAccountFriendResp>
```

### 扩展功能接口

#### 同步好友列表
```typescript
syncFriends: async (data: SyncDeboxAccountFriendsReq): Promise<SyncDeboxAccountFriendsResp>
```

#### 批量添加好友
```typescript
batchFriends: async (data: BatchAccountFriendsReq): Promise<BatchAccountFriendsResp>
```

#### 用户搜索
```typescript
deboxUserSearch: async (data: DeboxUserSearchReq): Promise<DeboxUserSearchResp>
```

## 使用示例

### 1. 获取好友列表
```typescript
import { DeboxAccountFriendApi } from '@/api/debox-account-friend';

const getFriendList = async () => {
  try {
    const response = await DeboxAccountFriendApi.list({
      page: 1,
      page_size: 20,
      all: false,
      account_ids: [1, 2],
      status: true,
      sorts: ['created_at:desc']
    });
    
    console.log('好友列表:', response.data_list);
    console.log('总数:', response.total);
  } catch (error) {
    console.error('获取好友列表失败:', error);
  }
};
```

### 2. 创建好友
```typescript
const createFriend = async () => {
  try {
    const response = await DeboxAccountFriendApi.create({
      account_id: 1,
      debox_user_id: 'debox_user_123',
      name: '张三',
      avatar: 'https://example.com/avatar.jpg',
      desc: '我的好朋友',
      status: true
    });
    
    console.log('创建好友成功:', response);
  } catch (error) {
    console.error('创建好友失败:', error);
  }
};
```

### 3. 更新好友信息
```typescript
const updateFriend = async () => {
  try {
    const response = await DeboxAccountFriendApi.update({
      id: 1,
      name: '李四',
      avatar: 'https://example.com/new-avatar.jpg',
      desc: '更新的描述信息',
      status: true
    });
    
    console.log('更新好友成功:', response);
  } catch (error) {
    console.error('更新好友失败:', error);
  }
};
```

### 4. 更新好友状态
```typescript
const updateFriendStatus = async () => {
  try {
    const response = await DeboxAccountFriendApi.updateStatus({
      id: 1,
      status: false
    });
    
    console.log('更新状态成功:', response);
  } catch (error) {
    console.error('更新状态失败:', error);
  }
};
```

### 5. 删除好友
```typescript
const deleteFriend = async () => {
  try {
    const response = await DeboxAccountFriendApi.delete({
      id: 1
    });
    
    console.log('删除好友成功:', response);
  } catch (error) {
    console.error('删除好友失败:', error);
  }
};
```

### 6. 同步好友列表
```typescript
const syncFriends = async () => {
  try {
    const response = await DeboxAccountFriendApi.syncFriends({
      account_ids: [1, 2, 3]
    });
    
    console.log('同步好友成功:', response);
  } catch (error) {
    console.error('同步好友失败:', error);
  }
};
```

### 7. 批量添加好友
```typescript
import { FriendType } from '@/enums/debox-account-friend';

const batchAddFriends = async () => {
  try {
    const response = await DeboxAccountFriendApi.batchFriends({
      account_id: 1,
      target_account_id: 2,
      friend_type: FriendType.Account,
      debox_user_ids: ['user1', 'user2', 'user3']
    });
    
    console.log('批量添加好友成功:', response);
  } catch (error) {
    console.error('批量添加好友失败:', error);
  }
};
```

### 8. 用户搜索
```typescript
const searchUsers = async () => {
  try {
    const response = await DeboxAccountFriendApi.deboxUserSearch({
      account_id: 1,
      search: '张三',
      page: 1,
      size: 10
    });
    
    console.log('搜索结果:', response.data_list);
  } catch (error) {
    console.error('用户搜索失败:', error);
  }
};
```

## 接口对应关系

| 前端接口 | 后端路由 | HTTP 方法 |
|---------|----------|-----------|
| `list` | `/debox/debox-account-friends/list` | POST |
| `info` | `/debox/debox-account-friends/{id}` | GET |
| `create` | `/debox/debox-account-friends/` | POST |
| `update` | `/debox/debox-account-friends/update` | PUT |
| `updateStatus` | `/debox/debox-account-friends/update-status` | PUT |
| `delete` | `/debox/debox-account-friends/{id}` | DELETE |
| `syncFriends` | `/debox/debox-account-friends/sync-friends` | POST |
| `batchFriends` | `/debox/debox-account-friends/batch-friends` | POST |
| `deboxUserSearch` | `/debox/debox-account-friends/debox-user-search` | POST |

## 错误处理

所有接口都使用统一的错误处理机制，通过 `try-catch` 捕获异常，错误信息包含：
- HTTP 状态码
- 错误消息
- 详细错误信息（如果有）

## 类型安全

- 所有接口都使用 TypeScript 严格类型检查
- 请求和响应参数都有完整的类型定义
- 枚举类型确保参数值的正确性
- IDE 可以提供完整的代码提示和类型检查

## 兼容性说明

- 与现有的 DeBox 账号关注人接口完全兼容
- 遵循相同的命名规范和代码风格
- 使用相同的请求/响应格式
- 支持相同的错误处理机制

## 注意事项

1. **参数验证**: 前端应该进行基本的参数验证，但主要验证逻辑在后端
2. **错误处理**: 统一使用 try-catch 处理异步操作错误
3. **类型导入**: 使用相对路径导入类型定义，确保类型安全
4. **枚举使用**: 使用枚举类型而不是魔法数字，提高代码可读性
5. **接口复用**: 相同的请求/响应类型可以在多个组件中复用