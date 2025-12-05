// Debox Group Member

// DeBox群组成员表
export interface DeboxGroupMember {
  id: number; // 群组成员ID
  user_id: number; // 用户ID
  account_id: number; // 账号ID
  group_id: number; // 群组ID
  group_gid: string; // Debox群组ID
  debox_user_id: string; // Debox用户ID
  address: string; // 钱包地址
  name: string; // 成员名称
  pic: string; // 成员头像
  desc: string; // 描述信息
  status: boolean; // 状态
  created_at: string; // 创建时间
  updated_at: string; // 更新时间
}

// 查询DeBox群组成员列表 请求体
export interface GetDeboxGroupMembersReq {
  page?: number; // 当前分页
  page_size?: number; // 页面大小
  start_time?: string; // 开始时间
  end_time?: string; // 结束时间
  all?: boolean; // 返回所有数据
  status?: boolean; // 成员状态
  account_ids?: number[]; // 账号IDs
  group_ids: number[]; // 群组IDs
  name?: string; // 成员名称
}

// 查询DeBox群组成员列表 响应体
export interface GetDeboxGroupMembersResp {
  data_list: DeboxGroupMemberMember[];
  total: number;
}

// 查询DeBox群组成员信息 请求体
export interface GetDeboxGroupMemberReq {
  id: number; // 成员ID
}

// 查询DeBox群组成员信息 响应体
export type GetDeboxGroupMemberResp = DeboxGroupMemberMember;

// 更新数据状态  请求体
export interface UpdateDeboxGroupMemberStatusReq {
  id: number; // 成员ID
  status: boolean; // 状态(false:停用,true:正常)
}

// 添加DeBox群组成员 请求体
export type CreateDeboxGroupMemberReq = DeboxGroupMemberMember;

// 添加DeBox群组成员 响应体
export interface CreateDeboxGroupMemberResp {}

// 更新DeBox群组成员 请求体
export type UpdateDeboxGroupMemberReq = DeboxGroupMemberMember;

// 更新DeBox群组成员 响应体
export interface UpdateDeboxGroupMemberResp {}

// 更新数据状态 响应体
export interface UpdateDeboxGroupMemberStatusResp {}

// 删除数据 请求体
export interface DeleteDeboxGroupMemberReq {
  id: number; // 成员ID
}

// 删除数据 响应体
export interface DeleteDeboxGroupMemberResp {}

// 同步DeBox群组成员列表  请求体
export interface SyncDeboxGroupMemberReq {
  group_ids: number[]; // 群组IDs
}

export interface SyncDeboxGroupMemberResp {}

// 添加DeBox群组成员 请求体
export interface AddDeboxGroupMemberReq {
  account_id: number; // 账号ID
  group_id: number; // 群组ID
  debox_user_ids: string[]; // Debox用户ID列表
}

// 添加DeBox群组成员 响应体
export interface AddDeboxGroupMemberResp {}
