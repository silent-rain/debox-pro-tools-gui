//! DeBox账号好友管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::debox::debox_account_friend;

/// 查询DeBox账号好友列表 请求体
#[derive(Default, Deserialize, Validate)]
#[serde(default)]
pub struct GetDeboxAccountFriendsReq {
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
    /// 账号好友状态
    pub status: Option<bool>,
    /// 排序字段
    /// sorts: ["id:asc"]
    pub sorts: Option<Vec<String>>,
}

/// 排序字段
pub struct DeboxAccountFriendSort(pub debox_account_friend::Column, pub sea_orm::Order);

impl TryFrom<String> for DeboxAccountFriendSort {
    type Error = sea_orm::DbErr;

    // sort: "id:asc"
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() != 2 {
            return Err(sea_orm::DbErr::Custom("Invalid sort format".to_string()));
        }
        let column = match parts[0] {
            "user_id" => debox_account_friend::Column::UserId,
            "created_at" => debox_account_friend::Column::CreatedAt,
            "updated_at" => debox_account_friend::Column::UpdatedAt,
            _ => debox_account_friend::Column::Id,
        };
        let order = match parts[1] {
            "asc" => sea_orm::Order::Asc,
            "desc" => sea_orm::Order::Desc,
            _ => sea_orm::Order::Asc,
        };
        Ok(DeboxAccountFriendSort(column, order))
    }
}

/// 查询DeBox账号好友列表 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeboxAccountFriendsResp {
    pub data_list: Vec<debox_account_friend::Model>,
    pub total: u64,
}

impl From<(Vec<debox_account_friend::Model>, u64)> for GetDeboxAccountFriendsResp {
    fn from((data_list, total): (Vec<debox_account_friend::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 查询DeBox账号好友信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetDeboxAccountFriendReq {
    /// 账号好友ID
    pub id: i32,
}

/// 查询DeBox账号好友信息 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetDeboxAccountFriendResp {
    #[serde(flatten)]
    model: debox_account_friend::Model,
}

impl From<debox_account_friend::Model> for GetDeboxAccountFriendResp {
    fn from(model: debox_account_friend::Model) -> Self {
        Self { model }
    }
}

/// 添加DeBox账号好友 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
#[serde(default)]
pub struct CreateDeboxAccountFriendReq {
    /// 账号ID
    pub account_id: i32,
    /// DeBox 用户ID
    pub debox_user_id: String,
    /// 好友名称
    pub name: String,
    /// 账号头像
    pub avatar: Option<String>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 添加DeBox账号好友 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeboxAccountFriendResp {}

/// 更新DeBox账号好友信息 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxAccountFriendReq {
    /// 账号好友ID
    pub id: i32,
    /// 好友名称
    pub name: String,
    /// 账号头像
    pub avatar: Option<String>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 更新DeBox账号好友信息 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxAccountFriendResp {}

/// 更新DeBox账号好友状态 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxAccountFriendStatusReq {
    /// 账号好友ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 更新DeBox账号好友状态 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxAccountFriendStatusResp {}

/// 删除DeBox账号好友 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteDeboxAccountFriendReq {
    /// 账号好友ID
    pub id: i32,
}

/// 删除DeBox账号好友 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDeboxAccountFriendResp {}

/// 同步好友列表 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct SyncDeboxAccountFriendsReq {
    /// 账号ID
    pub account_ids: Vec<i32>,
}

/// 同步好友列表 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncDeboxAccountFriendsResp {}

/// 发送私聊文本消息 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct SendPrivateMessageTextReq {
    /// 账号ID
    pub account_id: i32,
    /// 好友Debox用户ID
    pub debox_user_ids: Vec<String>,
    /// 消息内容    
    pub content: String,
}

/// 发送私聊文本消息 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct SendPrivateMessageTextResp {}
