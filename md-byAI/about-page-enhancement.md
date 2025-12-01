# 关于页面完善文档

## 功能说明

完善了DeBox Pro Tools项目的关于页面，提供了详细的应用信息、功能介绍和联系方式。

## 主要新增内容

### 1. 应用信息展示
- 应用名称、版本号、构建时间
- 应用描述和图标
- 版本号可复制功能

### 2. 功能介绍
- 六大核心功能详细说明
- 清晰的功能列表展示
- 图标化的功能分类

### 3. 联系方式
- 官方网站链接
- 邮箱支持
- 客服热线
- 使用文档入口

### 4. UI设计优化
- 渐变背景设计
- 卡片式布局
- 响应式设计
- 交互动效和悬停效果

## Props参数表格

该页面为静态页面，无需外部Props参数。

## 使用示例

### 示例1：基础使用
```tsx
// 在路由中配置
<Route path="/personal-center/about" element={<About />} />

// 在个人中心页面中跳转
<List.Item onClick={() => navigate('/personal-center/about')}>
  关于
</List.Item>
```

### 示例2：自定义版本信息
```tsx
// 修改应用信息
const [appInfo] = useState({
  version: '2.0.0',
  buildTime: '2024-12-01',
  appName: 'My App',
  description: '自定义应用描述'
});
```

### 示例3：自定义联系方式
```tsx
// 修改联系方式处理函数
const handleContact = (type: string) => {
  switch (type) {
    case 'email':
      window.open('mailto:support@example.com');
      break;
    case 'website':
      window.open('https://example.com');
      break;
    default:
      break;
  }
};
```

## 兼容性说明

### 浏览器兼容性
- Chrome 88+
- Firefox 85+
- Safari 14+
- Edge 88+

### 设备兼容性
- 移动端优化设计
- 响应式布局适配
- 触摸友好的交互设计

### 依赖项
- React 18.3.1+
- antd-mobile 5.41.1+
- antd-mobile-icons 0.3.0+

## 注意事项

1. **图标使用**：使用了antd-mobile-icons中的图标组件，确保版本兼容性
2. **样式依赖**：使用了Less预处理器，需要配置相应的loader
3. **路由依赖**：依赖react-router-dom进行导航
4. **Toast提示**：使用了antd-mobile的Toast组件进行用户反馈

## 文件结构

```
src/pages/personal-center/about/
├── index.tsx          # 主组件文件
└── index.module.less  # 样式文件
```

## 使用到的组件或页面

- **个人中心页面** (`/src/pages/personal-center/index.tsx`)：包含关于页面的入口链接
- **路由配置** (`/src/constants/routes.ts`)：定义了关于页面的路由常量 `PERSONAL_CENTER_ABOUT`

## 更新日志

- **v1.0.0** (2024-12-01): 初始版本发布
  - 完成基础功能实现
  - 添加UI设计和交互效果
  - 完善响应式布局