import { Avatar, Button, List, ActionSheet, Tag, Modal } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import { AddOutline, MoreOutline } from 'antd-mobile-icons';
import './index.module.less';
import { Action } from 'antd-mobile/es/components/action-sheet';
import { useState, useRef } from 'react';

// 用户列表
const AccountList = () => {
  const navigate = useNavigate();

  // Mock data for account list
  const accountList = [
    { id: 1, name: 'Account 1', status: 'Active' },
    { id: 2, name: 'Account 2', status: 'Inactive' },
  ];

  const [visible, setVisible] = useState(false);
  const [currentAccountId, setCurrentAccountId] = useState<number | null>(null);

  const actions: Action[] = [
    { text: '更新', key: 'update', onClick: () => handleMenuAction('update', currentAccountId!) },
    { text: '编辑', key: 'edit', onClick: () => handleMenuAction('edit', currentAccountId!) },
    { text: '复制', key: 'copy', onClick: () => handleMenuAction('copy', currentAccountId!) },
    { text: '删除', key: 'delete', onClick: () => handleMenuAction('delete', currentAccountId!) },
  ];

  const handleMenuAction = (action: string, accountId: number) => {
    switch (action) {
      case 'update':
        // Simulate updating account data
        alert(`Account ${accountId} updated successfully!`);
        break;
      case 'edit':
        navigate('/personal-center/import-account/form', { state: { mode: 'edit', accountId } });
        break;
      case 'copy':
        // Simulate copying account data
        alert(`Account ${accountId} copied to clipboard!`);
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
            <Tag color={account.status === 'Active' ? 'success' : 'default'} style={{ marginLeft: '8px' }}>
              {account.status}
            </Tag>
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
    navigate('/personal-center/import-account/form', { state: { mode: 'add' } });
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
