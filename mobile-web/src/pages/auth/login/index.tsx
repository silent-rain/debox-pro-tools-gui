import { Toast, Selector, Button } from 'antd-mobile';
import { JSX, useCallback, useMemo, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { LoginReq } from '@/typings/auth';
import { UserType } from '@/enums/auth';
import { AuthApi } from '@/api';
import { useAuthStore } from '@/stores';
import styles from './index.module.less';
import PhoneLogin from './components/PhoneLogin';
import BaseLogin from './components/BaseLogin';
import EmailLogin from './components/EmailLogin';

export default function Login(): JSX.Element {
  const navigate = useNavigate();
  const authStore = useAuthStore.getState();

  const [submitting, setSubmitting] = useState(false);
  const [loginType, setLoginType] = useState<UserType>(UserType.Base);

  const tabs = useMemo(
    () => [
      { key: UserType.Base, label: '用户名' },
      { key: UserType.Phone, label: '手机号' },
      { key: UserType.Email, label: '邮箱' },
    ],
    [],
  );

  const onSubmit = useCallback(
    async (values: LoginReq) => {
      if (submitting) return; // 防重
      setSubmitting(true);
      try {
        const response = await AuthApi.login(values);
        try {
          authStore.setToken(response.token);
          authStore.setUser();
        } catch (e) {
          // 某些环境可能禁用本地存储，不阻断登录
          console.log('存储 token 失败', e);
        }

        Toast.show({ icon: 'success', content: '登录成功' });
        navigate('/', { replace: true });
      } catch (e) {
        console.log('登录失败, err: ', e);
        Toast.show({ icon: 'fail', content: '登录失败，请稍后重试' });
      } finally {
        setSubmitting(false);
      }
    },
    [authStore, navigate, submitting],
  );

  // 处理登录方式选择变化（Selector 单选模式）
  const onLoginTypeChange = useCallback((arr: string[]) => {
    const first = arr?.[0];
    if (first === undefined || first === null) return;
    const next = Number(first);
    if (!Number.isNaN(next)) {
      setLoginType(next as UserType);
    }
  }, []);

  return (
    <div className={styles.login_container}>
      <div>
        <h1 className={styles.title}>欢迎登录</h1>
        <div className={styles.subtitle}>请选择登录方式并输入凭证</div>

        {/* 登录方式切换 - 使用 Selector 单选模式 */}
        <div className={styles.user_type}>
          <Selector
            options={tabs.map((t) => ({ label: t.label, value: String(t.key) }))}
            value={[String(loginType)]}
            onChange={onLoginTypeChange}
          />
        </div>

        {/* 对应的登录表单 */}
        <div>
          {loginType === UserType.Phone && <PhoneLogin submitting={submitting} onSubmit={onSubmit} />}
          {loginType === UserType.Base && <BaseLogin submitting={submitting} onSubmit={onSubmit} />}
          {loginType === UserType.Email && <EmailLogin submitting={submitting} onSubmit={onSubmit} />}
        </div>

        <div className={styles.tips}>
          <Button size='mini' color='primary' fill='none' onClick={() => navigate('/register')}>
            没有账号？去注册
          </Button>
        </div>
      </div>
    </div>
  );
}
