import { Avatar, Button, List, ActionSheet, Tag, Modal, DotLoading } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import { AddOutline, DownlandOutline, MoreOutline } from 'antd-mobile-icons';
import { Action } from 'antd-mobile/es/components/action-sheet';
import { useState, useRef, useEffect } from 'react';
import { saveAs } from 'file-saver';
import { DeboxAccountApi } from '@/api/debox-account';
import { DeboxAccount, GetDeboxAccountsReq } from '@/typings/debox-account';
import { useAuthStore } from '@/stores';
import { ROUTES } from '@/constants/routes';
import Empty from '@/components/empty';
import styles from './index.module.scss';
import { BaseDirectory, writeFile } from '@tauri-apps/plugin-fs';
import { platform } from '@tauri-apps/plugin-os';

// 获取账号列表
const fetchAccounts = async (): Promise<DeboxAccount[]> => {
  const data: GetDeboxAccountsReq = {
    page: 0,
    page_size: 0,
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

// 在 Tauri 环境中保存文件
const saveFileTauri = async (filename: string, content: string) => {
  try {
    const currentPlatform = platform();
    console.log(currentPlatform);

    // 在 Android 上，使用保存对话框让用户选择位置
    if (currentPlatform === 'android' || currentPlatform === 'ios') {
      // 动态导入保存对话框
      const { save } = await import('@tauri-apps/plugin-dialog');

      // 使用保存对话框
      const filePath = await save({
        filters: [
          {
            name: 'JSON文件',
            extensions: ['json'],
          },
        ],
        defaultPath: filename,
      });

      // const filePath = filename;

      if (filePath) {
        // 将字符串内容转换为Uint8Array
        const data = new TextEncoder().encode(content);
        await writeFile(filePath, data, { baseDir: BaseDirectory.Download });
        console.log('文件写入成功到用户选择路径');
      } else {
        console.log('用户取消了保存操作');
        return; // 用户取消保存
      }
    } else {
      // 网页环境使用原有逻辑
      // 其他平台使用原有逻辑
      const blob = new Blob([content], { type: 'application/json' });
      saveAs(blob, filename);
    }

    Modal.show({
      content: '导出成功',
      closeOnMaskClick: true,
    });
  } catch (error) {
    console.error('Tauri 保存文件失败:', error);
    throw error;
  }
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
  const content = JSON.stringify(data);

  await saveFileTauri(`${filename}.json`, content);

  return;
};

// 导出全部配置
const exportAllConfigFiles = async () => {
  const data: GetDeboxAccountsReq = {
    page: 0,
    page_size: 0,
    all: true,
  };
  const response = await DeboxAccountApi.list(data);
  const accounts = response.data_list;

  const filename = 'configs.json';
  const content = JSON.stringify(accounts);

  await saveFileTauri(filename, content);
};

// 用户列表
const AccountList = ({ accountsUpdateState }: { accountsUpdateState: number }) => {
  const navigate = useNavigate();

  const [visible, setVisible] = useState(false);
  const [currentAccountId, setCurrentAccountId] = useState<number | null>(null);

  const [accounts, setAccounts] = useState<DeboxAccount[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setLoading(true);
        const data = await fetchAccounts();
        setAccounts(data);
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [accountsUpdateState]);

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
          const data = await fetchAccounts();
          setAccounts(data);
        } catch (err) {
          console.error(err);
        } finally {
          setLoading(false);
        }
        break;
      case 'edit':
        navigate(ROUTES.ACCOUNT_MANAGEMENT_FORM, { state: { mode: 'edit', accountId } });
        break;
      case 'delete':
        try {
          setLoading(true);
          // 删除账号
          await deleteAccount(accountId);
          // 重新获取账号列表
          const data = await fetchAccounts();
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
        <Tag className={styles.accountStatus} color='default'>
          禁用
        </Tag>
      );
    }

    const apiKeyStatus = !account.api_key_status ? (
      <Tag className={styles.accountStatus} color='danger'>
        Api Key
      </Tag>
    ) : null;

    const accessTokenStatus = !account.access_token_status ? (
      <Tag className={styles.accountStatus} color='warning' aria-hidden='true'>
        Access Token
      </Tag>
    ) : null;

    const webTokenStatus = !account.web_token_status ? (
      <Tag className={styles.accountStatus} color='danger'>
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
    return <Empty />;
  }

  return (
    <div className='account-list'>
      <List>
        {accounts.map((account) => (
          <List.Item
            key={account.id}
            prefix={<Avatar className={styles.accountAvatar} src={account.avatar} />}
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

      <ActionSheet visible={visible} actions={actions} onClose={() => setVisible(false)} />
    </div>
  );
};

const ImportAccount = () => {
  const navigate = useNavigate();
  const authStore = useAuthStore();
  const [modalVisible, setModalVisible] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const mutifileInputRef = useRef<HTMLInputElement>(null);
  const [accountsUpdateState, setAccountsUpdateState] = useState<number>(0);

  const handleAddAccount = () => {
    setModalVisible(true);
  };

  const handleFormImport = () => {
    navigate(ROUTES.ACCOUNT_MANAGEMENT_FORM, { state: { replace: true, mode: 'add' } });
  };

  const handleFileImport = () => {
    fileInputRef.current?.click();
  };

  // 上传单个文件
  const handleSingleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) {
      console.error('请选择文件');
      Modal.show({
        content: '请选择文件',
        closeOnMaskClick: true,
      });
      return;
    }

    try {
      await DeboxAccountApi.uploadConfigFile(file, String(authStore.user_id));
      handleRefreshAccounts();
      navigate(ROUTES.ACCOUNT_MANAGEMENT, { replace: true });
    } catch (error) {
      console.error('上传失败:', error);
    }
  };

  const handleMutiFileImport = () => {
    mutifileInputRef.current?.click();
  };

  // 批量上传文件
  const handleMultipleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) {
      console.error('请选择文件');
      Modal.show({
        content: '请选择文件',
        closeOnMaskClick: true,
      });
      return;
    }

    try {
      await DeboxAccountApi.uploadConfigsFile(file, String(authStore.user_id));
      handleRefreshAccounts();
      navigate(ROUTES.ACCOUNT_MANAGEMENT, { replace: true });
    } catch (error) {
      console.error('上传失败:', error);
      Modal.show({
        content: '上传失败，请重试',
        closeOnMaskClick: true,
      });
    }
  };

  // 刷新群组列表
  const handleRefreshAccounts = () => {
    setAccountsUpdateState((prev) => prev + 1);
  };

  return (
    <div className='account-management'>
      <div className={styles.accountHeader}>
        <Button color='primary' size='small' fill='solid' onClick={handleRefreshAccounts}>
          刷新
        </Button>
        <Button fill='none' onClick={exportAllConfigFiles}>
          <DownlandOutline className={styles.addAccount} />
        </Button>
        <Button fill='none' onClick={handleAddAccount}>
          <AddOutline className={styles.addAccount} />
        </Button>
      </div>

      <Modal
        visible={modalVisible}
        title='添加账号'
        closeOnMaskClick={true}
        onClose={() => setModalVisible(false)}
        actions={[
          { key: 'form', text: '表单填写', onClick: handleFormImport },
          { key: 'file', text: '导入账号', onClick: handleFileImport },
          { key: 'mutifile', text: '批量导入账号', onClick: handleMutiFileImport },
        ]}
      />

      <input
        className={styles.importAccount}
        type='file'
        ref={fileInputRef}
        accept='.json'
        onChange={handleSingleFileChange}
      />

      <input
        className={styles.importAccount}
        type='file'
        ref={mutifileInputRef}
        accept='.json'
        onChange={handleMultipleFileChange}
      />

      {/* 账号列表 */}
      <AccountList accountsUpdateState={accountsUpdateState} />
    </div>
  );
};

export default ImportAccount;
