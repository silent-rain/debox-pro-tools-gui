# 移动端WEB项目开发指南

你是一个专业的 React、TypeScript、Ant Design Mobile 和移动 UI 开发专家。

## 项目技术架构

- [React](https://zh-hans.react.dev/learn/typescript)
- [TypeScript](https://www.typescriptlang.org/docs/)
- [Ant Design Mobile](https://mobile.ant.design/)
- [less](https://less.bootcss.com/)
- [tailwind](https://www.tailwindcss.cn/)

## 项目结构规范

- ​​单一职责原则​​：每个文件应只包含一个逻辑组件，便于维护和理解代码结构。
​​- 模块化组织​​：采用ESModule标准，每个文件只暴露必要的接口。
- ​​类型定义位置​​：在文件内部，类型定义应该出现在顶部，便于读者首先了解数据结构。

## 文件命名规范

- 使用​​小写字母和短横线​​表示目录（例如：components/auth-wizard）
- 为组件使用​​命名导出​​而非默认导出
- 组件文件使用 ​​PascalCase​​，工具文件使用​​camelCase​

## React与组件开发规范

### 组件设计原则

```ts
// ✅ 使用函数组件和TypeScript接口
interface ButtonProps {
  variant: 'primary' | 'secondary';
  children: React.ReactNode;
  onClick?: () => void;
}

const CustomButton: React.FC<ButtonProps> = ({ 
  variant, 
  children, 
  onClick 
}) => {
  return (
    <Button type={variant} onClick={onClick}>
      {children}
    </Button>
  );
};
```

### 状态管理优化

```ts
// ✅ 最小化useState使用，优先使用上下文和reducers进行状态管理
const AppContext = React.createContext<AppState | undefined>(undefined);

// ✅ 使用useMemo和useCallback避免不必要的重新渲染[9](@ref)
const memoizedValue = useMemo(() => computeExpensiveValue(a, b), [a, b]);
const memoizedCallback = useCallback(() => { doSomething(a); }, [a]);
```

## 工程化与团队协作规则

### 代码质量保障

- 使用Prettier​​实现一致的代码格式化
​​- 启用ESLint​​进行代码质量检查
​​- 实施Husky​​预提交钩子确保代码质量
​​- 编写单元测试​​使用Jest和React Native Testing Library
