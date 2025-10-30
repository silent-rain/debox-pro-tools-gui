# 登录页面多模式改造说明

## 功能说明
本次改造为移动端登录页引入三种可切换的登录模式，并将每种模式抽离为独立子组件：
- 用户名登录（BaseLogin）
- 手机号登录（PhoneLogin）
- 邮箱登录（EmailLogin）

登录页通过顶部选项切换渲染对应组件。保持原有鉴权流程、Zustand 状态写入与 Token 缓存逻辑不变；新增统一的“记住密码”控制，父组件根据当前登录类型分别读取/写入对应的账号与密码缓存，且兼容旧的手机号缓存键。

涉及页面/组件：
- mobile-web/src/pages/auth/login/index.tsx
- mobile-web/src/pages/auth/login/components/BaseLogin.tsx
- mobile-web/src/pages/auth/login/components/PhoneLogin.tsx
- mobile-web/src/pages/auth/login/components/EmailLogin.tsx

新增/调整的缓存键：
- 兼容旧键（原手机登录）：cachedPhone、cachedPassword
- 新增分类型账号键：cachedUsername、cachedEmail
- 新增分类型密码键：cachedPassword_phone、cachedPassword_username、cachedPassword_email

后端契约：沿用 LoginReq 与 UserType（Base/Phone/Email）。

## Props 参数表

```ts
// BaseLogin
export interface BaseLoginProps {
  submitting: boolean;
  cachedUsername?: string | null;
  cachedPassword?: string | null;
  onSubmit: (payload: LoginReq) => Promise<void> | void;
}

// PhoneLogin
export interface PhoneLoginProps {
  submitting: boolean;
  cachedPhone?: string | null;
  cachedPassword?: string | null;
  onSubmit: (payload: LoginReq) => Promise<void> | void;
}

// EmailLogin
export interface EmailLoginProps {
  submitting: boolean;
  cachedEmail?: string | null;
  cachedPassword?: string | null;
  onSubmit: (payload: LoginReq) => Promise<void> | void;
}
```

类型说明：
- LoginReq 按项目 typings/auth.d.ts 定义，必要字段：user_type、password；可选字段：username/phone/email 等。
- UserType 枚举按项目 enums/auth.ts 定义（Base=0, Phone=1, Email=2）。

## 使用示例

1) 用户名登录
```tsx
<BaseLogin
  submitting={submitting}
  cachedUsername={cachedUsername}
  cachedPassword={usernamePwd}
  onSubmit={onSubmit}
/>
```

2) 手机号登录（含统一记住密码逻辑）
```tsx
<PhoneLogin
  submitting={submitting}
  cachedPhone={cachedPhone}
  cachedPassword={phonePwd}
  onSubmit={onSubmit}
/>
```

3) 邮箱登录
```tsx
<EmailLogin
  submitting={submitting}
  cachedEmail={cachedEmail}
  cachedPassword={emailPwd}
  onSubmit={onSubmit}
/>
```

父组件 onSubmit 示例（节选）：
```ts
const onSubmit = async (values: LoginReq) => {
  const resp = await AuthApi.login(values);
  authStore.setToken(resp.token);
  authStore.setUser();
  localStorage.setItem(cacheTokenKey, resp.token);

  if (rememberPassword) {
    switch (values.user_type) {
      case UserType.Phone:
        localStorage.setItem(cachedPhoneKey, values.phone || '');
        localStorage.setItem(cachedPasswordPhoneKey, values.password);
        localStorage.setItem(cachedPasswordKey, values.password); // 兼容旧键
        break;
      case UserType.Base:
        localStorage.setItem(cachedUsernameKey, values.username || '');
        localStorage.setItem(cachedPasswordUsernameKey, values.password);
        break;
      case UserType.Email:
        localStorage.setItem(cachedEmailKey, values.email || '');
        localStorage.setItem(cachedPasswordEmailKey, values.password);
        break;
    }
  } else {
    // 清理对应类型缓存
    // ...同代码实现
  }
}
```

## 兼容性说明
- UI 基于 antd-mobile 与 React 18+/TypeScript，样式中少量使用 Tailwind 实用类（项目已启用 tailwind）。
- 不涉及外部服务依赖变更；API 契约与原登录一致。
- 记住密码逻辑保持向后兼容：手机号模式沿用 cachedPhone/cachedPassword，同时写入新的分类型密码键。

## 注意事项
- “记住密码”开关统一在父组件 index.tsx 中渲染与管理，子组件不承担此状态。
- 验证规则：
  - 用户名：至少 3 位
  - 手机号：^\d{6,20}$（可按业务调整）
  - 邮箱：通用邮箱正则（可按业务调整）
- 性能：子组件独立渲染，切换仅重渲染当前模式；提交防重由父组件 submitting 控制。
- 可测试性：各表单的 onFinish 统一回调 onSubmit，便于单元测试模拟不同 user_type 的请求。
- 可扩展性：如需新增“区块链钱包登录”等模式，新增组件并在 tabs 中追加即可。
