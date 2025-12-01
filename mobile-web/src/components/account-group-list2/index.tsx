import { Dropdown } from 'antd-mobile';
import { FC, useEffect, useState } from 'react';
import AccountList from './components/AccountList';
import GroupList from './components/AccountGroupList';

interface AccountGroupListProps {
  multiple?: boolean; // 是否多选
  defaultSelected?: boolean; // 是否默认选中第一个账号
  onChange?: (accountIds: number[], accountGroupIds: number[]) => void;
}

const AccountGroupList: FC<AccountGroupListProps> = ({ multiple = false, defaultSelected = false, onChange }) => {
  const [accounts, setAccounts] = useState<number[]>([]);
  const [accountGroups, setAccountGroups] = useState<number[]>([]);

  useEffect(() => {
    if (!onChange) {
      return;
    }
    onChange(accounts, accountGroups);
  }, [accounts, accountGroups, onChange]);

  return (
    <div className='account-group-list'>
      <Dropdown>
        <Dropdown.Item key='account' title='选择账号'>
          <AccountList selected={accounts} setSelected={setAccounts}></AccountList>
        </Dropdown.Item>
      </Dropdown>

      <GroupList
        accounts={accounts}
        selectedAccountGroups={accountGroups}
        setSelectedAccountGroups={setAccountGroups}
        multiple={multiple}
        defaultSelected={defaultSelected}
      ></GroupList>
    </div>
  );
};

export default AccountGroupList;
