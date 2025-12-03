//! DeBox 相关实体定义
pub mod debox_account;
pub mod debox_account_follow;
pub mod debox_account_friend;
pub mod debox_group;
pub mod debox_group_member;

pub use debox_account::Entity as DeboxAccount;
pub use debox_account_follow::Entity as DeboxAccountFollow;
pub use debox_account_friend::Entity as DeboxAccountFriend;
pub use debox_group::Entity as DeboxGroup;
pub use debox_group_member::Entity as DeboxGroupMember;
