import { Form, Input } from 'antd-mobile';
import { JSX } from 'react';

export default function EmailRegister(): JSX.Element {
  return (
    <>
      <Form.Item
        name='email'
        label='邮箱'
        rules={[
          { required: true, message: '请输入邮箱' },
          { type: 'email', message: '请输入正确的邮箱格式' },
        ]}
      >
        <Input clearable placeholder='请输入邮箱' aria-label='邮箱' autoComplete='email' />
      </Form.Item>
    </>
  );
}
