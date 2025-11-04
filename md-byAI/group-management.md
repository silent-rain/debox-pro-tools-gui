# 群组管理功能文档

## 功能说明

群组管理页面提供了对群组的管理功能，包括选择账户、批量同步群组以及禁用群组操作。

## Props参数表格

| 参数名 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| selectedAccount | string | 'all' | 当前选中的账户 |
| selectedGroups | string[] | [] | 当前选中的群组ID列表 |

## 使用示例

### 基本使用

```tsx
import GroupManagement from '@/pages/group-management';

const App = () => {
  return <GroupManagement />;
};
```

### 自定义账户列表

```tsx
const accounts = [
  { label: '自定义账户1', value: 'custom1' },
  { label: '自定义账户2', value: 'custom2' },
];

const App = () => {
  return <GroupManagement accounts={accounts} />;
};
```

### 自定义群组列表

```tsx
const groups = [
  { id: '101', name: '自定义群组1' },
  { id: '102', name: '自定义群组2' },
];

const App = () => {
  return <GroupManagement groups={groups} />;
};
```

## 兼容性说明

- 依赖 `antd-mobile` 组件库。
- 需要 `react-router-dom` 进行页面导航。
- 适用于移动端 Web 应用。
