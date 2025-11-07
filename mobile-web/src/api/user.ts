//! User APIs
import request from '@/utils/request';
import { SERVER } from '@/constants/http';
import { ProfileResp } from '@/typings/user';

export const UserApi = {
  // 获取用户个人信息
  profile: async (): Promise<ProfileResp> => {
    const response = await request({
      url: `${SERVER}/user/base/profile`,
      method: 'GET',
      params: {},
    });
    return response.data;
  },
};
