//! DeBox账号关注人管理

use debox_pro_rs::dto::user_ext::UserSearch;
use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::debox::debox_account_follow;

use crate::enums::debox_account_follow::FollowType;

/// 查询DeBox账号关注人列表 请求体
#[derive(Default, Deserialize, Validate)]
#[serde(default)]
pub struct GetDeboxAccountFollowsReq {
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
    /// 账号关注人状态
    pub status: Option<bool>,
    /// 排序字段
    /// sorts: ["id:asc"]
    pub sorts: Option<Vec<String>>,
}

/// 排序字段
pub struct DeboxAccountFollowSort(pub debox_account_follow::Column, pub sea_orm::Order);

impl TryFrom<String> for DeboxAccountFollowSort {
    type Error = sea_orm::DbErr;

    // sort: "id:asc"
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() != 2 {
            return Err(sea_orm::DbErr::Custom("Invalid sort format".to_string()));
        }
        let column = match parts[0] {
            "user_id" => debox_account_follow::Column::UserId,
            "created_at" => debox_account_follow::Column::CreatedAt,
            "updated_at" => debox_account_follow::Column::UpdatedAt,
            _ => debox_account_follow::Column::Id,
        };
        let order = match parts[1] {
            "asc" => sea_orm::Order::Asc,
            "desc" => sea_orm::Order::Desc,
            _ => sea_orm::Order::Asc,
        };
        Ok(DeboxAccountFollowSort(column, order))
    }
}

/// 查询DeBox账号关注人列表 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeboxAccountFollowsResp {
    pub data_list: Vec<debox_account_follow::Model>,
    pub total: u64,
}

impl From<(Vec<debox_account_follow::Model>, u64)> for GetDeboxAccountFollowsResp {
    fn from((data_list, total): (Vec<debox_account_follow::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 查询DeBox账号关注人信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetDeboxAccountFollowReq {
    /// 账号关注人ID
    pub id: i32,
}

/// 查询DeBox账号关注人信息 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetDeboxAccountFollowResp {
    #[serde(flatten)]
    model: debox_account_follow::Model,
}

impl From<debox_account_follow::Model> for GetDeboxAccountFollowResp {
    fn from(model: debox_account_follow::Model) -> Self {
        Self { model }
    }
}

/// 添加DeBox账号关注人 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
#[serde(default)]
pub struct CreateDeboxAccountFollowReq {
    /// 账号ID
    pub account_id: i32,
    /// DeBox 用户ID
    pub debox_user_id: String,
    /// 关注人名称
    pub name: String,
    /// 账号头像
    pub avatar: Option<String>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 添加DeBox账号关注人 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeboxAccountFollowResp {}

/// 更新DeBox账号关注人信息 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxAccountFollowReq {
    /// 账号关注人ID
    pub id: i32,
    /// 关注人名称
    pub name: String,
    /// 账号头像
    pub avatar: Option<String>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 更新DeBox账号关注人信息 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxAccountFollowResp {}

/// 更新DeBox账号关注人状态 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxAccountFollowStatusReq {
    /// 账号关注人ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 更新DeBox账号关注人状态 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxAccountFollowStatusResp {}

/// 删除DeBox账号关注人 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteDeboxAccountFollowReq {
    /// 账号关注人ID
    pub id: i32,
}

/// 删除DeBox账号关注人 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDeboxAccountFollowResp {}

/// 同步关注人列表 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct SyncDeboxAccountFollowsReq {
    /// 账号ID
    pub account_ids: Vec<i32>,
}

/// 同步关注人列表 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncDeboxAccountFollowsResp {}

/// 批量关注用户 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
#[serde(default)]
pub struct BatchAccountFollowsReq {
    /// 账号ID
    pub account_id: i32,
    /// 目标账号ID
    pub target_account_id: i32,
    /// 目标群组ID
    pub target_group_id: i32,
    /// DeBox用户IDs
    pub debox_user_ids: Vec<String>,
    /// 添加关注人账号类型
    pub follow_type: FollowType,
}

/// 批量关注用户 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchAccountFollowsResp {}

/// 用户搜索 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
#[serde(default)]
pub struct DeboxUserSearchReq {
    /// 账号ID
    pub account_id: i32,
    /// 搜索关键词
    pub search: String,
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub size: u64,
}

/// 用户搜索 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct DeboxUserSearchResp {
    pub data_list: Vec<UserSearch>,
}

impl From<Vec<UserSearch>> for DeboxUserSearchResp {
    fn from(data_list: Vec<UserSearch>) -> Self {
        Self { data_list }
    }
}
