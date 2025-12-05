import { FC, useState } from 'react';
import { Form, Input, Modal, Toast } from 'antd-mobile';
import { DeboxGroupApi } from '@/api';
import styles from './CreateGroup.module.scss';

interface CreateGroupProps {
  visible: boolean;
  accountId: number;
  deboxUserIds: string[];
  onClose: () => void;
  onSubmit: () => void;
}

const CreateGroup: FC<CreateGroupProps> = ({ visible, accountId, deboxUserIds, onClose, onSubmit }) => {
  const [groupName, setGroupName] = useState('');

  // 提交新建群组
  const handleCreateGroup = async () => {
    if (!groupName.trim()) {
      Toast.show('请输入群组名称');
      return;
    }

    try {
      await DeboxGroupApi.createSubgroup({
        account_id: accountId,
        group_name: groupName.trim(),
        debox_user_ids: deboxUserIds,
      });
      setGroupName('');
      onSubmit();
      onClose();
      Toast.show('群组创建成功');
    } catch (err) {
      console.error('创建群组失败:', err);
      Toast.show('创建群组失败');
    }
  };

  return (
    <div className='create-group'>
      {/* 新建群组弹窗 */}
      <Modal
        title='新建群组'
        visible={visible}
        showCloseButton
        onClose={() => {
          onClose();
          setGroupName('');
        }}
        content={
          <Form layout='vertical'>
            <Form.Item label='群组名称' required>
              <Input placeholder='请输入群组名称' value={groupName} onChange={(val) => setGroupName(val)} />
            </Form.Item>
            <div className={styles.groupMember}>将添加 {deboxUserIds.length} 个成员到新群组</div>
          </Form>
        }
        actions={[
          {
            key: 'confirm',
            text: '提交',
            primary: true,
            onClick: handleCreateGroup,
          },
        ]}
      />
    </div>
  );
};

export default CreateGroup;
