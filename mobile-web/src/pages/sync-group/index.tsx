import { useState } from 'react';
import { Button } from 'antd-mobile';
import AccountList from './components/AccountList';
import GroupList from './components/GroupList';
import styles from './index.module.less';

const GroupManagement = () => {
  const [selectedAccounts, setSelectedAccounts] = useState<number[]>([]);

  const handleSyncGroups = () => {
    console.log('同步群组:', selectedAccounts);
  };

  return (
    <div className='group-management'>
      {/* 选择账号 */}
      <AccountList selectedAccounts={selectedAccounts} onAccountChange={setSelectedAccounts} />

      <div className={styles.groupMgmtSyncBtn}>
        <Button color='primary' size='small' fill='solid' onClick={handleSyncGroups}>
          同步群组
        </Button>
      </div>

      {/* 群组列表 */}
      <GroupList accountIds={selectedAccounts} />
    </div>
  );
};

export default GroupManagement;
