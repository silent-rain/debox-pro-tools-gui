import { DotLoading, CheckList, DropdownRef, Avatar, Space } from 'antd-mobile';
import { FC, useEffect, useRef, useState } from 'react';
import Empty from '@/components/empty';
import { DeboxAccount, GetDeboxAccountsReq } from '@/typings/debox-account';
import { DeboxAccountApi } from '@/api/debox-account';
import './index.module.less';

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

  const ref = useRef<DropdownRef>(null);

  useEffect(() => {
    if (!onChange) {
      return;
    }
    onChange(accountIds);
  }, [accountIds, onChange]);

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
  }, [defaultSelected, setAccountIds]);

  if (loading) {
    return <DotLoading color='primary' />;
  }

  return (
    <div className='account-list'>
      {accounts.length === 0 ? <Empty title='暂无数据' /> : <></>}

      <CheckList
        defaultValue={accountIds ? accountIds : []}
        onChange={(val) => {
          if (multiple) {
            setAccountIds(val.map((v) => Number(v)));
          } else {
            setAccountIds([Number(val[0])]);
          }

          ref.current?.close();
        }}
      >
        {accounts.map((item) => (
          <CheckList.Item key={item.id} value={item.id}>
            <Space align='center'>
              <Avatar src={item.avatar ?? ''} />
              <span>{item.name}</span>
            </Space>
          </CheckList.Item>
        ))}
      </CheckList>
    </div>
  );
};

export default AccountList;
