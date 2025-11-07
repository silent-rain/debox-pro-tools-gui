import { FC, useEffect, useState } from 'react';
import { Avatar, DotLoading, List, Switch } from 'antd-mobile';
import { CheckOutline, CloseOutline } from 'antd-mobile-icons';
import styles from './GroupMemberList.module.less';
import Empty from '@/components/empty';
import { DeboxGroupMemberApi } from '@/api';
import { DeboxGroupMember, GetDeboxGroupMembersReq } from '@/typings/debox-group-member';

interface GroupListProps {
  accountIds: number[];
  groupIds: number[];
  groupsMemberUpdateState: number;
}

// 获取群组成员列表
const fetchGroupMembers = async (groupIds: number[]): Promise<DeboxGroupMember[]> => {
  const data: GetDeboxGroupMembersReq = {
    all: true,
    group_ids: groupIds,
    status: true,
  };
  const response = await DeboxGroupMemberApi.list(data);
  return response.data_list;
};

// 更新群组状态
const updateGroupMemberStatus = async (memberId: number, status: boolean) => {
  const data = await DeboxGroupMemberApi.updateStatus({
    id: memberId,
    status,
  });
  return data;
};

const GroupMemberList: FC<GroupListProps> = ({ accountIds, groupIds, groupsMemberUpdateState }) => {
  const [groupMembers, setGroupMembers] = useState<DeboxGroupMember[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setLoading(true);
        const data = await fetchGroupMembers(groupIds);
        setGroupMembers(data);
      } catch (err) {
        console.error(`fetchGroups error: ${err}`);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [groupIds, groupsMemberUpdateState]);

  const handleSwitchChange = async (groupId: number, checked: boolean) => {
    await updateGroupMemberStatus(groupId, checked);

    const data = await fetchGroupMembers(groupIds);
    setGroupMembers(data);
  };

  if (loading) {
    return <DotLoading color='primary' />;
  }

  if (accountIds.length === 0) {
    return <Empty title='请先选择账号' description='暂无数据' />;
  }

  if (groupIds.length === 0) {
    return <Empty title='请先选择群组' description='暂无数据' />;
  }

  if (groupMembers.length === 0) {
    return <Empty />;
  }

  return (
    <>
      <List className={styles.groupList}>
        {groupMembers.map((groupMember) => (
          <List.Item
            key={groupMember.id}
            prefix={<Avatar className={styles.groupAvatar} src={groupMember.pic ?? ''} />}
            extra={
              <Switch
                checkedText={<CheckOutline fontSize={18} />}
                uncheckedText={<CloseOutline fontSize={18} />}
                defaultChecked={groupMember.status}
                onChange={(checked) => handleSwitchChange(groupMember.id, checked)}
              />
            }
          >
            {groupMember.name}
          </List.Item>
        ))}
      </List>
    </>
  );
};

export default GroupMemberList;
