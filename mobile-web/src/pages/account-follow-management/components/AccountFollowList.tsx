import { FC, useEffect, useState } from 'react';
import { Avatar, DotLoading, List, Switch } from 'antd-mobile';
import { CheckOutline, CloseOutline } from 'antd-mobile-icons';
import styles from './AccountFollowList.module.less';
import Empty from '@/components/empty';
import { DeboxAccountFollowFollowApi } from '@/api';
import { DeboxAccountFollow, GetDeboxAccountFollowsReq } from '@/typings/debox-account-follows';

interface AccountFollowListProps {
  accountIds: number[];
  followsUpdateState: number;
}

// 获取账号关注人列表
const fetchAccountFollows = async (accountIds: number[]): Promise<DeboxAccountFollow[]> => {
  const data: GetDeboxAccountFollowsReq = {
    page: 0,
    page_size: 0,
    all: true,
    status: true,
    account_ids: accountIds,
  };
  const response = await DeboxAccountFollowFollowApi.list(data);
  return response.data_list;
};

// 更新账号关注人状态
const updateAccountFollowStatus = async (followId: number, status: boolean) => {
  const data = await DeboxAccountFollowFollowApi.updateStatus({
    id: followId,
    status,
  });
  return data;
};

const AccountFollowList: FC<AccountFollowListProps> = ({ accountIds, followsUpdateState }) => {
  const [accountFollows, setAccountFollows] = useState<DeboxAccountFollow[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setLoading(true);
        const data = await fetchAccountFollows(accountIds);
        setAccountFollows(data);
      } catch (err) {
        console.error(`fetchGroups error: ${err}`);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [accountIds, followsUpdateState]);

  const handleSwitchChange = async (followId: number, checked: boolean) => {
    await updateAccountFollowStatus(followId, checked);

    const data = await fetchAccountFollows(accountIds);
    setAccountFollows(data);
  };

  if (loading) {
    return <DotLoading color='primary' />;
  }

  if (accountIds.length === 0) {
    return <Empty title='请先选择账号' description='暂无数据' />;
  }

  if (accountFollows.length === 0) {
    return <Empty />;
  }

  return (
    <>
      <List className={styles.groupList}>
        {accountFollows.map((accountFollow) => (
          <List.Item
            key={accountFollow.id}
            prefix={<Avatar className={styles.groupAvatar} src={accountFollow.avatar ?? ''} />}
            extra={
              <Switch
                checkedText={<CheckOutline fontSize={18} />}
                uncheckedText={<CloseOutline fontSize={18} />}
                defaultChecked={accountFollow.status}
                onChange={(checked) => handleSwitchChange(accountFollow.id, checked)}
              />
            }
          >
            {accountFollow.name}
          </List.Item>
        ))}
      </List>
    </>
  );
};

export default AccountFollowList;
