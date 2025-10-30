import { Form, Input } from 'antd-mobile';
import { JSX } from 'react';

export default function UsernameRegister(): JSX.Element {
  return (
    <>
      <Form.Item
        name='username'
        label='用户名'
        rules={[
          { required: true, message: '请输入用户名' },
          { min: 3, message: '用户名至少3位' },
        ]}
      >
        <Input clearable placeholder='请输入用户名' aria-label='用户名' autoComplete='username' />
      </Form.Item>
    </>
  );
}
