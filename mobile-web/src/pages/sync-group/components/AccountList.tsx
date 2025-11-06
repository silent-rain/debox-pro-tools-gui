import { Checkbox, Space, Avatar, Dropdown, DotLoading } from 'antd-mobile';
import { FC, useCallback, useEffect, useState } from 'react';
import { DeboxAccount, GetDeboxAccountsReq } from '@/typings/debox-account';
import { DeboxAccountApi } from '@/api/debox-account';
import styles from './AccountList.module.less';

interface AccountListProps {
  selectedAccounts: number[];
  onAccountChange: (selected: number[]) => void;
}

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

const AccountList: FC<AccountListProps> = ({ selectedAccounts, onAccountChange }) => {
  const [accounts, setAccounts] = useState<DeboxAccount[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setLoading(true);
        const data = await fetchAccounts();
        setAccounts(data);

        if (data.length > 0) {
          onAccountChange([data[0].id]);
        }
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [onAccountChange]);

  const handleSelectAll = useCallback(
    (checked: boolean) => {
      if (checked) {
        onAccountChange(accounts.map((account) => account.id));
      } else {
        onAccountChange([]);
      }
    },
    [accounts, onAccountChange],
  );

  if (loading) {
    return <DotLoading color='primary' />;
  }

  return (
    <Dropdown>
      <Dropdown.Item key='sorter' title='选择用户'>
        <div className={styles.allAccountsCheckbox}>
          <Checkbox
            indeterminate={selectedAccounts.length > 0 && selectedAccounts.length < accounts.length}
            checked={selectedAccounts.length === accounts.length}
            onChange={handleSelectAll}
          >
            全选
          </Checkbox>
        </div>

        <div className={styles.accountListDivider}></div>

        <Checkbox.Group value={selectedAccounts} onChange={(values) => onAccountChange(values as number[])}>
          <Space className={styles.accountList} direction='vertical'>
            {accounts.map((account) => (
              <Checkbox key={account.id} value={account.id}>
                <Space className={styles.accountItem} align='center'>
                  <Avatar className={styles.accountItemAvatar} src={account.avatar ?? ''} />
                  <span>{account.name}</span>
                </Space>
              </Checkbox>
            ))}
          </Space>
        </Checkbox.Group>
      </Dropdown.Item>
    </Dropdown>
  );
};

export default AccountList;
