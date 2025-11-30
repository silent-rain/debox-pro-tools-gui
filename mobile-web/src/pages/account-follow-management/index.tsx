import { useState } from 'react';
import { Button, Toast } from 'antd-mobile';
import AccountList from './components/AccountList';
import AccountFollowList from './components/AccountFollowList';
import styles from './index.module.less';
import { DeboxAccountFollowFollowApi } from '@/api';

// 同步关注人列表
const syncFollows = async (accountIds: number[]) => {
  const data = {
    account_ids: accountIds,
  };
  await DeboxAccountFollowFollowApi.syncFollows(data);
};

// 群组/指定人/关注人列表/用户

const FollowManagement = () => {
  const [selectedAccounts, setSelectedAccounts] = useState<number[]>([]);
  const [followsUpdateState, setFollowsUpdateState] = useState<number>(0);

  // 同步关注人
  const handleSyncFollows = async () => {
    if (selectedAccounts.length === 0) {
      Toast.show({
        content: '请选择账号',
        position: 'top',
      });
      return;
    }

    await syncFollows(selectedAccounts);

    setFollowsUpdateState((prev) => prev + 1);
  };

  // 刷新关注人列表
  const handleRefreshFollows = () => {
    setFollowsUpdateState((prev) => prev + 1);
  };

  return (
    <div className='account-follow-management'>
      {/* 选择账号 */}
      <AccountList selected={selectedAccounts} setSelected={setSelectedAccounts} />

      <div className={styles.operationButton}>
        <Button color='primary' size='small' fill='solid' onClick={handleRefreshFollows}>
          刷新
        </Button>

        <Button color='primary' size='small' fill='solid' onClick={handleSyncFollows}>
          同步关注人
        </Button>

        <Button color='primary' size='small' fill='solid' onClick={handleSyncFollows}>
          添加关注人
        </Button>
      </div>

      <AccountFollowList accountIds={selectedAccounts} followsUpdateState={followsUpdateState} />
    </div>
  );
};

export default FollowManagement;
