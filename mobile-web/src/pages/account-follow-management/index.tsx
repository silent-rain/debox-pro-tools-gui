import { useState } from 'react';
import { Button, Dropdown } from 'antd-mobile';
import AccountList from '@/components/account-list';
import AccountFollowList from './components/AccountFollowList';
import styles from './index.module.less';
import { DeboxAccountFollowFollowApi } from '@/api';
import { ROUTES } from '@/constants/routes';
import { useNavigate } from 'react-router';

// 同步关注人列表
const syncFollows = async (accountId: number) => {
  const data = {
    account_ids: [accountId],
  };
  await DeboxAccountFollowFollowApi.syncFollows(data);
};

const FollowManagement = () => {
  const navigate = useNavigate();
  const [accountId, setAccountId] = useState<number>(0);
  const [followsUpdateState, setFollowsUpdateState] = useState<number>(0);

  // 同步关注人
  const handleSyncFollows = async () => {
    if (accountId === 0) {
      return;
    }

    await syncFollows(accountId);

    setFollowsUpdateState((prev) => prev + 1);
  };

  // 刷新关注人列表
  const handleRefreshFollows = () => {
    setFollowsUpdateState((prev) => prev + 1);
  };

  // 跳转到添加关注人页面
  const handleAddFollows = () => {
    navigate(ROUTES.ACCOUNT_FOLLOW_MANAGEMENT_FORM, {
      state: { replace: true, mode: 'add', accountId: accountId },
    });
  };

  return (
    <div className='account-follow-management'>
      {/* 选择账号 */}
      <Dropdown defaultActiveKey='account'>
        <Dropdown.Item key='account' title='选择账号'>
          <AccountList
            defaultSelected
            onChange={(accountIds) => {
              setAccountId(accountIds[0]);
            }}
          />
        </Dropdown.Item>
      </Dropdown>

      <div className={styles.operationButton}>
        <Button color='primary' size='small' fill='solid' onClick={handleRefreshFollows}>
          刷新
        </Button>

        <Button color='primary' size='small' fill='solid' onClick={handleSyncFollows}>
          同步关注人
        </Button>

        <Button color='primary' size='small' fill='solid' onClick={handleAddFollows}>
          添加关注人
        </Button>
      </div>

      <AccountFollowList accountId={accountId} followsUpdateState={followsUpdateState} />
    </div>
  );
};

export default FollowManagement;
