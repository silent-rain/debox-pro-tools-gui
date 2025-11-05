import { Checkbox, Space, Avatar, Button } from 'antd-mobile';
import { FC, useCallback } from 'react';
import styles from './index.module.less';

interface AccountListProps {
  accounts: Array<{ label: string; value: string; avatar?: string }>;
  selectedAccounts: string[];
  onAccountChange: (selected: string[]) => void;
}

const AccountList: FC<AccountListProps> = ({ accounts, selectedAccounts, onAccountChange }) => {
  const handleSelectAll = useCallback(
    (checked: boolean) => {
      if (checked) {
        onAccountChange(accounts.map((account) => account.value));
      } else {
        onAccountChange([]);
      }
    },
    [accounts, onAccountChange],
  );

  return (
    <Space direction='vertical' block>
      <h2 className={styles.accountsTitle}>选择同步账号</h2>
      <div className={styles.allAccountsCheckbox}>
        <Checkbox
          indeterminate={selectedAccounts.length > 0 && selectedAccounts.length < accounts.length}
          checked={selectedAccounts.length === accounts.length}
          onChange={handleSelectAll}
        >
          全选
        </Checkbox>

        {selectedAccounts.length > 0 && (
          <div className={styles.selectedAccountCount}>
            <span>已选</span>
            <span>{selectedAccounts.length}个账号</span>
          </div>
        )}

        <div>
          {selectedAccounts.length > 0 && (
            <Button color='primary' size='small' fill='solid' onClick={() => onAccountChange([])}>
              同步
            </Button>
          )}
        </div>
      </div>

      <div className={styles.accountListDivider}></div>

      <Checkbox.Group value={selectedAccounts} onChange={(values) => onAccountChange(values as string[])}>
        <Space direction='vertical'>
          {accounts.map((account) => (
            <Checkbox key={account.value} value={account.value}>
              <Space className={styles.accountItem} align='center'>
                <Avatar className={styles.accountItemAvatar} src={account.avatar ?? ''} />
                <span>{account.label}</span>
              </Space>
            </Checkbox>
          ))}
        </Space>
      </Checkbox.Group>
    </Space>
  );
};

export default AccountList;
