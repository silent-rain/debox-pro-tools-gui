import { Card, Dropdown, Button, List } from 'antd-mobile';
import { useState } from 'react';
import './index.module.less';

const GroupManagement = () => {
  const [selectedAccount, setSelectedAccount] = useState<string>('all');
  const [selectedGroups, setSelectedGroups] = useState<string[]>([]);

  const accounts = [
    { label: '所有账户', value: 'all' },
    { label: '账户1', value: 'account1' },
    { label: '账户2', value: 'account2' },
  ];

  const groups = [
    { id: '1', name: '群组1' },
    { id: '2', name: '群组2' },
    { id: '3', name: '群组3' },
  ];

  const handleSyncGroups = () => {
    console.log('同步群组:', selectedGroups);
  };

  const toggleGroupSelection = (groupId: string) => {
    setSelectedGroups((prev) => (prev.includes(groupId) ? prev.filter((id) => id !== groupId) : [...prev, groupId]));
  };

  return (
    <div className='group-management'>
      <Dropdown
        options={accounts}
        defaultValue={selectedAccount}
        onChange={(value) => setSelectedAccount(value as string)}
      />

      <Button onClick={handleSyncGroups} disabled={selectedGroups.length === 0}>
        同步群组
      </Button>

      <List>
        {groups.map((group) => (
          <List.Item
            key={group.id}
            extra={
              <>
                <Button size='small' onClick={() => toggleGroupSelection(group.id)}>
                  {selectedGroups.includes(group.id) ? '取消同步' : '同步'}
                </Button>
                <Button size='small' danger>
                  禁用
                </Button>
              </>
            }
          >
            {group.name}
          </List.Item>
        ))}
      </List>
    </div>
  );
};

export default GroupManagement;
