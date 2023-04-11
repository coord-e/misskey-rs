pub mod abuse_user_reports;
pub mod accounts;
pub mod announcements;
pub mod delete_logs;
pub mod emoji;
pub mod get_table_stats;
pub mod invite;
pub mod logs;
pub mod moderators;
pub mod reset_password;
pub mod resync_chart;
pub mod server_info;
pub mod show_moderation_logs;
pub mod show_user;
pub mod show_users;
pub mod silence_user;
pub mod suspend_user;
pub mod unsilence_user;
pub mod unsuspend_user;
pub mod update_meta;
pub mod vacuum;

#[cfg(feature = "12-13-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-13-0")))]
pub mod promo;

#[cfg(any(docsrs, not(feature = "12-49-0")))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-49-0"))))]
pub mod remove_abuse_user_report;

#[cfg(feature = "12-49-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-49-0")))]
pub mod resolve_abuse_user_report;

#[cfg(feature = "12-80-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
pub mod ad;
