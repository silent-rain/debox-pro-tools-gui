//! DeBox账号管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::system::config;

/// 查询DeBox账号列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetDeboxAccountsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// DeBox账号名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeboxAccountsResp {
    pub data_list: Vec<config::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetDeboxAccountReq {
    /// DeBox账号ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeboxAccountResp {
    #[serde(flatten)]
    data: config::Model,
}

/// 添加DeBox账号 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateDeboxAccountReq {
    /// 父节点ID
    pub pid: Option<i32>,
    /// DeBox账号名称
    pub name: String,
    /// DeBox账号编码(英文)
    pub code: String,
    /// DeBox账号值
    pub value: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// DeBox账号描述
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeboxAccountResp {}

/// 更新数据 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxAccountReq {
    /// DeBox账号ID
    pub id: i32,
    /// 父节点ID
    pub pid: Option<i32>,
    /// DeBox账号名称
    pub name: String,
    /// DeBox账号编码(英文)
    pub code: String,
    /// DeBox账号值
    pub value: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// DeBox账号描述
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxAccountResp {}

/// 更新数据状态 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxAccountStatusReq {
    /// DeBox账号ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxAccountStatusResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteDeboxAccountReq {
    /// DeBox账号ID
    pub id: i32,
}
