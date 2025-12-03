//! DeBox群组成员管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::debox::debox_group_member;

/// 查询DeBox群组成员列表 请求体
#[derive(Default, Deserialize, Validate)]
#[serde[default]]
pub struct GetDeboxGroupMembersReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
    /// 账号IDs
    pub account_ids: Option<Vec<i32>>,
    /// 群组IDs
    pub group_ids: Option<Vec<i32>>,
    /// 成员名称
    pub name: Option<String>,
    /// 成员状态
    pub status: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeboxGroupMembersResp {
    pub data_list: Vec<debox_group_member::Model>,
    pub total: u64,
}

impl From<(Vec<debox_group_member::Model>, u64)> for GetDeboxGroupMembersResp {
    fn from((data_list, total): (Vec<debox_group_member::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 查询DeBox群组成员详情 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetDeboxGroupMemberReq {
    /// 成员ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeboxGroupMemberResp {
    #[serde(flatten)]
    model: debox_group_member::Model,
}

impl From<debox_group_member::Model> for GetDeboxGroupMemberResp {
    fn from(model: debox_group_member::Model) -> Self {
        Self { model }
    }
}

/// 添加DeBox群组成员 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateDeboxGroupMemberReq {
    /// 账号ID
    pub account_id: i32,
    /// 群组ID
    pub group_id: i32,
    /// Debox群组ID
    pub group_gid: String,
    /// Debox用户ID
    pub debox_user_id: u64,
    /// 钱包地址
    pub address: String,
    /// 成员名称
    pub name: String,
    /// 成员头像
    pub pic: Option<String>,
    /// 是否管理员
    pub is_admin: bool,
    /// 是否构建者
    pub is_builder: bool,
    /// 是否创始人
    pub is_founder: bool,
    /// 是否角色
    pub is_role: bool,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 添加DeBox群组成员 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeboxGroupMemberResp {}

/// 更新数据 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxGroupMemberReq {
    /// 群组成员ID
    pub id: i32,
    /// 钱包地址
    pub address: String,
    /// 成员名称
    pub name: String,
    /// 成员头像
    pub pic: Option<String>,
    /// 是否管理员
    pub is_admin: bool,
    /// 是否构建者
    pub is_builder: bool,
    /// 是否创始人
    pub is_founder: bool,
    /// 是否角色
    pub is_role: bool,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 更新数据 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxGroupMemberResp {}

/// 更新数据状态 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxGroupMemberStatusReq {
    /// 成员ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 更新数据状态 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxGroupMemberStatusResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteDeboxGroupMemberReq {
    /// 成员ID
    pub id: i32,
}

/// 删除数据 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDeboxGroupMemberResp {}

/// 同步DeBox群组成员列表  请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct SyncDeboxGroupMemberReq {
    /// 群组IDs
    pub group_ids: Vec<i32>,
}

/// 同步DeBox群组成员列表  响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncDeboxGroupMemberResp {}

/// 添加DeBox群组成员 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct AddDeboxGroupMemberReq {
    /// 账号ID
    pub account_id: i32,
    /// 群组ID
    pub group_id: i32,
    /// Debox用户ID列表
    pub debox_user_ids: Vec<String>,
}
/// 添加DeBox群组成员 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct AddDeboxGroupMemberResp {}
