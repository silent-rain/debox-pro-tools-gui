import { Button, Form, Input, Radio, Toast, CalendarPickerView, Popup } from 'antd-mobile';
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
  const [calendarVisible, setCalendarVisible] = useState(false);
  const dateBirth = Form.useWatch('date_birth', form);

  const onFinish = async (values: RegisterValue) => {
    if (submitting) return;
    setSubmitting(true);
    try {
      // 基本防御：再次校验两次密码一致
      if (values.password !== values.password2) {
        Toast.show({ icon: 'fail', content: '两次输入的密码不一致' });
        return;
      }

      // 根据出生日期计算年龄（按年）
      const computedAge = (() => {
        const str = values.date_birth;
        if (!str) return undefined;
        const parts = str.split('-');
        if (parts.length !== 3) return undefined;
        const y = Number(parts[0]);
        const m = Number(parts[1]) - 1;
        const d = Number(parts[2]);
        const dob = new Date(y, m, d);
        if (Number.isNaN(dob.getTime())) return undefined;
        const today = new Date();
        let age = today.getFullYear() - dob.getFullYear();
        const hasBirthdayPassed =
          today.getMonth() > dob.getMonth() ||
          (today.getMonth() === dob.getMonth() && today.getDate() >= dob.getDate());
        if (!hasBirthdayPassed) age -= 1;
        return age >= 0 ? age : undefined;
      })();
      values.age = computedAge;

      // TODO: 使用真实注册接口替换此处模拟
      await new Promise((r) => setTimeout(r, 600));

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
              autoComplete='username'
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

          <Form.Item label='出生日期'>
            <Button size='small' onClick={() => setCalendarVisible(true)}>
              {dateBirth || '选择日期（可选）'}
            </Button>
            <Popup
              visible={calendarVisible}
              onMaskClick={() => setCalendarVisible(false)}
              bodyStyle={{ borderRadius: 12, minHeight: '320px' }}
            >
              <div style={{ padding: 12 }}>
                <CalendarPickerView
                  selectionMode='single'
                  // min={new Date(1900, 0, 1)}
                  value={
                    dateBirth
                      ? new Date(
                          Number(dateBirth.split('-')[0]),
                          Number(dateBirth.split('-')[1]) - 1,
                          Number(dateBirth.split('-')[2]),
                        )
                      : null
                  }
                  onChange={(val) => {
                    if (!val) return;
                    const d = val as Date;
                    const yyyy = d.getFullYear();
                    const mm = String(d.getMonth() + 1).padStart(2, '0');
                    const dd = String(d.getDate()).padStart(2, '0');
                    const formatted = `${yyyy}-${mm}-${dd}`;
                    form.setFieldsValue({ date_birth: formatted });
                    setCalendarVisible(false);
                  }}
                />
              </div>
            </Popup>
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
