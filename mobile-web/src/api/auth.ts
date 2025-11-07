//! Auth APIs
import request from '@/utils/request';
import { SERVER } from '@/constants/http';
import { LoginReq, LoginResp, RegisterReq, RegisterResp } from '@/typings/auth';

export const AuthApi = {
  // 登陆
  login: async (data: LoginReq): Promise<LoginResp> => {
    const response = await request({
      url: `${SERVER}/auth/login`,
      method: 'POST',
      data,
    });
    return response.data;
  },

  // 注册用户
  register: async (data: RegisterReq): Promise<RegisterResp> => {
    const response = await request({
      url: `${SERVER}/auth/register`,
      method: 'POST',
      data,
    });
    return response.data;
  },
};
