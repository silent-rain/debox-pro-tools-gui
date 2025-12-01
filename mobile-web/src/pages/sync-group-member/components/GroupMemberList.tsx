import { FC, useEffect, useState } from 'react';
import { Avatar, DotLoading, List, Switch } from 'antd-mobile';
import { CheckOutline, CloseOutline } from 'antd-mobile-icons';
import Empty from '@/components/empty';
import { DeboxAccountApi, DeboxGroupApi, DeboxGroupMemberApi } from '@/api';
import { DeboxGroupMember, GetDeboxGroupMembersReq } from '@/typings/debox-group-member';
import { DeboxAccount, GetDeboxAccountsReq } from '@/typings/debox-account';
import { DeboxGroup, GetDeboxGroupsReq } from '@/typings/debox-group';
import styles from './GroupMemberList.module.less';

interface GroupListProps {
  accountId: number;
  groupIds: number[];
  groupsMemberUpdateState: number;
}

// 获取账号列表
const fetchAccounts = async (): Promise<DeboxAccount[]> => {
  const data: GetDeboxAccountsReq = {
    page: 0,
    page_size: 0,
    all: true,
    status: true,
  };
  const response = await DeboxAccountApi.list(data);
  return response.data_list;
};

// 获取群组列表
const fetchGroups = async (): Promise<DeboxGroup[]> => {
  const data: GetDeboxGroupsReq = {
    all: true,
    account_ids: [],
    status: true,
  };
  const response = await DeboxGroupApi.list(data);
  return response.data_list;
};

// 获取群组成员列表
const fetchGroupMembers = async (groupIds: number[]): Promise<DeboxGroupMember[]> => {
  const data: GetDeboxGroupMembersReq = {
    all: true,
    group_ids: groupIds,
    // status: true,
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

const GroupMemberList: FC<GroupListProps> = ({ accountId, groupIds, groupsMemberUpdateState }) => {
  const [groupMembers, setGroupMembers] = useState<DeboxGroupMember[]>([]);
  const [loading, setLoading] = useState(false);
  const [accountNameMap, setAccountNameMap] = useState<{ [key: number]: string }>({});
  const [groupNameMap, setGroupNameMap] = useState<{ [key: number]: string }>({});

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        const accountResponse = await fetchAccounts();
        const accountNameMap = accountResponse.reduce(
          (acc, cur) => {
            acc[cur.id] = cur.name;
            return acc;
          },
          {} as { [key: number]: string },
        );
        setAccountNameMap(accountNameMap);

        const groupResponse = await fetchGroups();
        const groupNameMap = groupResponse.reduce(
          (acc, cur) => {
            acc[cur.id] = cur.name;
            return acc;
          },
          {} as { [key: number]: string },
        );
        setGroupNameMap(groupNameMap);
      } catch (err) {
        console.error(err);
      }
    };

    loadAccounts();
  }, []);

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

  if (accountId === 0) {
    return <Empty title='请先选择账号' description='暂无数据' />;
  }

  if (groupIds.length === 0) {
    return <Empty title='请先选择群组' description='暂无数据' />;
  }

  if (groupMembers.length === 0) {
    return <Empty title='暂无数据' />;
  }

  return (
    <>
      <List className={styles.groupList}>
        {groupMembers.map((groupMember) => (
          <List.Item
            key={groupMember.id}
            prefix={<Avatar className={styles.groupAvatar} src={groupMember.pic ?? ''} />}
            description={`${accountNameMap[groupMember.account_id]}/${groupNameMap[groupMember.group_id]}`}
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
