import { Form, Input } from 'antd-mobile';
import { JSX } from 'react';

export default function PhoneRegister(): JSX.Element {
  return (
    <>
      <Form.Item
        name='phone'
        label='手机号码'
        rules={[
          { required: true, message: '请输入手机号码' },
          { pattern: /^1[3-9]\d{9}$/, message: '请输入正确的手机号码' },
        ]}
      >
        <Input clearable placeholder='请输入手机号码' aria-label='手机号码' autoComplete='tel' />
      </Form.Item>
    </>
  );
}
