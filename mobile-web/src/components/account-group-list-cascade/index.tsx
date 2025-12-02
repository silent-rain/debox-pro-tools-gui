import { Dropdown } from 'antd-mobile';
import { FC, useEffect, useState } from 'react';
import AccountList from '@/components/account-list';
import AccountGroupList from '@/components/account-group-list';

interface AccountGroupListCascadeProps {
  multiple?: boolean; // 是否多选
  defaultSelected?: boolean; // 是否默认选中第一个账号
  onChange?: (accountId: number, accountGroupIds: number[]) => void;
}

const AccountGroupListCascade: FC<AccountGroupListCascadeProps> = ({
  multiple = false,
  defaultSelected = false,
  onChange,
}) => {
  const [accountId, setAccountId] = useState<number>(0);
  const [accountGroupIds, setAccountGroupIds] = useState<number[]>([]);

  useEffect(() => {
    if (!onChange) {
      return;
    }
    if (!accountId || accountId === 0 || !accountGroupIds || accountGroupIds.length === 0) {
      return;
    }
    onChange(accountId, accountGroupIds);
  }, [accountId, accountGroupIds, onChange]);

  return (
    <div className='account-group-list'>
      <Dropdown defaultActiveKey='account'>
        <Dropdown.Item key='account' title='选择账号'>
          <AccountList
            defaultSelected
            onChange={(accountIds) => {
              if (!accountIds || accountIds.length === 0) {
                return;
              }
              setAccountId(accountIds[0]);
            }}
          />
        </Dropdown.Item>

        <Dropdown.Item key='group' title='选择群组'>
          <AccountGroupList
            accountId={accountId}
            multiple={multiple}
            defaultSelected={defaultSelected}
            onChange={(accountGroupIds) => {
              setAccountGroupIds(accountGroupIds);
            }}
          />
        </Dropdown.Item>
      </Dropdown>
    </div>
  );
};

export default AccountGroupListCascade;
