import { Dropdown, DotLoading, CheckList, DropdownRef } from 'antd-mobile';
import { FC, useEffect, useRef, useState } from 'react';
import { DeboxAccount, GetDeboxAccountsReq } from '@/typings/debox-account';
import { DeboxAccountApi } from '@/api/debox-account';
import styles from './AccountList.module.less';

interface AccountListProps {
  selected: number[];
  setSelected: (selected: number[]) => void;
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

const AccountList: FC<AccountListProps> = ({ selected, setSelected }) => {
  const [accounts, setAccounts] = useState<DeboxAccount[]>([]);
  const [loading, setLoading] = useState(false);

  const ref = useRef<DropdownRef>(null);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setLoading(true);
        const data = await fetchAccounts();
        setAccounts(data);

        // 默认第一个账号选中
        if (data.length > 0) {
          setSelected([data[0].id]);
        }
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [setSelected]);

  if (loading) {
    return <DotLoading color='primary' />;
  }

  return (
    <Dropdown ref={ref}>
      <Dropdown.Item key='account' title='选择用户'>
        <CheckList
          className={styles.allAccounts}
          defaultValue={selected ? selected : []}
          onChange={(val) => {
            setSelected([Number(val[0])]);
            ref.current?.close();
          }}
        >
          {accounts.map((item) => (
            <CheckList.Item key={item.id} value={item.id}>
              {item.name}
            </CheckList.Item>
          ))}
        </CheckList>
      </Dropdown.Item>
    </Dropdown>
  );
};

export default AccountList;
