import { useState } from 'react';
import { Button, Toast } from 'antd-mobile';
import AccountList from './components/AccountGroupList';
import GroupMemberList from './components/GroupMemberList';
import styles from './index.module.less';
import { DeboxGroupMemberApi } from '@/api';

// 同步DeBox群组列表
const syncGroupMembers = async (groupIds: number[]) => {
  const data = {
    group_ids: groupIds,
  };
  await DeboxGroupMemberApi.syncGroupMembers(data);
};

const GroupMemberManagement = () => {
  const [selectedAccounts, setSelectedAccounts] = useState<number[]>([]);
  const [selectedGroups, setGroups] = useState<number[]>([]);
  const [groupMembersUpdateState, setGroupMembersUpdateState] = useState<number>(0);

  // 同步群组成员
  const handleSyncGroupMembers = async () => {
    if (selectedAccounts.length === 0) {
      Toast.show({
        content: '请选择账号',
        position: 'top',
      });
      return;
    }

    await syncGroupMembers(selectedGroups);

    // 更新 GroupList
    setGroupMembersUpdateState((prev) => prev + 1);
  };

  // 刷新群员列表
  const handleRefreshGroupMembers = () => {
    setGroupMembersUpdateState((prev) => prev + 1);
  };

  return (
    <div className='group-management'>
      {/* 选择账号 */}
      <AccountList
        selectedAccounts={selectedAccounts}
        selectedGroups={selectedGroups}
        onAccountChange={setSelectedAccounts}
        onGroupChange={setGroups}
      />

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
        accountIds={selectedAccounts}
        groupIds={selectedGroups}
        groupsMemberUpdateState={groupMembersUpdateState}
      />
    </div>
  );
};

export default GroupMemberManagement;
