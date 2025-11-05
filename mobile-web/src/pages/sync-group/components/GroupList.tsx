import { FC, useEffect, useState } from 'react';
import { Avatar, DotLoading, List, Switch } from 'antd-mobile';
import { CheckOutline, CloseOutline } from 'antd-mobile-icons';
import styles from './GroupList.module.less';
import { DeboxGroup, GetDeboxGroupsReq } from '@/typings/debox-group';
import { DeboxGroupApi } from '@/api/debox-group';

interface GroupListProps {
  accountIds: number[];
}

// 获取群组列表
const fetchGroups = async (accountIds: number[]): Promise<DeboxGroup[]> => {
  const data: GetDeboxGroupsReq = {
    page: 0,
    page_size: 0,
    all: true,
    start_time: '',
    end_time: '',
    account_ids: accountIds,
    group_name: '',
  };
  const response = await DeboxGroupApi.list(data);
  return response.data_list;
};

const GroupList: FC<GroupListProps> = ({ accountIds }) => {
  const [groups, setGroups] = useState<DeboxGroup[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setLoading(true);
        // const data = await fetchGroups(accountIds);
        // 模拟数据
        const data: DeboxGroup[] = [
          {
            id: 1,
            group_name: '群组1',
            pic: 'https://picsum.photos/200/300',
            status: true,
            account_id: 0,
            url: '',
            group_code: '',
            desc: '',
            created_at: '',
            updated_at: '',
          },
          {
            id: 2,
            group_name: '群组2',
            pic: 'https://picsum.photos/200/300',
            status: false,
            account_id: 0,
            url: '',
            group_code: '',
            desc: '',
            created_at: '',
            updated_at: '',
          },
        ];

        setGroups(data);
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [accountIds]);

  const handleSwitchChange = async (checked: boolean, groupId: number) => {
    console.log('handleSwitchChange:', checked, groupId);

    const data = await fetchGroups(accountIds);
    setGroups(data);
  };

  if (loading) {
    return <DotLoading color='primary' />;
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
                onChange={() => handleSwitchChange(group.status, group.id)}
              />
            }
          >
            {group.group_name}
          </List.Item>
        ))}
      </List>
    </>
  );
};

export default GroupList;
