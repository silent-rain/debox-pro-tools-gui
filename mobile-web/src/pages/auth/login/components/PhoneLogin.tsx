import { JSX, useCallback, useMemo, useState } from 'react';
import { Form, Input, Button, Checkbox } from 'antd-mobile';
import { EyeInvisibleOutline, EyeOutline } from 'antd-mobile-icons';
import { LoginReq } from '@/typings/auth';
import { UserType } from '@/enums/auth';
import { cachedPasswordPhoneKey, cachedPhoneKey } from '@/constant/auth';
import styles from './index.module.less';

export interface PhoneLoginProps {
  submitting: boolean;
  onSubmit: (payload: LoginReq) => Promise<void> | void;
}

export default function PhoneLogin({ submitting, onSubmit }: PhoneLoginProps): JSX.Element {
  const [form] = Form.useForm<LoginReq>();
  const [passwordVisible, setPasswordVisible] = useState(false);

  // 只读取一次本地缓存，避免在 effect 中同步 setState 造成级联渲染
  const initialCache = useMemo(() => {
    const username = localStorage.getItem(cachedPhoneKey) || '';
    const pwd = localStorage.getItem(cachedPasswordPhoneKey) || '';
    return { username, pwd };
  }, []);
  const [rememberPassword, setRememberPassword] = useState<boolean>(() =>
    Boolean(initialCache.username || initialCache.pwd),
  );

  const initialValues: LoginReq = useMemo(
    () => ({
      user_type: UserType.Phone,
      phone: '',
      password: '',
      captcha_id: '',
      captcha: '',
    }),
    [],
  );

  const handlePasswordEnter = useCallback(() => {
    form.submit();
  }, [form]);

  const handleFinish = useCallback(
    async (values: LoginReq) => {
      if (rememberPassword) {
        localStorage.setItem(cachedPhoneKey, values.phone || '');
        localStorage.setItem(cachedPasswordPhoneKey, values.password);
      } else {
        localStorage.removeItem(cachedPhoneKey);
        localStorage.removeItem(cachedPasswordPhoneKey);
      }
      try {
        await onSubmit(values);
      } catch (e) {
        // 忽略本地存储异常，避免阻断登录
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
          name='phone'
          label='手机号'
          rules={[
            { required: true, message: '请输入手机号' },
            { pattern: /^\d{6,20}$/g, message: '请输入正确的手机号' },
          ]}
        >
          <Input
            clearable
            placeholder='请输入手机号'
            autoComplete='phone'
            onBlur={() => {
              const val = (form.getFieldValue('phone') || '').toString();
              const trimmed = val.trim();
              if (trimmed !== val) form.setFieldsValue({ phone: trimmed });
            }}
            aria-label='手机号'
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
            autoComplete='current-password'
            onEnterPress={handlePasswordEnter}
            aria-label='密码'
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
