import { useState, FC, useEffect } from 'react';
import { Dropdown } from 'antd-mobile';
import AccountList from '@/components/account-list';
import GroupList from '@/components/account-group-list';
import './AccountGroupList.module.scss';

interface AccountGroupListProps {
  multiple?: boolean; // 是否多选
  defaultSelected?: boolean; // 是否默认选中第一个账号
  onChange?: (accountId: number, accountGroupIds: number[]) => void;
}

const AccountGroupList: FC<AccountGroupListProps> = ({ multiple = false, defaultSelected = false, onChange }) => {
  const [accountId, setAccountId] = useState<number>(0);
  const [accountGroupIds, setAccountGroupIds] = useState<number[]>([]);

  useEffect(() => {
    if (!onChange) {
      return;
    }
    onChange(accountId, accountGroupIds);
  }, [accountId, accountGroupIds, onChange]);

  return (
    <div className='account-group-list'>
      {/* 选择账号 */}
      <Dropdown defaultActiveKey='account'>
        <Dropdown.Item key='account' title='选择账号'>
          <AccountList
            multiple
            defaultSelected
            onChange={(accountIds) => {
              if (accountIds.length === 0) {
                return;
              }
              setAccountId(accountIds[0]);
            }}
          />
        </Dropdown.Item>
      </Dropdown>

      {/* 群组列表 */}
      <GroupList
        accountId={accountId}
        multiple={multiple}
        defaultSelected={defaultSelected}
        onChange={(groupIds) => {
          setAccountGroupIds(groupIds);
        }}
      />
    </div>
  );
};

export default AccountGroupList;
