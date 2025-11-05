// DeBox群组管理

import request from '@/utils/request';
import { server } from './constant';
import {} from '@/typings/debox-account';
import { GetDeboxGroupsReq, GetDeboxGroupsResp } from '@/typings/debox-group';

export const DeboxGroupApi = {
  // 获取群组列表
  list: async (data: GetDeboxGroupsReq): Promise<GetDeboxGroupsResp> => {
    const response = await request({
      url: `${server}/debox/debox-groups`,
      method: 'GET',
      params: data,
    });
    return response.data;
  },
};
