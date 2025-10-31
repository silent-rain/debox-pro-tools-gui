import { Avatar, Button, List, ActionSheet, Tag, Modal, ErrorBlock, DotLoading } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import { AddOutline, MoreOutline } from 'antd-mobile-icons';
import { Action } from 'antd-mobile/es/components/action-sheet';
import { useState, useRef, useEffect } from 'react';
import { DeboxAccountApi } from '@/api/debox-account';
import { DeboxAccount, GetDeboxAccountsReq } from '@/typings/debox-account';
import { useAuthStore } from '@/stores';
import { ROUTES } from '@/constants/routes';
import './index.module.less';

// 用户列表
const AccountList = () => {
  const navigate = useNavigate();
  const authStore = useAuthStore();

  const [visible, setVisible] = useState(false);
  const [currentAccountId, setCurrentAccountId] = useState<number | null>(null);

  const [accountList, setAccountList] = useState<DeboxAccount[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const fetchAccounts = async () => {
      setLoading(true);
      try {
        const data: GetDeboxAccountsReq = {
          page: 0,
          page_size: 0,
          user_id: authStore.user_id!,
          all: true,
        };
        const response = await DeboxAccountApi.list(data);
        setLoading(false);
        setAccountList(response.data_list);
      } catch (err) {
        console.error('Failed to fetch accounts:', err);
      } finally {
        setLoading(false);
      }
    };

    fetchAccounts();
  }, [authStore.user_id]);

  const actions: Action[] = [
    { text: '更新', key: 'update', onClick: () => handleMenuAction('update', currentAccountId!) },
    { text: '编辑', key: 'edit', onClick: () => handleMenuAction('edit', currentAccountId!) },
    { text: '删除', key: 'delete', onClick: () => handleMenuAction('delete', currentAccountId!) },
  ];

  const handleMenuAction = (action: string, accountId: number) => {
    switch (action) {
      case 'update':
        // Simulate updating account data
        alert(`Account ${accountId} updated successfully!`);
        break;
      case 'edit':
        navigate(ROUTES.PERSONAL_CENTER_IMPORT_ACCOUNT_FORM, { state: { mode: 'edit', accountId } });
        break;
      case 'delete':
        // Handle delete logic
        break;
      case 'save':
        // Handle save logic
        break;
      default:
        break;
    }
    setVisible(false);
  };

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

  if (accountList.length === 0) {
    return <ErrorBlock status='empty' />;
  }

  return (
    <div className='account-list'>
      <List>
        {accountList.map((account) => (
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
          marginBottom: '20px',
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
