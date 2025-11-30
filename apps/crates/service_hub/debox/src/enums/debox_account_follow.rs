//! DeBox账号关注人管理

use serde_repr::{Deserialize_repr, Serialize_repr};

/// 添加关注人类型
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum FollowType {
    /// 账号
    #[default]
    Account,
    /// 群组
    Group,
    /// 用户
    User,
}
