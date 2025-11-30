import { useState } from 'react';
import { useLocation } from 'react-router-dom';
import { Toast, Tabs, Button } from 'antd-mobile';
import styles from './index.module.less';
import AccountList from '@/components/account-list';
import AccountGroupList from '@/components/account-group-list';
import { DeboxAccountFollowFollowApi } from '@/api';
import { BatchAccountFollowsReq } from '@/typings/debox-account-follows';
import { FollowType } from '@/enums/debox-account-follows';

import UserSearchList from './components/UserSearchList';

// 批量关注用户
const batchFollows = async (data: BatchAccountFollowsReq) => {
  await DeboxAccountFollowFollowApi.batchFollows(data);
};

const AccountFollowForm = () => {
  const location = useLocation();
  const [selectedAccounts, setSelectedAccounts] = useState<number[]>([]);
  const [selectedAccountGroups, setSelectedAccountGroups] = useState<number[]>([]);
  const [userIds, setUserIds] = useState<number[]>([]);

  const { accountId } = location.state || {};

  const handleSubmit = async (followType: FollowType) => {
    try {
      switch (followType) {
        case FollowType.Account: {
          for (const targetAccountId of selectedAccounts) {
            const data: BatchAccountFollowsReq = {
              account_id: accountId,
              target_account_id: targetAccountId,
              follow_type: FollowType.Account,
            };
            await batchFollows(data);
          }
          break;
        }
        case FollowType.Group: {
          for (const targetAccountId of selectedAccounts) {
            for (const groupId of selectedAccountGroups) {
              const data: BatchAccountFollowsReq = {
                account_id: accountId,
                target_account_id: targetAccountId,
                target_group_id: groupId,
                follow_type: FollowType.Group,
              };
              await batchFollows(data);
            }
          }
          break;
        }
        case FollowType.User: {
          const data: BatchAccountFollowsReq = {
            account_id: accountId,
            debox_user_ids: userIds.map(String),
            follow_type: FollowType.User,
          };
          await batchFollows(data);
          break;
        }
      }

      Toast.show({
        icon: 'success',
        content: '添加成功',
      });
      // navigate(ROUTES.ACCOUNT_FOLLOW_MANAGEMENT, { replace: true });
    } catch (error) {
      console.error('添加关注人失败, err: ', error);
      Toast.show({
        icon: 'fail',
        content: '操作失败',
      });
    }
  };

  return (
    <div className='account-follow-form'>
      <Tabs defaultActiveKey='1'>
        <Tabs.Tab title='账号' key='1'>
          <p>请选择一个账号进行批量关注用户</p>
          <div className={styles.operationButton}>
            <Button
              color='primary'
              size='small'
              fill='solid'
              onClick={() => {
                handleSubmit(FollowType.Account);
              }}
            >
              提交
            </Button>
          </div>
          <AccountList
            defaultSelected={true}
            onChange={(accountIds: number[]) => {
              setSelectedAccounts(accountIds);
            }}
          />
        </Tabs.Tab>
        <Tabs.Tab title='群组' key='2'>
          <p>请选择一个账号的群组进行批量关注用户</p>
          <div className={styles.operationButton}>
            <Button
              color='primary'
              size='small'
              fill='solid'
              onClick={() => {
                handleSubmit(FollowType.Group);
              }}
            >
              提交
            </Button>
          </div>
          <AccountGroupList
            defaultSelected={true}
            onChange={(accountIds: number[], accountGroupIds: number[]) => {
              setSelectedAccounts(accountIds);
              setSelectedAccountGroups(accountGroupIds);
            }}
          />
        </Tabs.Tab>
        <Tabs.Tab title='用户' key='3'>
          <p>指定账号进行关注用户</p>
          <div className={styles.operationButton}>
            <Button
              color='primary'
              size='small'
              fill='solid'
              onClick={() => {
                handleSubmit(FollowType.User);
              }}
            >
              提交
            </Button>
          </div>

          <UserSearchList accountId={accountId} userIds={userIds} setUserIds={setUserIds} />
        </Tabs.Tab>
      </Tabs>
    </div>
  );
};

export default AccountFollowForm;
