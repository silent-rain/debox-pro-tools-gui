import { FC, useEffect, useState } from 'react';
import { Avatar, DotLoading, List, Switch } from 'antd-mobile';
import { CheckOutline, CloseOutline } from 'antd-mobile-icons';
import styles from './GroupList.module.less';
import { DeboxGroup, GetDeboxGroupsReq } from '@/typings/debox-group';
import { DeboxGroupApi } from '@/api/debox-group';
import Empty from '@/components/empty';

interface GroupListProps {
  accountIds: number[];
  groupsUpdateState: number;
}

// 获取群组列表
const fetchGroups = async (accountIds: number[]): Promise<DeboxGroup[]> => {
  if (accountIds.length === 0) {
    return [];
  }
  const data: GetDeboxGroupsReq = {
    all: true,
    account_ids: accountIds,
    status: true,
  };
  const response = await DeboxGroupApi.list(data);
  return response.data_list;
};

// 更新群组状态
const updateGroupStatus = async (groupId: number, status: boolean) => {
  const data = await DeboxGroupApi.updateStatus({
    id: groupId,
    status,
  });
  return data;
};

const GroupList: FC<GroupListProps> = ({ accountIds, groupsUpdateState }) => {
  const [groups, setGroups] = useState<DeboxGroup[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setGroups([]);
        setLoading(true);
        const data = await fetchGroups(accountIds);
        setGroups(data);
      } catch (err) {
        console.error(`fetchGroups error: ${err}`);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [accountIds, groupsUpdateState]);

  const handleSwitchChange = async (groupId: number, checked: boolean) => {
    await updateGroupStatus(groupId, checked);

    const data = await fetchGroups(accountIds);
    setGroups(data);
  };

  if (loading) {
    return <DotLoading color='primary' />;
  }

  if (accountIds.length === 0) {
    return <Empty title='请先选择账号' description='暂无数据' />;
  }

  if (groups.length === 0) {
    return <Empty />;
  }

  return (
    <>
      <List className={styles.groupList}>
        {groups.map((group) => (
          <List.Item
            key={group.id}
            prefix={<Avatar className={styles.groupAvatar} src={group.pic ?? ''} />}
            extra={
              <Switch
                checkedText={<CheckOutline fontSize={18} />}
                uncheckedText={<CloseOutline fontSize={18} />}
                defaultChecked={group.status}
                onChange={(checked) => handleSwitchChange(group.id, checked)}
              />
            }
          >
            {group.name}
          </List.Item>
        ))}
      </List>
    </>
  );
};

export default GroupList;
