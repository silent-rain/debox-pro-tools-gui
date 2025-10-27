//! Auth APIs
import request from '@/utils/request';
import { server } from './constant';
import { LoginReq, LoginResp, RegisterReq, RegisterResp } from '@/typings/auth';

export const AuthApi = {
  // 登陆
  login: async (data: LoginReq): Promise<LoginResp> => {
    const response = await request({
      url: `${server}/auth/login`,
      method: 'POST',
      data,
    });
    return response.data;
  },

  // 注册用户
  register: async (data: RegisterReq): Promise<RegisterResp> => {
    const response = await request({
      url: `${server}/auth/register`,
      method: 'POST',
      data,
    });
    return response.data;
  },
};
