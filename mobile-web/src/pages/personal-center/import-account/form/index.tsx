import { useNavigate } from 'react-router-dom';
import { Button, Form, Input, TextArea, Switch, Toast } from 'antd-mobile';
import { ROUTES } from '@/constants/routes';
import { DeboxAccountApi } from '@/api/debox-account';
import { useAuthStore } from '@/stores';
import { CreateDeboxAccountReq } from '@/typings/debox-account';
import './index.module.less';

const { Item } = Form;

const AddAccountForm = () => {
  const navigate = useNavigate();
  const authStore = useAuthStore.getState();
  const [form] = Form.useForm();

  const handleSubmit = async (values: CreateDeboxAccountReq) => {
    // Handle form submission logic here
    console.log('Form values:', values);

    values.user_id = authStore.user_id!;
    await DeboxAccountApi.create(values);

    Toast.show({
      icon: 'success',
      content: '添加成功',
    });

    form.resetFields();
    navigate(ROUTES.PERSONAL_CENTER_IMPORT_ACCOUNT, { replace: true });
  };

  return (
    <div className='add-account-form'>
      <Form
        form={form}
        onFinish={handleSubmit}
        footer={
          <Button block type='submit' color='primary' size='large'>
            提交
          </Button>
        }
      >
        {/* <Item name='user_id' label='用户ID' rules={[{ required: false }]}>
          <Input placeholder='请输入用户ID' type='number' />
        </Item> */}
        <Item name='app_id' label='App Id' rules={[{ required: true }]}>
          <Input placeholder=' 请输入 AppId，在DeBox开放平台获取' />
        </Item>
        <Item name='api_key' label='API Key' rules={[{ required: true }]}>
          <Input placeholder=' 请输入 API Key，在DeBox开放平台获取' />
        </Item>
        <Item name='app_secret' label='App Secret' rules={[{ required: true }]}>
          <Input placeholder='请输入 App Secret，在DeBox开放平台获取' />
        </Item>
        <Item name='access_token' label='Access登录授权' rules={[{ required: false }]}>
          <Input placeholder='请输入登录授权' />
        </Item>
        <Item name='web_token' label='WEB登录授权' rules={[{ required: true }]}>
          <Input placeholder='请输入WEB登录授权' />
        </Item>
        <Item name='debox_user_id' label='DeBox用户ID' rules={[{ required: true }]}>
          <Input placeholder='请输入DeBox用户ID' />
        </Item>
        {/* <Item name='wallet_address' label='钱包地址' rules={[{ required: true }]}>
          <Input placeholder='请输入钱包地址' />
        </Item>
        <Item name='api_key_status' label='API Key状态' rules={[{ required: true }]}>
          <Selector options={statusOptions} />
        </Item>
        <Item name='access_token_status' label='Access Token状态' rules={[{ required: true }]}>
          <Selector options={statusOptions} />
        </Item>
        <Item name='web_token_status' label='Web Token状态' rules={[{ required: true }]}>
          <Selector options={statusOptions} />
        </Item> */}
        <Item name='desc' label='描述信息'>
          <TextArea placeholder='请输入描述信息' />
        </Item>
        <Item name='status' label='启用状态' initialValue={true}>
          <Switch />
        </Item>
      </Form>
    </div>
  );
};

export default AddAccountForm;
