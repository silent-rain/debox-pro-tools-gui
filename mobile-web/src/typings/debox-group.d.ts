// Debox Group

// DeBox群组表
export interface DeboxGroup {
  id: number; // 群组ID
  account_id: number; // 账号ID
  url: string; // 群组分享链接
  group_name: string; // 群组名称
  group_code: string; // 群组邀请码
  pic: string; // 群组头像
  desc: string; // 描述信息
  status: boolean; // 状态(false:停用,true:正常)
  created_at: string; // 创建时间
  updated_at: string; // 更新时间
}

// 查询DeBox群组列表 请求体
export interface GetDeboxGroupsReq {
  page: number; // 当前分页
  page_size: number; // 页面大小
  start_time: string; // 开始时间
  end_time: string; // 结束时间
  all: boolean; // 返回所有数据
  account_ids: number[]; // 账号ID
  group_name: string; // 群组名称
}

// 查询DeBox群组列表 响应体
export interface GetDeboxGroupsResp {
  data_list: DeboxGroup[];
  total: number;
}
