/*用户关注管理 */
import { FollowType } from '@/enums/debox-account-follow';

/// DeBox账号关注表
export interface DeboxAccountFollow {
  id: number; // 关注ID
  user_id: number; // 用户ID
  account_id: number; // 账号ID
  debox_user_id: string; // DeBox 用户ID
  invite_code: string; // 邀请码
  name: string; // 用户名称
  avatar: string; // 账号头像
  desc: string; // 描述信息
  status: boolean; // 状态
  created_at: string; // 创建时间
  updated_at: string; // 更新时间
}

// 查询DeBox账号列表 请求体
export interface GetDeboxAccountFollowsReq {
  page?: number; // 当前分页
  page_size?: number; // 页面大小
  start_time?: string; // 开始时间
  end_time?: string; // 结束时间
  all: boolean; // 返回所有数据
  account_ids: number[]; // 账号IDs
  status: boolean; // 账号状态
  sorts?: string[]; // 排序字段, ["id:asc"]
}

// 查询DeBox账号列表 响应体
export interface GetDeboxAccountFollowsResp {
  data_list: DeboxAccountFollow[];
  total: number;
}

// 查询DeBox账号关注人信息 请求体
export interface GetDeboxAccountFollowReq {
  id: number; // 账号关注人ID
}

// 查询DeBox账号关注人信息 响应体
export type GetDeboxAccountFollowResp = DeboxAccountFollow;

// 添加DeBox账号关注人 请求体
export interface CreateDeboxAccountFollowReq {
  account_id: number; // 账号ID
  debox_user_id: string; // DeBox 用户ID
  invite_code: string; // 邀请码
  name: string; // 关注人名称
  avatar: string; // 账号头像
  desc: string; // 描述信息
  status: boolean; // 状态
}

// 添加DeBox账号关注人 响应体
export interface CreateDeboxAccountFollowResp {}

// 更新DeBox账号关注人信息 请求体
export interface UpdateDeboxAccountFollowReq {
  id: number; // 账号关注人ID
  name: string; // 关注人名称
  avatar: string; // 账号头像
  desc: string; // 描述信息
  status: boolean; // 状态
}

// 更新DeBox账号关注人信息 响应体
export interface UpdateDeboxAccountFollowResp {}

// 更新DeBox账号关注人状态 请求体
export interface UpdateDeboxAccountFollowStatusReq {
  id: number; // 账号关注人ID
  status: boolean; // 状态
}

// 更新DeBox账号关注人状态 响应体
export interface UpdateDeboxAccountFollowStatusResp {}

// 删除DeBox账号关注人 请求体
export interface DeleteDeboxAccountFollowReq {
  id: number; // 账号关注人ID
}

// 删除DeBox账号关注人 响应体
export interface DeleteDeboxAccountFollowResp {}

// 同步关注人列表 请求体
export interface SyncDeboxAccountFollowsReq {
  account_ids: number[]; // 账号IDs
}

// 同步关注人列表 响应体
export interface SyncDeboxAccountFollowsResp {}

// 批量关注用户 请求体
export interface BatchAccountFollowsReq {
  account_id: number; // 账号ID
  target_account_id?: number; // 目标账号ID
  target_group_id?: number; // 目标群组ID
  debox_user_ids?: string[]; // DeBox用户IDs
  follow_type: FollowType; // 添加关注人账号类型
}

// 批量关注用户 响应体
export interface BatchAccountFollowsResp {}

// 用户搜索 请求体
export interface DeboxUserSearchReq {
  account_id: number; // 账号ID
  search: string; // 搜索关键词
  page: number; // 当前分页
  size: number; // 页面大小
}

// 用户搜索 响应体
export interface DeboxUserSearchResp {
  data_list: UserSearch[];
}

// 用户搜索结果
export interface UserSearch {
  user_id: number;
  name: string;
  group_alias: string;
  alias_name: string;
  pic: string;
  chain_id: number;
  address: string;
  solana_address: string;
  tron_address: string;
  colors?: any;
  is_admin: number;
  is_builder: number;
  is_founder: number;
  is_role: number;
  identity?: any;
  user_label?: any;
  ext_text: string;
  icons: any[];
}
