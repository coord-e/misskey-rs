pub mod abuse_user_reports;
pub mod accounts;
pub mod announcements;
pub mod emoji;
pub mod get_table_stats;
pub mod reset_password;
pub mod server_info;
pub mod show_moderation_logs;
pub mod show_user;
pub mod show_users;
pub mod suspend_user;
pub mod unsuspend_user;
pub mod update_meta;

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

#[cfg(feature = "12-81-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-81-0")))]
pub mod get_index_stats;

#[cfg(not(feature = "12-93-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-93-0"))))]
pub mod delete_logs;

#[cfg(not(feature = "12-93-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-93-0"))))]
pub mod logs;

#[cfg(not(feature = "12-106-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-106-0"))))]
pub mod resync_chart;

#[cfg(feature = "12-109-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-109-0")))]
pub mod meta;

#[cfg(feature = "12-112-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
pub mod delete_account;

#[cfg(feature = "12-112-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
pub mod get_user_ips;

#[cfg(feature = "12-112-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
pub mod update_user_note;

#[cfg(all(feature = "12-112-0", not(feature = "13-0-0")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "12-112-0", not(feature = "13-0-0")))))]
pub mod drive_capacity_override;

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
pub mod roles;

#[cfg(not(feature = "13-0-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
pub mod invite;

#[cfg(not(feature = "13-0-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
pub mod moderators;

#[cfg(not(feature = "13-0-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
pub mod silence_user;

#[cfg(not(feature = "13-0-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
pub mod unsilence_user;

#[cfg(not(feature = "13-0-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
pub mod vacuum;
