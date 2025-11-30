import { Checkbox, Space, Avatar, Dropdown, DotLoading } from 'antd-mobile';
import { FC, useCallback, useEffect, useState } from 'react';
import { DeboxAccount, GetDeboxAccountsReq } from '@/typings/debox-account';
import { DeboxAccountApi } from '@/api/debox-account';
import styles from './AccountGroupList.module.less';
import { DeboxGroup, GetDeboxGroupsReq } from '@/typings/debox-group';
import { DeboxGroupApi } from '@/api/debox-group';
import Empty from '@/components/empty';

interface AccountListProps {
  selectedAccounts: number[];
  selectedGroups: number[];
  onAccountChange: (selected: number[]) => void;
  onGroupChange: (selected: number[]) => void;
}

// 获取账号列表
const fetchAccounts = async (): Promise<DeboxAccount[]> => {
  const data: GetDeboxAccountsReq = {
    page: 0,
    page_size: 0,
    all: true,
    status: true,
  };
  const response = await DeboxAccountApi.list(data);
  return response.data_list;
};

// 获取群组列表
const fetchGroups = async (accountIds: number[]): Promise<DeboxGroup[]> => {
  const data: GetDeboxGroupsReq = {
    page: 0,
    page_size: 0,
    all: true,
    status: true,
    account_ids: accountIds,
  };
  const response = await DeboxGroupApi.list(data);
  return response.data_list;
};

const AccountList: FC<AccountListProps> = ({ selectedAccounts, selectedGroups, onAccountChange, onGroupChange }) => {
  const [accounts, setAccounts] = useState<DeboxAccount[]>([]);
  const [groups, setGroups] = useState<DeboxGroup[]>([]);
  const [loading, setLoading] = useState(false);

  // 获取账号列表
  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setLoading(true);
        const data = await fetchAccounts();
        setAccounts(data);

        // 默认第一个账号选中
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

  // 获取群组列表
  useEffect(() => {
    const loadGroups = async () => {
      if (selectedAccounts.length === 0) {
        setGroups([]);
        return;
      }
      try {
        setLoading(true);
        const data = await fetchGroups(selectedAccounts);
        setGroups(data);

        // 默认第一个群组选中
        if (data.length > 0) {
          onGroupChange([data[0].id]);
        }
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    loadGroups();
  }, [onGroupChange, selectedAccounts]);

  const handleSelectAllAccount = useCallback(
    (checked: boolean) => {
      if (checked) {
        onAccountChange(accounts.map((account) => account.id));
      } else {
        onAccountChange([]);
      }
    },
    [accounts, onAccountChange],
  );

  const handleSelectAllGroup = useCallback(
    (checked: boolean) => {
      if (checked) {
        onGroupChange(groups.map((group) => group.id));
      } else {
        onGroupChange([]);
      }
    },
    [groups, onGroupChange],
  );

  if (loading) {
    return <DotLoading color='primary' />;
  }

  return (
    <Dropdown>
      <Dropdown.Item key='account' title='选择用户'>
        <div className={styles.allAccountsCheckbox}>
          {accounts.length > 0 ? (
            <Checkbox
              indeterminate={selectedAccounts.length > 0 && selectedAccounts.length < accounts.length}
              checked={selectedAccounts.length === accounts.length}
              onChange={handleSelectAllAccount}
            >
              全选
            </Checkbox>
          ) : (
            <></>
          )}
        </div>

        <div className={styles.accountListDivider}></div>
        {accounts.length === 0 ? <Empty title='暂无数据' /> : <></>}

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

      <Dropdown.Item key='group' title='选择群组'>
        <div className={styles.allGroupsCheckbox}>
          {groups.length > 0 ? (
            <Checkbox
              indeterminate={selectedGroups.length > 0 && selectedGroups.length < groups.length}
              checked={selectedGroups.length === groups.length}
              onChange={handleSelectAllGroup}
            >
              全选
            </Checkbox>
          ) : (
            <></>
          )}
        </div>

        <div className={styles.groupListDivider}></div>

        {groups.length === 0 ? <Empty title='暂无数据' /> : <></>}

        <Checkbox.Group value={selectedGroups} onChange={(values) => onGroupChange(values as number[])}>
          <Space className={styles.groupList} direction='vertical'>
            {groups.map((group) => (
              <Checkbox key={group.id} value={group.id}>
                <Space className={styles.groupItem} align='center'>
                  <Avatar className={styles.groupItemAvatar} src={group.pic ?? ''} />
                  <span>{group.name}</span>
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
