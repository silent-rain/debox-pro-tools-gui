import { Button, Form, Input, Radio, Toast, Selector, DatePickerRef, DatePicker } from 'antd-mobile';
import { JSX, RefObject, useCallback, useMemo, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import dayjs from 'dayjs';
import styles from './index.module.less';
import { AuthApi } from '@/api';
import { RegisterReq } from '@/typings/auth';
import { UserType } from '@/enums/auth';
import UsernameRegister from './components/UsernameRegister';
import PhoneRegister from './components/PhoneRegister';
import EmailRegister from './components/EmailRegister';
import { getAgeByBirthday } from '@/utils/date';

export default function Register(): JSX.Element {
  const [submitting, setSubmitting] = useState(false);
  const [form] = Form.useForm<RegisterReq>();
  const navigate = useNavigate();
  const [userType, setUserType] = useState<UserType>(UserType.Base);

  const initialValues: RegisterReq = useMemo(
    () => ({
      user_type: userType,
      username: '',
      phone: '',
      email: '',
      password: '',
      password2: '',
      gender: 0,
      date_birth: undefined,
      captcha_id: '',
      captcha: '',
    }),
    [userType],
  );

  const tabs = useMemo(
    () => [
      { key: UserType.Base, label: '用户名' },
      { key: UserType.Phone, label: '手机号' },
      { key: UserType.Email, label: '邮箱' },
    ],
    [],
  );

  // 处理注册方式选择变化（Selector 单选模式）
  const onUserTypeChange = useCallback((arr: string[]) => {
    const first = arr?.[0];
    if (first === undefined || first === null) return;
    const next = Number(first);
    if (!Number.isNaN(next)) {
      setUserType(next as UserType);
    }
  }, []);

  const onFinish = async (values: RegisterReq) => {
    if (submitting) return;
    setSubmitting(true);
    try {
      // 基本防御：再次校验两次密码一致
      if (values.password !== values.password2) {
        Toast.show({ icon: 'fail', content: '两次输入的密码不一致' });
        return;
      }

      // 格式化出生日期为 YYYY-MM-DD 格式
      values.date_birth = values.date_birth ? dayjs(values.date_birth).format('YYYY-MM-DD') : undefined;
      // 根据出生日期计算年龄（按年）
      values.age = getAgeByBirthday(values.date_birth!);

      await AuthApi.register(values);
      console.log('注册成功: ', values);

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
        <div className={styles.subtitle}>请选择注册方式，并请填写以下信息</div>

        {/* 登录方式切换 - 使用 Selector 单选模式 */}
        <div>
          <Selector
            className={styles.user_type}
            options={tabs.map((t) => ({ label: t.label, value: String(t.key) }))}
            value={[String(userType)]}
            onChange={onUserTypeChange}
          />
        </div>

        <Form
          form={form}
          layout='horizontal'
          mode='card'
          initialValues={initialValues}
          onFinish={onFinish}
          footer={
            <div className={styles.actions}>
              <Button block color='primary' type='submit' disabled={submitting} loading={submitting}>
                {submitting ? '注册中...' : '注册'}
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

          {userType === UserType.Base && <UsernameRegister />}
          {userType === UserType.Phone && <PhoneRegister />}
          {userType === UserType.Email && <EmailRegister />}

          <Form.Item
            name='password'
            label='密码'
            rules={[
              { required: true, message: '请输入密码' },
              { min: 6, message: '密码至少6位' },
            ]}
          >
            <Input
              type='password'
              placeholder='请输入密码'
              clearable
              maxLength={64}
              aria-label='密码'
              autoComplete='new-password'
            />
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
            <Input
              type='password'
              placeholder='请再次输入密码'
              clearable
              maxLength={64}
              aria-label='确认密码'
              autoComplete='new-password'
            />
          </Form.Item>

          <Form.Item name='date_birth' style={{ display: 'none' }}>
            <Input aria-hidden='true' />
          </Form.Item>

          <Form.Item name='gender' label='性别'>
            <Radio.Group defaultValue={0} onChange={(val) => form.setFieldsValue({ gender: val as 0 | 1 | 2 })}>
              <Radio value={0}>保密</Radio>
              <Radio value={1}>女</Radio>
              <Radio value={2}>男</Radio>
            </Radio.Group>
          </Form.Item>

          <Form.Item
            name='date_birth'
            label='出生日期'
            trigger='onConfirm'
            onClick={(_e, datePickerRef: RefObject<DatePickerRef>) => {
              datePickerRef.current?.open();
            }}
          >
            <DatePicker>
              {(value) => {
                return value ? dayjs(value).format('YYYY-MM-DD') : '请选择日期';
              }}
            </DatePicker>
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
