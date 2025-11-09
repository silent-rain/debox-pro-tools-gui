import { useState } from 'react';
import { Button, Toast } from 'antd-mobile';
import AccountList from './components/AccountList';
import GroupList from './components/GroupList';
import styles from './index.module.less';
import { DeboxGroupApi } from '@/api/debox-group';

// 同步DeBox群组列表
const syncGroups = async (accountIds: number[]) => {
  const data = {
    account_ids: accountIds,
  };
  await DeboxGroupApi.syncGroups(data);
};

const GroupManagement = () => {
  const [selectedAccounts, setSelectedAccounts] = useState<number[]>([]);
  const [groupsUpdateState, setGroupsUpdateState] = useState<number>(0);

  // 同步群组
  const handleSyncGroups = async () => {
    if (selectedAccounts.length === 0) {
      Toast.show({
        content: '请选择账号',
        position: 'top',
      });
      return;
    }

    await syncGroups(selectedAccounts);

    // 更新 GroupList
    setGroupsUpdateState((prev) => prev + 1);
  };

  // 刷新群组列表
  const handleRefreshGroups = () => {
    setGroupsUpdateState((prev) => prev + 1);
  };

  return (
    <div className='group-management'>
      {/* 选择账号 */}
      <AccountList selectedAccounts={selectedAccounts} onAccountChange={setSelectedAccounts} />

      <div className={styles.groupMgmtSyncBtn}>
        <Button color='primary' size='small' fill='solid' onClick={handleRefreshGroups}>
          刷新
        </Button>

        <Button color='primary' size='small' fill='solid' onClick={handleSyncGroups}>
          同步群组
        </Button>
      </div>

      {/* 群组列表 */}
      <GroupList accountIds={selectedAccounts} groupsUpdateState={groupsUpdateState} />
    </div>
  );
};

export default GroupManagement;
