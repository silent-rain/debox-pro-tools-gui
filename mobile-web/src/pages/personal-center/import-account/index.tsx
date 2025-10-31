import { Avatar, Button, List, ActionSheet, Tag, Modal, ErrorBlock, DotLoading } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import { AddOutline, MoreOutline } from 'antd-mobile-icons';
import { Action } from 'antd-mobile/es/components/action-sheet';
import { useState, useRef, useEffect } from 'react';
import { DeboxAccountApi } from '@/api/debox-account';
import { DeboxAccount, GetDeboxAccountsReq } from '@/typings/debox-account';
import { useAuthStore } from '@/stores';
import { ROUTES } from '@/constants/routes';
import { saveAs } from 'file-saver';
import './index.module.less';

// 获取账号列表
const fetchAccounts = async (userId: number): Promise<DeboxAccount[]> => {
  const data: GetDeboxAccountsReq = {
    page: 0,
    page_size: 0,
    user_id: userId,
    all: true,
  };
  const response = await DeboxAccountApi.list(data);
  return response.data_list;
};

// 更新账号信息
const updateAccountInfo = async (accountId: number) => {
  const data = await DeboxAccountApi.updateAccountInfo({
    id: accountId,
  });
  return data;
};

// 删除账号
const deleteAccount = async (accountId: number) => {
  const data = await DeboxAccountApi.delete({
    id: accountId,
  });
  return data;
};

// 下载配置文件
const downloadConfigFile = async (accountId: number) => {
  const response = await DeboxAccountApi.info({
    id: accountId,
  });

  const data = {
    app_id: response.app_id,
    api_key: response.api_key,
    app_secret: response.app_secret,
    access_token: response.access_token,
    web_token: response.web_token,
    debox_user_id: response.debox_user_id,
  };

  const filename = response.name !== '' ? response.name : response.debox_user_id;
  const blob = new Blob([JSON.stringify(data)], { type: 'application/json' });

  saveAs(blob, filename); // 自动处理下载逻辑
  return;
};

// 用户列表
const AccountList = () => {
  const navigate = useNavigate();
  const authStore = useAuthStore();

  const [visible, setVisible] = useState(false);
  const [currentAccountId, setCurrentAccountId] = useState<number | null>(null);

  const [accounts, setAccounts] = useState<DeboxAccount[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setLoading(true);
        const data = await fetchAccounts(authStore.user_id!);
        setAccounts(data);
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [authStore.user_id]);

  const actions: Action[] = [
    { text: '更新', key: 'update', onClick: () => handleMenuAction('update', currentAccountId!) },
    { text: '编辑', key: 'edit', onClick: () => handleMenuAction('edit', currentAccountId!) },
    { text: '删除', key: 'delete', onClick: () => handleMenuAction('delete', currentAccountId!) },
    { text: '导出配置', key: 'export', onClick: () => handleMenuAction('export', currentAccountId!) },
  ];

  const handleMenuAction = async (action: string, accountId: number) => {
    switch (action) {
      case 'update':
        try {
          setLoading(true);
          // 更新账号信息
          await updateAccountInfo(accountId);
          // 重新获取账号列表
          const data = await fetchAccounts(authStore.user_id!);
          setAccounts(data);
        } catch (err) {
          console.error(err);
        } finally {
          setLoading(false);
        }
        break;
      case 'edit':
        navigate(ROUTES.PERSONAL_CENTER_IMPORT_ACCOUNT_FORM, { state: { mode: 'edit', accountId } });
        break;
      case 'delete':
        try {
          setLoading(true);
          // 删除账号
          await deleteAccount(accountId);
          // 重新获取账号列表
          const data = await fetchAccounts(authStore.user_id!);
          setAccounts(data);
        } catch (err) {
          console.error(err);
        } finally {
          setLoading(false);
        }
        break;
      case 'export':
        try {
          // 下载配置
          await downloadConfigFile(accountId);
        } catch (err) {
          console.error(err);
        }
        break;
      default:
        break;
    }
    setVisible(false);
  };

  // 账号状态
  const accountStatus = (account: DeboxAccount) => {
    if (!account.status) {
      return (
        <Tag color='default' style={{ marginLeft: '8px' }}>
          禁用
        </Tag>
      );
    }

    const apiKeyStatus = !account.api_key_status ? (
      <Tag color='danger' style={{ marginLeft: '8px' }}>
        Api Key
      </Tag>
    ) : null;

    const accessTokenStatus = !account.access_token_status ? (
      <Tag color='warning' style={{ marginLeft: '8px' }} aria-hidden='true'>
        Access Token
      </Tag>
    ) : null;

    const webTokenStatus = !account.web_token_status ? (
      <Tag color='danger' style={{ marginLeft: '8px' }}>
        Web Token
      </Tag>
    ) : null;

    return (
      <>
        {apiKeyStatus}
        {accessTokenStatus}
        {webTokenStatus}
      </>
    );
  };

  if (loading) {
    return <DotLoading color='primary' />;
  }

  if (accounts.length === 0) {
    return <ErrorBlock status='empty' />;
  }

  return (
    <div className='account-list'>
      <List>
        {accounts.map((account) => (
          <List.Item
            key={account.id}
            prefix={<Avatar src='https://example.com/avatar.png' style={{ '--size': '32px' }} />}
            extra={
              <Button
                fill='none'
                onClick={() => {
                  setCurrentAccountId(account.id);
                  setVisible(true);
                }}
              >
                <MoreOutline />
              </Button>
            }
          >
            {account.name}
            {accountStatus(account)}
          </List.Item>
        ))}
      </List>

      <ActionSheet
        visible={visible}
        actions={actions}
        style={{
          marginBottom: '0px',
        }}
        onClose={() => setVisible(false)}
      />
    </div>
  );
};

const ImportAccount = () => {
  const navigate = useNavigate();
  const [modalVisible, setModalVisible] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleAddAccount = () => {
    setModalVisible(true);
  };

  const handleFormImport = () => {
    navigate(ROUTES.PERSONAL_CENTER_IMPORT_ACCOUNT_FORM, { state: { mode: 'add' } });
  };

  const handleFileImport = () => {
    fileInputRef.current?.click();
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      // Handle file import logic here
      alert(`File ${file.name} selected for import!`);
    }
  };

  return (
    <div className='import-account'>
      <div className='import-account-box' style={{ display: 'flex', justifyContent: 'flex-end', alignItems: 'center' }}>
        <Button fill='none' onClick={handleAddAccount}>
          <AddOutline style={{ fontSize: '24px' }} />
        </Button>
      </div>

      <Modal
        visible={modalVisible}
        title='选择导入方式'
        closeOnMaskClick={true}
        onClose={() => setModalVisible(false)}
        actions={[
          { key: 'form', text: '表单填写', onClick: handleFormImport },
          { key: 'file', text: '从文件导入', onClick: handleFileImport },
        ]}
      />

      <input
        type='file'
        ref={fileInputRef}
        style={{ display: 'none' }}
        accept='.xlsx, .xls'
        onChange={handleFileChange}
      />

      <AccountList />
    </div>
  );
};

export default ImportAccount;
