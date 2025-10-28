import { Button, Form, Input, Toast } from 'antd-mobile';
import { EyeInvisibleOutline, EyeOutline } from 'antd-mobile-icons';
import { JSX, useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import styles from './index.module.less';
import { LoginReq } from '@/typings/auth';
import { UserType } from '@/enums/auth';
import { AuthApi } from '@/api';
import { useAuthStore } from '@/stores';
import { cachedPasswordKey, cachedPhoneKey, cacheTokenKey } from '@/constant/auth';

export default function Login(): JSX.Element {
  const [passwordVisible, setPasswordVisible] = useState(false);
  const [submitting, setSubmitting] = useState(false);
  const [form] = Form.useForm<LoginReq>();
  const navigate = useNavigate();
  const authStore = useAuthStore.getState();

  // 测试用户
  const initialValues: LoginReq = {
    user_type: UserType.Phone,
    phone: '18312542746',
    password: '123456',
    captcha_id: '',
    captcha: '',
  };

  const [rememberPassword, setRememberPassword] = useState(false);

  useEffect(() => {
    // 读取缓存的用户名和密码
    const cachedPhone = localStorage.getItem(cachedPhoneKey);
    const cachedPassword = localStorage.getItem(cachedPasswordKey);
    if (cachedPhone && cachedPassword) {
      form.setFieldsValue({
        phone: cachedPhone,
        password: cachedPassword,
      });
    }
  }, [form]);

  const onFinish = async (values: LoginReq) => {
    if (submitting) {
      // 防止重复提交
      return;
    }

    setSubmitting(true);
    try {
      const response = await AuthApi.login(values);

      // 使用 Zustand 存储用户认证数据
      try {
        authStore.setToken(response.token);
        authStore.setUser(response.user_id, response.username, response.avatar);
        localStorage.setItem(cacheTokenKey, response.token);
      } catch (e) {
        console.log('存储 token 失败', e);
        // 某些隐私模式或受限环境下可能抛出异常，忽略但不要阻止登录流程
      }

      // 如果用户选择记住密码，则缓存用户名和密码
      if (rememberPassword) {
        localStorage.setItem(cachedPhoneKey, values.phone!);
        localStorage.setItem(cachedPasswordKey, values.password);
      } else {
        localStorage.removeItem(cachedPhoneKey);
        localStorage.removeItem(cachedPasswordKey);
      }

      Toast.show({ icon: 'success', content: '登录成功' });

      // 跳转到首页
      navigate('/', { replace: true });
    } catch (e) {
      // 避免把异常原文泄露给用户，显示友好的提示
      console.log('登录失败, err: ', e);
      Toast.show({ icon: 'fail', content: '登录失败，请稍后重试' });
    } finally {
      setSubmitting(false);
    }
  };

  // 当用户在密码输入框按回车时提交表单
  const handlePasswordEnter = () => {
    form.submit();
  };

  return (
    <div className={styles.login_container}>
      <div>
        <h1 className={styles.title}>欢迎登录</h1>
        <div className={styles.subtitle}>请输入账号和密码</div>
        <Form
          form={form}
          initialValues={initialValues}
          layout='horizontal'
          mode='card'
          onFinish={onFinish}
          footer={
            <div className={styles.actions}>
              <Button block color='primary' type='submit' disabled={submitting} loading={submitting}>
                {submitting ? '登录中...' : '登录'}
              </Button>
            </div>
          }
        >
          <Form.Item name='user_type' hidden>
            <Input type='hidden' />
          </Form.Item>
          <Form.Item name='captcha_id' hidden>
            <Input type='hidden' />
          </Form.Item>
          <Form.Item name='captcha' hidden>
            <Input type='hidden' />
          </Form.Item>

          <Form.Item
            name='phone'
            label='账号'
            rules={[
              { required: true, message: '请输入账号' },
              { min: 3, message: '用户名至少3位' },
            ]}
          >
            <Input
              clearable
              placeholder='请输入用户名'
              onBlur={() => {
                // 自动 trim 并回填到表单，保持数据规范
                const val = (form.getFieldValue('phone') || '').toString();
                const trimmed = val.trim();
                if (trimmed !== val) form.setFieldsValue({ phone: trimmed });
              }}
              aria-label='用户名'
            />
          </Form.Item>

          <Form.Item
            name='password'
            label='密码'
            rules={[
              { required: true, message: '请输入密码' },
              { min: 6, message: '密码至少6位' },
            ]}
            extra={
              <div className={styles.eye}>
                {!passwordVisible ? (
                  <EyeInvisibleOutline onClick={() => setPasswordVisible(true)} />
                ) : (
                  <EyeOutline onClick={() => setPasswordVisible(false)} />
                )}
              </div>
            }
          >
            <Input
              type={passwordVisible ? 'text' : 'password'}
              placeholder='请输入密码'
              clearable
              maxLength={64}
              onEnterPress={handlePasswordEnter}
              aria-label='密码'
            />
          </Form.Item>
        </Form>

        <div className={styles.tips}>
          <Button size='small' fill='none' onClick={() => navigate('/register')}>
            没有账号？去注册
          </Button>
          <div className={styles.remember}>
            <input
              type='checkbox'
              id='rememberPassword'
              checked={rememberPassword}
              onChange={(e) => setRememberPassword(e.target.checked)}
            />
            <label htmlFor='rememberPassword'>记住密码</label>
          </div>
        </div>
      </div>
    </div>
  );
}
