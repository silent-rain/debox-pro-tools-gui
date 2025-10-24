import { Button, Form, Input, Radio, Toast } from 'antd-mobile';
import { JSX, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import styles from './index.module.less';

interface RegisterValue {
  username: string;
  gender: 0 | 1 | 2; // 0:保密,1:女,2:男
  password: string;
  password2: string;
  age?: number;
  date_birth?: string; // 格式: YYYY-MM-DD
}

export default function Register(): JSX.Element {
  const [submitting, setSubmitting] = useState(false);
  const [form] = Form.useForm<RegisterValue>();
  const navigate = useNavigate();

  const onFinish = async (values: RegisterValue) => {
    if (submitting) return;
    setSubmitting(true);
    try {
      // 基本防御：再次校验两次密码一致
      if (values.password !== values.password2) {
        Toast.show({ icon: 'fail', content: '两次输入的密码不一致' });
        return;
      }

      // TODO: 使用真实注册接口替换此处模拟
      await new Promise((r) => setTimeout(r, 600));

      Toast.show({ icon: 'success', content: '注册成功' });
      // 注册成功后跳转到登录页或首页
      navigate('/login', { replace: true });
    } catch (e) {
      console.log('注册失败, err:', e);
      Toast.show({ icon: 'fail', content: '注册失败，请稍后重试' });
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className={styles.register_container}>
      <div className={styles.card}>
        <h1 className={styles.title}>用户注册</h1>
        <div className={styles.subtitle}>请填写以下信息</div>

        <Form
          form={form}
          layout='horizontal'
          mode='card'
          onFinish={onFinish}
          footer={
            <div className={styles.actions}>
              <Button block color='primary' type='submit' disabled={submitting} loading={submitting}>
                {submitting ? '注册中...' : '注册'}
              </Button>
            </div>
          }
        >
          <Form.Item
            name='username'
            label='用户名'
            rules={[
              { required: true, message: '请输入用户名' },
              { min: 3, message: '用户名至少3位' },
            ]}
          >
            <Input
              clearable
              placeholder='请输入用户名'
              onBlur={() => {
                const val = (form.getFieldValue('username') || '').toString();
                const trimmed = val.trim();
                if (trimmed !== val) form.setFieldsValue({ username: trimmed });
              }}
              aria-label='用户名'
            />
          </Form.Item>

          <Form.Item name='gender' label='性别' rules={[{ required: true, message: '请选择性别' }]}>
            <Radio.Group defaultValue={0} onChange={(val) => form.setFieldsValue({ gender: val as 0 | 1 | 2 })}>
              <Radio value={0}>保密</Radio>
              <Radio value={1}>女</Radio>
              <Radio value={2}>男</Radio>
            </Radio.Group>
          </Form.Item>

          <Form.Item
            name='password'
            label='密码'
            rules={[
              { required: true, message: '请输入密码' },
              { min: 6, message: '密码至少6位' },
            ]}
          >
            <Input type='password' placeholder='请输入密码' clearable maxLength={64} aria-label='密码' />
          </Form.Item>

          <Form.Item
            name='password2'
            label='确认密码'
            rules={[
              { required: true, message: '请再次输入密码' },
              { min: 6, message: '密码至少6位' },
              {
                validator: (_, value) => {
                  const pwd = form.getFieldValue('password');
                  if (!value || pwd === value) return Promise.resolve();
                  return Promise.reject(new Error('两次输入的密码不一致'));
                },
              },
            ]}
          >
            <Input type='password' placeholder='请再次输入密码' clearable maxLength={64} aria-label='确认密码' />
          </Form.Item>

          <Form.Item name='age' label='年龄'>
            <Input type='number' placeholder='可选' clearable maxLength={3} aria-label='年龄' />
          </Form.Item>

          <Form.Item name='date_birth' label='出生日期'>
            <Input placeholder='YYYY-MM-DD（可选）' clearable aria-label='出生日期' />
          </Form.Item>
        </Form>

        <div className={styles.tips}>
          <Button size='small' fill='none' onClick={() => navigate('/login')}>
            已有账号？去登录
          </Button>
        </div>
      </div>
    </div>
  );
}
