import { JSX, useCallback, useMemo, useState } from 'react';
import { Form, Input, Button, Checkbox } from 'antd-mobile';
import { EyeInvisibleOutline, EyeOutline } from 'antd-mobile-icons';
import { LoginReq } from '@/typings/auth';
import { UserType } from '@/enums/auth';
import { cachedPasswordUsernameKey, cachedUsernameKey } from '@/constant/auth';
import styles from './index.module.less';

export interface BaseLoginProps {
  submitting: boolean;
  onSubmit: (payload: LoginReq) => Promise<void> | void;
}

export default function BaseLogin({ submitting, onSubmit }: BaseLoginProps): JSX.Element {
  const [form] = Form.useForm<LoginReq>();
  const [passwordVisible, setPasswordVisible] = useState(false);
  const [rememberPassword, setRememberPassword] = useState<boolean>(false);

  const initialValues: LoginReq = useMemo(
    () => ({
      user_type: UserType.Base,
      username: localStorage.getItem(cachedUsernameKey) || 'SR',
      password: localStorage.getItem(cachedPasswordUsernameKey) || '123456',
      captcha_id: '',
      captcha: '',
    }),
    [],
  );

  useMemo(() => {
    setRememberPassword(Boolean(initialValues.username || initialValues.password));
  }, [initialValues]);

  const handlePasswordEnter = useCallback(() => {
    form.submit();
  }, [form]);

  const handleFinish = useCallback(
    async (values: LoginReq) => {
      if (rememberPassword) {
        localStorage.setItem(cachedUsernameKey, values.username || '');
        localStorage.setItem(cachedPasswordUsernameKey, values.password);
      } else {
        localStorage.removeItem(cachedUsernameKey);
        localStorage.removeItem(cachedPasswordUsernameKey);
      }

      try {
        await onSubmit(values);
      } catch (e) {
        console.log('本地缓存失败: ', e);
      }
    },
    [onSubmit, rememberPassword],
  );

  return (
    <>
      <Form
        form={form}
        initialValues={initialValues}
        layout='horizontal'
        mode='card'
        onFinish={handleFinish}
        footer={
          <div className='mt-4'>
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
          name='username'
          label='用户名'
          rules={[
            { required: true, message: '请输入用户名' },
            { min: 2, message: '用户名至少2位' },
          ]}
        >
          <Input
            clearable
            placeholder='请输入用户名'
            autoComplete='off'
            onBlur={() => {
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
            <div className='text-base'>
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
            autoComplete='off'
            aria-label='密码'
            onEnterPress={handlePasswordEnter}
          />
        </Form.Item>
      </Form>

      <div className={styles.remember_password}>
        <Checkbox checked={rememberPassword} onChange={(val) => setRememberPassword(Boolean(val))}>
          记住密码
        </Checkbox>
      </div>
    </>
  );
}
