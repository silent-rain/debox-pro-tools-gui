import { Checkbox, Space, Avatar, DotLoading } from 'antd-mobile';
import { FC, useCallback, useEffect, useState } from 'react';
import { DeboxAccount, GetDeboxAccountsReq } from '@/typings/debox-account';
import { DeboxAccountApi } from '@/api/debox-account';
import styles from './AccountList.module.less';

interface AccountListProps {
  selected: number[]; // 账号ID列表
  setSelected: (selected: number[]) => void; // 设置选中的账号ID列表
  multiple?: boolean; // 是否多选
  defaultSelected?: boolean; // 是否默认选中第一个账号
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

const AccountList: FC<AccountListProps> = ({
  selected = [],
  setSelected,
  multiple = false,
  defaultSelected = false,
}) => {
  const [accounts, setAccounts] = useState<DeboxAccount[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setLoading(true);
        const data = await fetchAccounts();
        setAccounts(data);

        // 默认第一个账号选中
        if (defaultSelected && data.length > 0) {
          setSelected([data[0].id]);
        }
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [defaultSelected, setSelected]);

  const handleSelectAll = useCallback(
    (checked: boolean) => {
      if (checked) {
        setSelected(accounts.map((account) => account.id));
      } else {
        setSelected([]);
      }
    },
    [accounts, setSelected],
  );

  if (loading) {
    return <DotLoading color='primary' />;
  }

  return (
    <div className='account-list'>
      {multiple ? (
        <div className={styles.allAccountsCheckbox}>
          <Checkbox
            indeterminate={selected.length > 0 && selected.length < accounts.length}
            checked={selected.length === accounts.length}
            onChange={handleSelectAll}
          >
            全选
          </Checkbox>
        </div>
      ) : (
        <></>
      )}

      <div className={styles.accountListDivider}></div>

      <Checkbox.Group
        value={selected}
        onChange={(values) => {
          if (!values || values.length === 0) {
            setSelected([]);
            return;
          }
          if (multiple) {
            setSelected(values as number[]);
          } else {
            setSelected([Number(values[values.length - 1])]);
          }
        }}
      >
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
    </div>
  );
};

export default AccountList;
