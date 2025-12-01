import { DotLoading, Avatar, Space, Checkbox } from 'antd-mobile';
import { FC, useCallback, useEffect, useState } from 'react';
import Empty from '@/components/empty';
import { DeboxAccount, GetDeboxAccountsReq } from '@/typings/debox-account';
import { DeboxAccountApi } from '@/api/debox-account';
import styles from './index.module.less';

interface AccountListProps {
  multiple?: boolean; // 是否多选
  defaultSelected?: boolean; // 是否默认选中第一个账号
  onChange?: (accountIds: number[]) => void;
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

const AccountList: FC<AccountListProps> = ({ multiple = false, defaultSelected = false, onChange }) => {
  const [accounts, setAccounts] = useState<DeboxAccount[]>([]);
  const [accountIds, setAccountIds] = useState<number[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setLoading(true);
        const data = await fetchAccounts();
        setAccounts(data);

        // 默认第一个账号选中
        if (defaultSelected && data.length > 0) {
          setAccountIds([data[0].id]);
        }
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [defaultSelected]);

  useEffect(() => {
    if (!onChange) {
      return;
    }
    onChange(accountIds);
  }, [accountIds, onChange]);

  // 全选
  const handleSelectAll = useCallback(
    (checked: boolean) => {
      if (checked) {
        setAccountIds(accounts.map((item) => item.id));
      } else {
        setAccountIds([]);
      }
    },
    [accounts, setAccountIds],
  );

  if (loading) {
    return <DotLoading color='primary' />;
  }
  if (accounts.length === 0) {
    return <Empty className={styles.empty} title='暂无数据' />;
  }

  return (
    <div className='account-list'>
      <div className={styles.allCheckbox}>
        {multiple ? (
          <Checkbox
            indeterminate={accountIds.length > 0 && accountIds.length < accounts.length}
            checked={accountIds.length === accounts.length}
            onChange={handleSelectAll}
          >
            全选
          </Checkbox>
        ) : (
          <></>
        )}
      </div>

      <Checkbox.Group
        value={accountIds}
        onChange={(values) => {
          if (!values || values.length === 0) {
            setAccountIds([]);
            return;
          }
          if (multiple) {
            setAccountIds(values as number[]);
          } else {
            setAccountIds([Number(values[values.length - 1])]);
          }
        }}
      >
        <Space className={styles.accountList} direction='vertical'>
          {accounts.map((item) => (
            <Checkbox key={item.id} value={item.id}>
              <Space align='center'>
                <Avatar src={item.avatar ?? ''} />
                <span>{item.name}</span>
              </Space>
            </Checkbox>
          ))}
        </Space>
      </Checkbox.Group>
    </div>
  );
};

export default AccountList;
