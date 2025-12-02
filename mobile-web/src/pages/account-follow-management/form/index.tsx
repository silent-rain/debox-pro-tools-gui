import { useState, useRef } from 'react';
import { useLocation } from 'react-router-dom';
import { Toast, Tabs, Button } from 'antd-mobile';
import styles from './index.module.scss';
import AccountList from '@/components/account-list';
import AccountGroupList from './components/AccountGroupList';
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
  const [userIds, setUserIds] = useState<number[]>([]);
  const selectedAccountIds = useRef<number[]>([]);
  const selectedAccountGroupIds = useRef<number[]>([]);

  const { accountId } = location.state || {};

  const handleSubmit = async (followType: FollowType) => {
    try {
      switch (followType) {
        case FollowType.Account: {
          for (const targetAccountId of selectedAccountIds.current) {
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
          if (selectedAccountIds.current.length === 0 || selectedAccountGroupIds.current.length === 0) {
            return;
          }
          const targetAccountId = selectedAccountIds.current[0];
          for (const groupId of selectedAccountGroupIds.current) {
            const data: BatchAccountFollowsReq = {
              account_id: accountId,
              target_account_id: targetAccountId,
              target_group_id: groupId,
              follow_type: FollowType.Group,
            };
            await batchFollows(data);
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
        <Tabs.Tab title='账号' key='account'>
          <p>请选择一个账号进行批量关注该账号的用户</p>
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
              if (accountIds.length === 0) {
                return;
              }
              selectedAccountIds.current = accountIds;
            }}
          />
        </Tabs.Tab>
        <Tabs.Tab title='群组' key='account-group'>
          <p>请选择一个账号的群组进行批量关注该群组中的用户</p>
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
            onChange={(accountId: number, accountGroupIds: number[]) => {
              if (!accountId || accountId === 0 || !accountGroupIds || accountGroupIds.length === 0) {
                return;
              }
              selectedAccountIds.current = [accountId];
              selectedAccountGroupIds.current = accountGroupIds;
            }}
          />
        </Tabs.Tab>
        <Tabs.Tab title='用户' key='user'>
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
