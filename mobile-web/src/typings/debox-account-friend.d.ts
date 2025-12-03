/*用户好友管理 */
import { FriendType } from '@/enums/debox-account-friend';

/// DeBox账号好友表
export interface DeboxAccountFriend {
  id: number; // 好友ID
  user_id: number; // 用户ID
  account_id: number; // 账号ID
  debox_user_id: string; // DeBox 用户ID
  name: string; // 用户名称
  avatar: string; // 账号头像
  desc: string; // 描述信息
  status: boolean; // 状态
  created_at: string; // 创建时间
  updated_at: string; // 更新时间
}

// 查询DeBox账号好友列表 请求体
export interface GetDeboxAccountFriendsReq {
  page?: number; // 当前分页
  page_size?: number; // 页面大小
  start_time?: string; // 开始时间
  end_time?: string; // 结束时间
  all: boolean; // 返回所有数据
  account_ids: number[]; // 账号IDs
  status: boolean; // 账号状态
  sorts?: string[]; // 排序字段, ["id:asc"]
}

// 查询DeBox账号好友列表 响应体
export interface GetDeboxAccountFriendsResp {
  data_list: DeboxAccountFriend[];
  total: number;
}

// 查询DeBox账号好友信息 请求体
export interface GetDeboxAccountFriendReq {
  id: number; // 账号好友ID
}

// 查询DeBox账号好友信息 响应体
export type GetDeboxAccountFriendResp = DeboxAccountFriend;

// 添加DeBox账号好友 请求体
export interface CreateDeboxAccountFriendReq {
  account_id: number; // 账号ID
  debox_user_id: string; // DeBox 用户ID
  name: string; // 好友名称
  avatar: string; // 账号头像
  desc: string; // 描述信息
  status: boolean; // 状态
}

// 添加DeBox账号好友 响应体
export interface CreateDeboxAccountFriendResp {}

// 更新DeBox账号好友信息 请求体
export interface UpdateDeboxAccountFriendReq {
  id: number; // 账号好友ID
  name: string; // 好友名称
  avatar: string; // 账号头像
  desc: string; // 描述信息
  status: boolean; // 状态
}

// 更新DeBox账号好友信息 响应体
export interface UpdateDeboxAccountFriendResp {}

// 更新DeBox账号好友状态 请求体
export interface UpdateDeboxAccountFriendStatusReq {
  id: number; // 账号好友ID
  status: boolean; // 状态
}

// 更新DeBox账号好友状态 响应体
export interface UpdateDeboxAccountFriendStatusResp {}

// 删除DeBox账号好友 请求体
export interface DeleteDeboxAccountFriendReq {
  id: number; // 账号好友ID
}

// 删除DeBox账号好友 响应体
export interface DeleteDeboxAccountFriendResp {}

// 同步好友列表 请求体
export interface SyncDeboxAccountFriendsReq {
  account_ids: number[]; // 账号IDs
}

// 同步好友列表 响应体
export interface SyncDeboxAccountFriendsResp {}

// 发送私聊文本消息 请求体
export interface SendPrivateMessageTextReq {
  account_id: number; // 账号ID
  debox_user_ids: string[]; // Debox用户ID列表
  content: string; // 消息内容
}

// 发送私聊文本消息 响应体
export interface SendPrivateMessageTextResp {}
