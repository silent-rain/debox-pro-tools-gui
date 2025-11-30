# useLogout Hook 使用指南

## 功能说明

`useLogout` 是一个自定义 Hook，用于处理用户登出逻辑。它封装了登出确认弹窗、清除登录信息和页面跳转的功能。

## Props参数表格

| 参数名 | 类型 | 说明 |
|--------|------|------|
| 返回值 | `{ logout: () => void }` | 返回包含登出函数的对象 |

## 使用示例

### 示例1：在组件中使用 useLogout Hook

```tsx
import React from 'react';
import { Button } from 'antd-mobile';
import { useLogout } from '@/hooks/useLogout';

const UserProfile: React.FC = () => {
  const { logout } = useLogout();

  const handleLogout = () => {
    logout();
  };

  return (
    <div>
      <h1>用户资料</h1>
      <Button onClick={handleLogout} color="danger">
        退出登录
      </Button>
    </div>
  );
};

export default UserProfile;
```

### 示例2：在菜单组件中使用

```tsx
import React from 'react';
import { List } from 'antd-mobile';
import { useLogout } from '@/hooks/useLogout';

const SettingsMenu: React.FC = () => {
  const { logout } = useLogout();

  return (
    <List>
      <List.Item
        onClick={logout}
        arrow={false}
        style={{ color: 'var(--adm-color-danger)' }}
      >
        退出登录
      </List.Item>
    </List>
  );
};

export default SettingsMenu;
```

### 示例3：结合其他逻辑使用

```tsx
import React, { useCallback } from 'react';
import { Button, Toast } from 'antd-mobile';
import { useLogout } from '@/hooks/useLogout';

const Header: React.FC = () => {
  const { logout } = useLogout();

  const handleLogoutWithCleanup = useCallback(() => {
    // 执行一些清理逻辑
    Toast.show('正在退出...');
    
    // 清理其他状态
    localStorage.removeItem('userPreferences');
    
    // 执行登出
    logout();
  }, [logout]);

  return (
    <header>
      <Button onClick={handleLogoutWithCleanup} size="small">
        安全退出
      </Button>
    </header>
  );
};

export default Header;
```

## 兼容性说明

### 依赖项
- React 18+
- React Router 6+
- antd-mobile 5+

### 使用场景
- 用户主动退出登录
- 会话过期自动登出
- 权限验证失败时登出

### 注意事项
1. 该 Hook 只能在函数组件内部使用
2. 登出后会自动清除 `localStorage` 中的 token
3. 会显示确认弹窗，用户确认后才执行登出
4. 使用 `replace: true` 进行页面跳转，避免用户通过返回按钮回到需要登录的页面

### 相关文件
- `@/utils/request.ts` - 包含 `dispatchLogout` 核心逻辑
- `@/constants/routes.ts` - 定义路由常量
- `@/constants/auth.ts` - 定义认证相关常量