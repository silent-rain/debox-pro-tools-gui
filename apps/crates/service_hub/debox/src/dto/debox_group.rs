//! DeBox群组管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::debox::debox_group;

/// 查询DeBox群组列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetDeboxGroupsReq {
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
    /// 账号ID
    pub account_id: Option<i32>,
    /// 群组名称
    pub group_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeboxGroupsResp {
    pub data_list: Vec<debox_group::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetDeboxGroupReq {
    /// DeBox群组ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeboxGroupResp {
    #[serde(flatten)]
    data: debox_group::Model,
}

/// 添加DeBox群组 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateDeboxGroupReq {
    /// 账号ID
    pub account_id: i32,
    /// 群组分享链接
    pub url: String,
    /// 群组名称
    pub group_name: String,
    /// 群组邀请码
    pub group_code: String,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeboxGroupResp {}

/// 更新数据 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxGroupReq {
    /// DeBox群组ID
    pub id: i32,
    /// 账号ID
    pub account_id: i32,
    /// 群组分享链接
    pub url: String,
    /// 群组名称
    pub group_name: String,
    /// 群组邀请码
    pub group_code: String,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxGroupResp {}

/// 更新数据状态 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxGroupStatusReq {
    /// DeBox群组ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 更新数据状态 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxGroupStatusResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteDeboxGroupReq {
    /// DeBox群组ID
    pub id: i32,
}

/// 删除数据 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDeboxGroupResp {}
