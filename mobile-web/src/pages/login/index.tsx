import { Button, Form, Input, Toast } from 'antd-mobile';
import { EyeInvisibleOutline, EyeOutline } from 'antd-mobile-icons';
import { JSX, useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import styles from './index.module.less';

interface LoginValue {
  username: string;
  password: string;
}

export default function Login(): JSX.Element {
  const [passwordVisible, setPasswordVisible] = useState(false);
  const [submitting, setSubmitting] = useState(false);
  const [form] = Form.useForm<LoginValue>();
  const navigate = useNavigate();

  // 测试数据
  useEffect(() => {
    // 设置字段值
    form.setFieldsValue({
      username: '18312542746',
      password: '123456',
    });
  }, [form]);

  const onFinish = async (values: LoginValue) => {
    console.log('values', values);

    if (submitting) {
      // 防止重复提交
      return;
    }

    setSubmitting(true);
    try {
      // TODO: 使用真正的登录接口替换此处模拟逻辑
      await new Promise((r) => setTimeout(r, 600));

      // 存储 token。生产环境请使用 HttpOnly Cookie 或者后端会话管理，避免在前端存放敏感凭证。
      try {
        localStorage.setItem('token', 'mock-token');
      } catch (e) {
        console.log('存储 token 失败', e);
        // 某些隐私模式或受限环境下可能抛出异常，忽略但不要阻止登录流程
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
      <div className={styles.card}>
        <h1 className={styles.title}>欢迎登录</h1>
        <div className={styles.subtitle}>请输入账号和密码</div>
        <Form
          form={form}
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
          <Form.Item
            name='username'
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
                const val = (form.getFieldValue('username') || '').toString();
                const trimmed = val.trim();
                if (trimmed !== val) form.setFieldsValue({ username: trimmed });
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
        </div>
      </div>
    </div>
  );
}
