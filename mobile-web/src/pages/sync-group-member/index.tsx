import { useState } from 'react';
import { Button, Dropdown, Toast } from 'antd-mobile';
import AccountList from '@/components/account-list';
import AccountGroupList from '@/components/account-group-list';
import GroupMemberList from './components/GroupMemberList';
import styles from './index.module.scss';
import { DeboxGroupMemberApi } from '@/api';

// 同步DeBox群组列表
const syncGroupMembers = async (groupIds: number[]) => {
  const data = {
    group_ids: groupIds,
  };
  await DeboxGroupMemberApi.syncGroupMembers(data);
};

const GroupMemberManagement = () => {
  const [accountId, setAccountId] = useState<number>(0);
  const [accountGroupIds, setAccountGroupIds] = useState<number[]>([]);
  const [groupMembersUpdateState, setGroupMembersUpdateState] = useState<number>(0);

  // 同步群组成员
  const handleSyncGroupMembers = async () => {
    if (accountId === 0) {
      Toast.show({
        content: '请选择账号',
        position: 'top',
      });
      return;
    }

    await syncGroupMembers(accountGroupIds);

    // 更新 GroupList
    setGroupMembersUpdateState((prev) => prev + 1);
  };

  // 刷新群员列表
  const handleRefreshGroupMembers = () => {
    setGroupMembersUpdateState((prev) => prev + 1);
  };

  return (
    <div className='group-management'>
      <Dropdown defaultActiveKey='account'>
        <Dropdown.Item key='account' title='选择账号'>
          <AccountList
            defaultSelected
            onChange={(accountIds) => {
              setAccountId(accountIds[0]);
            }}
          />
        </Dropdown.Item>

        <Dropdown.Item key='group' title='选择群组'>
          <AccountGroupList
            accountId={accountId}
            multiple
            defaultSelected
            className={styles.accountGroupList}
            onChange={(accountGroupIds) => {
              setAccountGroupIds(accountGroupIds);
            }}
          />
        </Dropdown.Item>
      </Dropdown>

      <div className={styles.groupMgmtSyncBtn}>
        <Button color='primary' size='small' fill='solid' onClick={handleRefreshGroupMembers}>
          刷新
        </Button>
        <Button color='primary' size='small' fill='solid' onClick={handleSyncGroupMembers}>
          同步群员
        </Button>
      </div>

      {/* 群组成员列表 */}
      <GroupMemberList
        accountId={accountId}
        groupIds={accountGroupIds}
        groupsMemberUpdateState={groupMembersUpdateState}
      />
    </div>
  );
};

export default GroupMemberManagement;
