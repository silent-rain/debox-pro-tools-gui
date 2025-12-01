# Less 到 Sass 迁移文档

## 功能说明

本文档记录了将整个 DeBox Pro Tools GUI 项目从 Less 预处理器迁移到 Sass 预处理器的完整过程。

## 迁移内容

### 1. 依赖更新
- 移除 `postcss-less` 依赖
- 移除 `stylelint-config-recommended-less` 依赖
- 添加 `sass` 依赖
- 添加 `stylelint-scss` 依赖
- 更新 `stylelint-config-standard` 为 `stylelint-config-standard-scss`

### 2. 配置文件更新
- 更新 `package.json` 中的 stylelint 脚本路径
- 更新 `.stylelintrc.mjs` 配置以支持 Sass 语法
- 更新 `src/vite-env.d.ts` 添加 `.scss` 模块类型声明
- 更新 `index.html` 中的样式文件引用

### 3. 文件转换
- 将所有 `.less` 文件重命名为 `.scss`
- 将 `main.css` 重命名为 `main.scss`
- 更新所有 TypeScript/JSX 文件中的 import 语句
- 转换 Less 语法为 Sass 语法：
  - 变量声明：`@variable:` → `$variable:`
  - 变量引用：`@variable` → `$variable`
  - 保留 `@keyframes` 和 `@media` 规则

### 4. 语法修复
- 修复语法转换过程中误转换的 `@keyframes` 和 `@media` 规则
- 删除空的样式文件
- 修复 stylelint 配置以支持 CSS Modules 的 `:global` 伪类

## 转换后的文件结构

```
src/
├── styles/
│   └── colors.scss           # 全局颜色变量
├── main.scss                 # 主样式文件
├── components/
│   ├── page-loading/
│   │   └── index.module.scss
│   ├── account-list/
│   │   └── index.module.scss
│   └── ...
├── layouts/
│   ├── header.module.scss
│   ├── footer.module.scss
│   └── index.module.scss
├── pages/
│   ├── home/
│   │   └── index.module.scss
│   ├── personal-center/
│   │   └── about/
│   │       └── index.module.scss
│   └── ...
└── pages/error404/
    └── index.module.scss
```

## 兼容性说明

### Sass 语法特性
- 使用 `$` 符号定义变量
- 支持 `@use` 和 `@import`（当前使用 `@import` 以保持兼容性）
- 支持嵌套规则、混合器、函数等高级特性

### 构建工具兼容性
- Vite 原生支持 `.scss` 文件
- PostCSS 配置无需修改
- TypeScript 类型检查正常
- ESLint 检查正常

### 注意事项
1. **Tailwind CSS 集成**：继续使用 `@import "tailwindcss/utilities"` 语法
2. **CSS Modules**：保持 `.module.scss` 命名规范
3. **全局样式**：`:global()` 伪类在 stylelint 中已配置支持

## 使用示例

### 变量定义和使用
```scss
// colors.scss
$primary-color: #667eea;
$primary-gradient: linear-gradient(135deg, #667eea 0%, #764ba2 100%);

// 组件中使用
.button {
  background: $primary-gradient;
  color: $primary-color;
}
```

### 嵌套规则
```scss
.card {
  padding: 16px;
  border-radius: 8px;
  
  &__header {
    font-weight: 600;
    margin-bottom: 12px;
  }
  
  &:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
}
```

### 媒体查询
```scss
.container {
  width: 100%;
  
  @media (min-width: 768px) {
    max-width: 1200px;
    margin: 0 auto;
  }
}
```

## 验证结果

迁移完成后，以下功能均已验证正常：
- ✅ TypeScript 类型检查通过
- ✅ ESLint 代码检查通过
- ✅ Stylelint 样式检查通过
- ✅ Vite 开发服务器启动成功
- ✅ 项目构建成功（除了一些已知的警告）

## 后续建议

1. **逐步迁移到 `@use` 语法**：考虑在未来版本中将 `@import` 迁移到 `@use` 以获得更好的模块化支持
2. **利用 Sass 高级特性**：可以使用 Sass 的混合器、函数等特性来提高样式复用性
3. **性能优化**：考虑使用 Sass 的 `@extend` 和 `%placeholder` 来优化 CSS 输出

## 迁移脚本

项目包含了两个迁移脚本供参考：
- `convert-less-to-scss.sh`：文件重命名和 import 更新
- `convert-syntax.sh`：Less 语法到 Sass 语法转换

这些脚本已保存在项目根目录，可用于类似的迁移任务。