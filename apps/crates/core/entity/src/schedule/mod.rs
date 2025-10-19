//! 任务调度相关表
pub mod schedule_event_log;
pub mod schedule_job;
pub mod schedule_status_log;

pub use schedule_event_log::Entity as ScheduleEventLogEntity;
pub use schedule_job::Entity as ScheduleJobEntity;
pub use schedule_status_log::Entity as ScheduleStatusLogEntity;
