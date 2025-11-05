// DeBox群组管理

import request from '@/utils/request';
import { server } from './constant';
import {} from '@/typings/debox-account';
import {
  GetDeboxGroupsReq,
  GetDeboxGroupsResp,
  SyncDeboxGroupReq,
  SyncDeboxGroupResp,
  UpdateDeboxGroupStatusReq,
  UpdateDeboxGroupStatusResp,
} from '@/typings/debox-group';

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

  //  .route("/{id}/status", put(DeboxGroupController::update_status))

  // 更新群组状态
  updateStatus: async (data: UpdateDeboxGroupStatusReq): Promise<UpdateDeboxGroupStatusResp> => {
    const response = await request({
      url: `${server}/debox/debox-groups/status`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 同步DeBox群组列表
  syncGroups: async (data: SyncDeboxGroupReq): Promise<SyncDeboxGroupResp> => {
    const response = await request({
      url: `${server}/debox/debox-groups/sync-groups`,
      method: 'POST',
      data,
    });
    return response.data;
  },
};
