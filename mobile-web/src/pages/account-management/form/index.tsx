import { useMemo, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { Button, Form, Input, TextArea, Switch, Toast } from 'antd-mobile';
import { ROUTES } from '@/constants/routes';
import { DeboxAccountApi } from '@/api/debox-account';
import { DeboxAccount } from '@/typings/debox-account';
import './index.module.scss';

const { Item } = Form;

const AccountForm = () => {
  const navigate = useNavigate();
  const [form] = Form.useForm();
  const location = useLocation();
  const [isEditMode, setIsEditMode] = useState(false);

  useMemo(() => {
    const { mode, accountId } = location.state || {};

    const fetchAccountData = async (id: string) => {
      try {
        const data = await DeboxAccountApi.info({ id: Number(id) });
        form.setFieldsValue(data);
      } catch (error) {
        console.error('获取账号数据失败, err: ', error);
        Toast.show({
          icon: 'fail',
          content: '获取账号数据失败',
        });
      }
    };

    if (mode === 'edit' && accountId) {
      setIsEditMode(true);
      fetchAccountData(accountId);
    }
  }, [form, location.state]);

  const handleSubmit = async (values: DeboxAccount) => {
    try {
      if (isEditMode) {
        await DeboxAccountApi.update(values);
        Toast.show({
          icon: 'success',
          content: '更新成功',
        });
      } else {
        await DeboxAccountApi.create(values);
        Toast.show({
          icon: 'success',
          content: '添加成功',
        });
      }
      form.resetFields();
      navigate(ROUTES.ACCOUNT_MANAGEMENT, { replace: true });
    } catch (error) {
      console.error('添加账号失败, err: ', error);
      Toast.show({
        icon: 'fail',
        content: '操作失败',
      });
    }
  };

  return (
    <div className='account-form'>
      <Form
        form={form}
        onFinish={handleSubmit}
        footer={
          <Button block type='submit' color='primary' size='large'>
            提交
          </Button>
        }
      >
        <Form.Item name='id' hidden>
          <Input type='hidden' />
        </Form.Item>

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

        <Item name='wallet_address' label='钱包地址' disabled>
          <Input placeholder='请输入钱包地址' />
        </Item>

        {/* 
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
        <Item name='status' label='启用状态' valuePropName='checked'>
          <Switch />
        </Item>
      </Form>
    </div>
  );
};

export default AccountForm;
