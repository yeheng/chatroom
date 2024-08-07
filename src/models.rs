use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppChatGroupEncryptHistory {
    pub id: i64,
    pub group_ids: String,
    pub group_names: Option<String>,
    pub content: String,
    pub encrypt_content: String,
    pub decrypt_time: DateTime,
    pub decrypt_pwd: Option<String>,
    pub decrypt_status: Option<i16>,
    pub from_user_id: i64,
    pub from_user_name: String,
    pub from_nick_name: String,
    pub from_user_type: String,
    pub from_user_avatar: Option<String>,
    pub from_level_id: Option<i64>,
    pub from_level_name: Option<String>,
    pub from_level_avatar: Option<String>,
    pub from_dept_id: Option<i64>,
    pub from_dept_name: Option<String>,
    pub from_dept_indx: Option<i16>,
    pub from_lecturer_name: Option<String>,
    pub from_lecturer_level: Option<i16>,
    pub from_lecturer_card_no: Option<String>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub status: i16,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppChatGroupHistory {
    pub id: i64,
    pub msg_type: String,
    pub content: String,
    pub length: Option<i64>,
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub group_id: i64,
    pub group_name: Option<String>,
    pub from_user_id: i64,
    pub from_user_name: String,
    pub from_nick_name: String,
    pub from_user_type: String,
    pub from_user_avatar: Option<String>,
    pub from_level_id: Option<i64>,
    pub from_level_name: Option<String>,
    pub from_level_avatar: Option<String>,
    pub from_dept_id: Option<i64>,
    pub from_dept_name: Option<String>,
    pub from_dept_indx: Option<i16>,
    pub from_lecturer_name: Option<String>,
    pub from_lecturer_level: Option<i16>,
    pub from_lecturer_card_no: Option<String>,
    pub at_user_id: Option<i64>,
    pub at_user_name: Option<String>,
    pub at_nick_name: Option<String>,
    pub at_user_type: Option<String>,
    pub at_user_avatar: Option<String>,
    pub at_level_id: Option<i64>,
    pub at_level_name: Option<String>,
    pub at_level_avatar: Option<String>,
    pub at_dept_id: Option<i64>,
    pub at_dept_name: Option<String>,
    pub at_dept_indx: Option<i16>,
    pub at_lecturer_name: Option<String>,
    pub at_lecturer_level: Option<i16>,
    pub at_lecturer_card_no: Option<String>,
    pub status: i16,
    pub client_ip: Option<String>,
    pub client_address: Option<String>,
    pub client_agent: Option<String>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
    pub ref_user_name: Option<String>,
    pub ref_nick_name: Option<String>,
    pub ref_msg_type: Option<String>,
    pub ref_content: Option<String>,
    pub ref_file_type: Option<String>,
    pub ref_file_name: Option<String>,
    pub encrypt_id: Option<i64>,
    pub encrypt_content: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppChatGroupHistoryDecryptRecord {
    pub user_id: i64,
    pub encrypt_id: i64,
    pub group_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppChatWhisperHistory {
    pub id: i64,
    pub msg_type: String,
    pub content: String,
    pub length: Option<i64>,
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub from_user_id: i64,
    pub from_user_name: String,
    pub from_nick_name: String,
    pub from_user_type: String,
    pub from_user_avatar: Option<String>,
    pub from_level_id: Option<i64>,
    pub from_level_name: Option<String>,
    pub from_level_avatar: Option<String>,
    pub from_dept_id: Option<i64>,
    pub from_dept_name: Option<String>,
    pub from_dept_indx: Option<i16>,
    pub from_lecturer_name: Option<String>,
    pub from_lecturer_level: Option<i16>,
    pub from_lecturer_card_no: Option<String>,
    pub to_user_id: i64,
    pub to_user_name: String,
    pub to_nick_name: String,
    pub to_user_type: String,
    pub to_user_avatar: Option<String>,
    pub to_level_id: Option<i64>,
    pub to_level_name: Option<String>,
    pub to_level_avatar: Option<String>,
    pub to_dept_id: Option<i64>,
    pub to_dept_name: Option<String>,
    pub to_dept_indx: Option<i16>,
    pub to_lecturer_name: Option<String>,
    pub to_lecturer_level: Option<i16>,
    pub to_lecturer_card_no: Option<String>,
    pub client_ip: Option<String>,
    pub client_address: Option<String>,
    pub client_agent: Option<String>,
    pub status: i16,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
    pub ref_user_name: Option<String>,
    pub ref_nick_name: Option<String>,
    pub ref_msg_type: Option<String>,
    pub ref_content: Option<String>,
    pub ref_file_type: Option<String>,
    pub ref_file_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppMoment {
    pub id: i64,
    pub topic: Option<String>,
    pub publisher_id: i64,
    pub publisher: String,
    pub content: String,
    pub like_counts: i64,
    pub comment_counts: i64,
    pub real_like_counts: i64,
    pub real_comment_counts: i64,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
    pub summary: Option<String>,
    pub img_urls: Option<String>,
    pub video_urls: Option<String>,
    pub is_top: Option<i16>,
    pub topping_time: Option<DateTime>,
    pub password: Option<String>,
    pub prev_duration: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppMomentComment {
    pub id: i64,
    pub moment_id: i64,
    pub publisher_id: i64,
    pub replier_id: i64,
    pub content: String,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppMomentLike {
    pub moment_id: i64,
    pub user_id: i64,
    pub publisher_id: i64,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppMomentLikeLog {
    pub id: i64,
    pub moment_id: i64,
    pub user_id: i64,
    pub publisher_id: i64,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppMomentsImage {
    pub id: i64,
    pub moment_id: i64,
    pub url: String,
    pub indx: i16,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppNew {
    pub id: i64,
    pub type_: i16,
    pub author: Option<String>,
    pub user_id: Option<i64>,
    pub title: String,
    pub tags: Option<String>,
    pub catalog_id: Option<i64>,
    pub catalog: Option<String>,
    pub image: Option<String>,
    pub summary: Option<String>,
    pub content: String,
    pub published: i16,
    pub publish_time: Option<DateTime>,
    pub like_counts: i64,
    pub comment_counts: i64,
    pub favorites_counts: i64,
    pub real_like_counts: i64,
    pub real_comment_counts: i64,
    pub real_favorites_counts: i64,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
    pub video_urls: Option<String>,
    pub img_urls: Option<String>,
    pub is_top: Option<i16>,
    pub topping_time: Option<DateTime>,
    pub password: Option<String>,
    pub prev_duration: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppNewsCatalog {
    pub id: i64,
    pub type_: i16,
    pub catalog: String,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppNewsComment {
    pub id: i64,
    pub news_id: i64,
    pub replier_id: i64,
    pub content: String,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppNewsFavorite {
    pub news_id: i64,
    pub user_id: i64,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppNewsFavoriteLikeLog {
    pub id: i64,
    pub type_: i16,
    pub news_id: i64,
    pub user_id: i64,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppNewsLike {
    pub news_id: i64,
    pub user_id: i64,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveChatHistory {
    pub id: i64,
    pub room_id: i64,
    pub room_name: Option<String>,
    pub msg_type: String,
    pub channel_code: Option<String>,
    pub ws_msg_type: Option<String>,
    pub content: String,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub type_: Option<i16>,
    pub avatar: Option<String>,
    pub parent_id: Option<i64>,
    pub parent_user_name: Option<String>,
    pub level_id: Option<i64>,
    pub level_name: Option<String>,
    pub level_avatar: Option<String>,
    pub dept_id: Option<i64>,
    pub dept_name: Option<String>,
    pub dept_indx: Option<i16>,
    pub cur_to_top: Option<String>,
    pub top_to_cur: Option<String>,
    pub lecturer_name: Option<String>,
    pub lecturer_level: Option<i16>,
    pub lecturer_card_no: Option<String>,
    pub client_ip: Option<String>,
    pub client_address: Option<String>,
    pub client_agent: Option<String>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub status: i16,
    pub del_flag: i16,
    pub at_id: Option<i64>,
    pub at_user_id: Option<i64>,
    pub at_user_name: Option<String>,
    pub at_nick_name: Option<String>,
    pub at_type: Option<i16>,
    pub at_avatar: Option<String>,
    pub at_parent_id: Option<i64>,
    pub at_parent_user_name: Option<String>,
    pub at_level_id: Option<i64>,
    pub at_level_name: Option<String>,
    pub at_level_avatar: Option<String>,
    pub at_dept_id: Option<i64>,
    pub at_dept_name: Option<String>,
    pub at_dept_indx: Option<i16>,
    pub at_cur_to_top: Option<String>,
    pub at_top_to_cur: Option<String>,
    pub at_lecturer_name: Option<String>,
    pub at_lecturer_level: Option<i16>,
    pub at_lecturer_card_no: Option<String>,
    pub at_content: Option<String>,
    pub real_user_id: Option<i64>,
    pub real_user_name: Option<String>,
    pub real_nick_name: Option<String>,
    pub real_type: Option<i16>,
    pub real_avatar: Option<String>,
    pub real_parent_id: Option<i64>,
    pub real_parent_user_name: Option<String>,
    pub real_level_id: Option<i64>,
    pub real_level_name: Option<String>,
    pub real_level_avatar: Option<String>,
    pub real_dept_id: Option<i64>,
    pub real_dept_name: Option<String>,
    pub real_cur_to_top: Option<String>,
    pub real_top_to_cur: Option<String>,
    pub real_lecturer_name: Option<String>,
    pub real_lecturer_level: Option<i16>,
    pub real_lecturer_card_no: Option<String>,
    pub as_type: Option<String>,
    pub msg_category: Option<i16>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveForbiddenSet {
    pub id: i64,
    pub room_id: Option<i64>,
    pub forbidden_type: i16,
    pub enable: i16,
    pub type_: Option<i16>,
    pub type_id: i64,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveMobileNavBar {
    pub id: i64,
    pub theme_id: i64,
    pub name: String,
    pub code: String,
    pub indx: i16,
    pub disable: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveNavBar {
    pub id: i64,
    pub theme_id: i64,
    pub name: String,
    pub target: i16,
    pub url: Option<String>,
    pub content: Option<String>,
    pub indx: i16,
    pub status: i16,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
    pub width: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRobotTimeSet {
    pub id: i64,
    pub room_id: i64,
    pub type_: i16,
    pub count: i32,
    pub apply_time: DateTime,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub status: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRoom {
    pub id: i64,
    pub domain: String,
    pub name: String,
    pub decs: Option<String>,
    pub channel_code: Option<String>,
    pub allow_speech: Option<i16>,
    pub status: i16,
    pub indx: i16,
    pub theme_id: Option<i64>,
    pub theme_name: Option<String>,
    pub need_pswd: Option<i16>,
    pub live_pswd: Option<String>,
    pub is_guest_login: i16,
    pub is_team_admin: i16,
    pub is_virtual_msg: i16,
    pub short_msgs: Option<String>,
    pub like_counts: Option<i64>,
    pub favorites_counts: Option<i64>,
    pub comment_counts: Option<i64>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
    pub remote_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRoomChapter {
    pub id: i64,
    pub room_id: Option<i64>,
    pub name: Option<String>,
    pub common: Option<String>,
    pub date: Option<DateTime>,
    pub status: i16,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRoomChapterItem {
    pub id: i64,
    pub chapter_id: i64,
    pub name: String,
    pub common: Option<String>,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub type_: Option<i16>,
    pub url: Option<String>,
    pub status: i16,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRoomChapterItemLecturer {
    pub chapter_item_id: i64,
    pub user_id: i64,
    pub indx: Option<i16>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRoomChapterLog {
    pub id: i64,
    pub room_id: i64,
    pub room_name: String,
    pub channel_id: i64,
    pub channel_code: String,
    pub channel_name: String,
    pub chapter_id: i64,
    pub chapter_name: String,
    pub chapter_item_id: Option<i64>,
    pub chapter_item_name: Option<String>,
    pub lecturer_id: Option<i64>,
    pub lecturer_code: Option<String>,
    pub lecturer_name: Option<String>,
    pub date: Option<DateTime>,
    pub begin_time: Option<DateTime>,
    pub end_time: Option<DateTime>,
    pub live_status: i16,
    pub push_flow_url: Option<String>,
    pub pull_flow_url: Option<String>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRoomLectureVote {
    pub id: i64,
    pub room_id: i64,
    pub user_id: i64,
    pub vote_user_id: i64,
    pub vote_count: i32,
    pub vote_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRoomLecturer {
    pub room_id: i64,
    pub user_id: i64,
    pub indx: Option<i16>,
    pub is_vote: i16,
    pub vote_count: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRoomStyle {
    pub id: i64,
    pub theme_id: i64,
    pub catalog: String,
    pub catalog_name: String,
    pub style_value: String,
    pub del_flag: i16,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRoomTheme {
    pub id: i64,
    pub name: String,
    pub layout: Option<String>,
    pub icon: Option<String>,
    pub cover_url: Option<String>,
    pub main_page_logo: Option<String>,
    pub rich_text: Option<i16>,
    pub background_image: Option<String>,
    pub notice_title: Option<String>,
    pub notice_content: Option<String>,
    pub notice_btn_label: Option<String>,
    pub warm_video: Option<String>,
    pub warm_img: Option<String>,
    pub login_page_logo: Option<String>,
    pub login_page_bgimage: Option<String>,
    pub swiper_content: Option<String>,
    pub swiper_speed: Option<i16>,
    pub del_flag: i16,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRoomUser {
    pub room_id: i64,
    pub user_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRoomUserFavorite {
    pub id: i64,
    pub room_id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub type_: i16,
    pub content: Option<String>,
    pub followed_id: Option<i64>,
    pub followed_user_id: Option<i64>,
    pub followed_user_name: Option<String>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveRoomWatchLog {
    pub id: i64,
    pub action_type: String,
    pub room_id: i64,
    pub room_name: String,
    pub channel_code: String,
    pub user_id: i64,
    pub user_name: String,
    pub nick_name: Option<String>,
    pub parent_id: Option<i64>,
    pub parent_user_name: Option<String>,
    pub type_: Option<i16>,
    pub level_id: Option<i64>,
    pub level_name: String,
    pub level_avatar: Option<String>,
    pub lecturer_name: Option<String>,
    pub lecturer_level: Option<i16>,
    pub lecturer_card_no: Option<String>,
    pub dept_id: Option<i64>,
    pub dept_name: Option<String>,
    pub dept_indx: Option<i16>,
    pub cur_to_top: Option<String>,
    pub top_to_cur: Option<String>,
    pub client_ip: Option<String>,
    pub client_address: Option<String>,
    pub client_agent: Option<String>,
    pub action: Option<String>,
    pub action_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveSensitiveWord {
    pub room_id: i64,
    pub sensitive_word: String,
    pub created_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveSlideshowAdv {
    pub id: i64,
    pub theme_id: i64,
    pub type_: Option<i16>,
    pub title: Option<String>,
    pub img: Option<String>,
    pub target: i16,
    pub url: Option<String>,
    pub content: Option<String>,
    pub status: i16,
    pub indx: i16,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaChannel {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub decs: Option<String>,
    pub image: Option<String>,
    pub live_status: Option<i16>,
    pub status: i16,
    pub indx: Option<i16>,
    pub push_flow_url: Option<String>,
    pub push_url_update_time: Option<DateTime>,
    pub pull_flow_url: Option<String>,
    pub pull_flow_mobile_url: Option<String>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaParam {
    pub id: i64,
    pub channel_id: Option<i64>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub decs: Option<String>,
    pub status: i16,
    pub indx: Option<i16>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaPushLog {
    pub id: i64,
    pub time: Option<String>,
    pub usrargs: Option<String>,
    pub action: String,
    pub app: String,
    pub appname: String,
    pub push_id: String,
    pub ip: Option<String>,
    pub node: Option<String>,
    pub height: Option<String>,
    pub width: Option<String>,
    pub created_time: DateTime,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysConfig {
    pub config_id: i64,
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    pub config_value: Option<String>,
    pub config_type: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: DateTime,
    pub updated_by: Option<i64>,
    pub updated_time: DateTime,
    pub remark: Option<String>,
    pub last_time: Option<DateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysContact {
    pub user_id: i64,
    pub contact_id: i64,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysDept {
    pub dept_id: i64,
    pub parent_id: Option<i64>,
    pub parent_name: Option<String>,
    pub ancestors: Option<String>,
    pub dept_name: Option<String>,
    pub order_num: Option<i32>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub account_prefix: Option<String>,
    pub account_seq: Option<i64>,
    pub status: Option<i16>,
    pub del_flag: Option<i16>,
    pub created_by: Option<i64>,
    pub created_time: DateTime,
    pub updated_by: Option<i64>,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysDictData {
    pub dict_code: i64,
    pub dict_sort: Option<i16>,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
    pub dict_type: String,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: Option<i16>,
    pub status: Option<i16>,
    pub created_by: Option<i64>,
    pub created_time: DateTime,
    pub updated_by: Option<i64>,
    pub updated_time: DateTime,
    pub remark: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysDictType {
    pub dict_id: i64,
    pub dict_name: Option<String>,
    pub dict_type: String,
    pub status: Option<i16>,
    pub created_by: Option<i64>,
    pub created_time: DateTime,
    pub updated_by: Option<i64>,
    pub updated_time: DateTime,
    pub remark: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysFollow {
    pub user_id: i64,
    pub follower_id: i64,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysFollowLog {
    pub id: i64,
    pub user_id: i64,
    pub follower_id: i64,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysGroup {
    pub id: i64,
    pub name: String,
    pub avatar: Option<String>,
    pub owner_id: i64,
    pub status: i16,
    pub remark: Option<String>,
    pub allow_speech: i16,
    pub allow_invition: i16,
    pub member_rule: i16,
    pub sensitive_words: Option<String>,
    pub tags: Option<String>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysGroupUser {
    pub group_id: i64,
    pub user_id: i64,
    pub is_admin: i16,
    pub forbidden_chat: i16,
    pub user_nick_name: Option<String>,
    pub user_remark: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysJob {
    pub job_id: i64,
    pub job_name: String,
    pub job_group: String,
    pub invoke_target: String,
    pub cron_expression: Option<String>,
    pub misfire_policy: Option<String>,
    pub concurrent: Option<String>,
    pub status: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: Option<DateTime>,
    pub updated_by: Option<i64>,
    pub updated_time: Option<DateTime>,
    pub remark: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysJobLog {
    pub job_log_id: i64,
    pub job_name: String,
    pub job_group: String,
    pub invoke_target: String,
    pub job_message: Option<String>,
    pub status: Option<String>,
    pub exception_info: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: Option<DateTime>,
    pub updated_by: Option<i64>,
    pub updated_time: Option<DateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysLevel {
    pub id: i64,
    pub level: Option<i16>,
    pub name: String,
    pub common: Option<String>,
    pub avatar: Option<String>,
    pub is_default: i16,
    pub status: i16,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysLogininfor {
    pub info_id: i64,
    pub user_id: Option<i64>,
    pub user_name: String,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub status: Option<i16>,
    pub msg: Option<String>,
    pub login_time: Option<DateTime>,
    pub user_agent_str: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysMenu {
    pub menu_id: i64,
    pub menu_name: String,
    pub parent_id: Option<i64>,
    pub order_num: Option<i16>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub component_name: Option<String>,
    pub query: Option<String>,
    pub is_frame: Option<i16>,
    pub is_cache: Option<i16>,
    pub menu_type: Option<String>,
    pub visible: Option<i16>,
    pub status: Option<i16>,
    pub perms: Option<String>,
    pub icon: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
    pub remark: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysNotice {
    pub notice_id: i64,
    pub notice_title: String,
    pub notice_type: i16,
    pub notice_content: Option<String>,
    pub status: Option<i16>,
    pub module: Option<i16>,
    pub module_id: Option<i64>,
    pub news_type: i16,
    pub from_user_id: Option<i64>,
    pub from_user_name: Option<String>,
    pub from_user_nick: Option<String>,
    pub to_user_id: Option<i64>,
    pub to_user_name: Option<String>,
    pub to_user_nick: Option<String>,
    pub remark: Option<String>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysOperLog {
    pub oper_id: i64,
    pub title: Option<String>,
    pub business_type: Option<i16>,
    pub method: Option<String>,
    pub request_method: Option<String>,
    pub operator_type: Option<i16>,
    pub oper_name: Option<String>,
    pub dept_name: Option<String>,
    pub oper_url: Option<String>,
    pub oper_ip: Option<String>,
    pub oper_location: Option<String>,
    pub oper_param: Option<String>,
    pub json_result: Option<String>,
    pub status: Option<i16>,
    pub error_msg: Option<String>,
    pub oper_time: Option<DateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysOssLog {
    pub id: i64,
    pub url: String,
    pub module: String,
    pub del_flag: Option<i16>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysPost {
    pub post_id: i64,
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    pub status: String,
    pub created_by: Option<String>,
    pub created_time: Option<DateTime>,
    pub updated_by: Option<String>,
    pub updated_time: Option<DateTime>,
    pub remark: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysReadLog {
    pub id: i64,
    pub user_id: i64,
    pub type_: Option<i16>,
    pub record_id: Option<i64>,
    pub last_date: DateTime,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysRobot {
    pub user_id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub avatar: Option<String>,
    pub level_id: Option<i64>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysRole {
    pub role_id: i64,
    pub role_code: Option<String>,
    pub role_name: String,
    pub role_key: String,
    pub role_sort: i16,
    pub data_scope: Option<i16>,
    pub menu_check_strictly: Option<i16>,
    pub dept_check_strictly: Option<i16>,
    pub type_: i16,
    pub status: i16,
    pub del_flag: Option<i16>,
    pub remark: Option<String>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysRoleDept {
    pub role_id: i64,
    pub dept_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysRoleMenu {
    pub role_id: i64,
    pub menu_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysSeq {
    pub id: i64,
    pub region: String,
    pub pre: String,
    pub code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysSignOn {
    pub id: i64,
    pub user_id: i64,
    pub module: i16,
    pub module_id: Option<i64>,
    pub module_name: Option<String>,
    pub channel_code: Option<String>,
    pub sign_on_time: DateTime,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub type_: Option<i16>,
    pub avatar: Option<String>,
    pub parent_id: Option<i64>,
    pub parent_user_name: Option<String>,
    pub level_id: Option<i64>,
    pub level_name: Option<String>,
    pub level_avatar: Option<String>,
    pub dept_id: Option<i64>,
    pub dept_name: Option<String>,
    pub dept_indx: Option<i16>,
    pub cur_to_top: Option<String>,
    pub top_to_cur: Option<String>,
    pub client_ip: Option<String>,
    pub client_address: Option<String>,
    pub client_agent: Option<String>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysUser {
    pub user_id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub user_type: Option<i16>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<i16>,
    pub avatar: Option<String>,
    pub cover: Option<String>,
    pub password: Option<String>,
    pub status: Option<i16>,
    pub del_flag: Option<i16>,
    pub remark: Option<String>,
    pub level_id: Option<i64>,
    pub dept_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub cur_to_top: Option<String>,
    pub top_to_cur: Option<String>,
    pub wechat: Option<String>,
    pub weichat_openid: Option<String>,
    pub qq: Option<String>,
    pub weibo: Option<String>,
    pub weibo_openid: Option<String>,
    pub decs: Option<String>,
    pub type_: Option<i16>,
    pub is_hot: Option<i16>,
    pub hot_count: Option<i32>,
    pub is_elite: Option<i16>,
    pub elite_count: Option<i32>,
    pub lecturer_name: Option<String>,
    pub lecturer_level: Option<i16>,
    pub lecturer_card_no: Option<String>,
    pub is_vote: Option<i16>,
    pub vote_count: Option<i32>,
    pub indx: Option<i16>,
    pub batch_no: Option<String>,
    pub follow_count: Option<i64>,
    pub real_follow_count: Option<i64>,
    pub tags: Option<String>,
    pub account_prefix: Option<String>,
    pub share_user_id: Option<i64>,
    pub created_by: Option<i64>,
    pub created_time: DateTime,
    pub updated_by: Option<i64>,
    pub updated_time: DateTime,
    pub login_ip: Option<String>,
    pub login_date: Option<DateTime>,
    pub virtual_role: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysUserAgent {
    pub user_id: i64,
    pub act_for_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysUserDept {
    pub user_id: i64,
    pub dept_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysUserPost {
    pub user_id: i64,
    pub post_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysUserRole {
    pub user_id: i64,
    pub role_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SysUserUser {
    pub user_id: i64,
    pub manage_user_id: i64,
    pub manage_subs: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysUserWsLog {
    pub id: i64,
    pub ws_session_type: Option<String>,
    pub room_id: Option<i64>,
    pub room_name: Option<String>,
    pub channel_code: Option<String>,
    pub user_id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub type_: i16,
    pub avatar: Option<String>,
    pub parent_id: Option<i64>,
    pub parent_user_name: Option<String>,
    pub level_id: Option<i64>,
    pub level_name: Option<String>,
    pub level_avatar: Option<String>,
    pub lecturer_name: Option<String>,
    pub lecturer_level: Option<i16>,
    pub lecturer_card_no: Option<String>,
    pub dept_id: Option<i64>,
    pub dept_name: Option<String>,
    pub dept_indx: Option<i16>,
    pub cur_to_top: Option<String>,
    pub top_to_cur: Option<String>,
    pub login_time: DateTime,
    pub logout_time: Option<DateTime>,
    pub heart_beat_time: Option<DateTime>,
    pub split_time: Option<DateTime>,
    pub status: i16,
    pub del_flag: i16,
    pub client_ip: Option<String>,
    pub client_address: Option<String>,
    pub client_agent: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysUserWsLogStatic {
    pub id: i64,
    pub ref_id: Option<i64>,
    pub minutes_level_date: Option<String>,
    pub is_online: i16,
    pub year: String,
    pub month: String,
    pub day: String,
    pub hour: String,
    pub minute: String,
    pub del_flag: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysUserWsLogStaticsJson {
    pub id: i64,
    pub ref_id: Option<i64>,
    pub room_id: Option<i64>,
    pub user_id: i64,
    pub year: String,
    pub month: String,
    pub day: String,
    pub hour: String,
    pub minute_json: serde_json::Value,
    pub split_time: Option<DateTime>,
    pub ref_ids: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SysUserWsLogStaticsMonth {
    pub id: i64,
    pub type_: i16,
    pub key: String,
    pub count: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XxlJobGroup {
    pub id: i32,
    pub app_name: String,
    pub title: String,
    pub address_type: i16,
    pub address_list: Option<String>,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XxlJobInfo {
    pub id: i32,
    pub job_group: i32,
    pub job_desc: String,
    pub add_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub author: Option<String>,
    pub alarm_email: Option<String>,
    pub schedule_type: String,
    pub schedule_conf: Option<String>,
    pub misfire_strategy: String,
    pub executor_route_strategy: Option<String>,
    pub executor_handler: Option<String>,
    pub executor_param: Option<String>,
    pub executor_block_strategy: Option<String>,
    pub executor_timeout: i32,
    pub executor_fail_retry_count: i32,
    pub glue_type: String,
    pub glue_source: Option<String>,
    pub glue_remark: Option<String>,
    pub glue_updatetime: Option<DateTime>,
    pub child_jobid: Option<String>,
    pub trigger_status: i16,
    pub trigger_last_time: i64,
    pub trigger_next_time: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XxlJobLock {
    pub lock_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XxlJobLog {
    pub id: i64,
    pub job_group: i32,
    pub job_id: i32,
    pub executor_address: Option<String>,
    pub executor_handler: Option<String>,
    pub executor_param: Option<String>,
    pub executor_sharding_param: Option<String>,
    pub executor_fail_retry_count: i32,
    pub trigger_time: Option<DateTime>,
    pub trigger_code: i32,
    pub trigger_msg: Option<String>,
    pub handle_time: Option<DateTime>,
    pub handle_code: i32,
    pub handle_msg: Option<String>,
    pub alarm_status: i16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XxlJobLogReport {
    pub id: i32,
    pub trigger_day: Option<DateTime>,
    pub running_count: i32,
    pub suc_count: i32,
    pub fail_count: i32,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XxlJobLogglue {
    pub id: i32,
    pub job_id: i32,
    pub glue_type: Option<String>,
    pub glue_source: Option<String>,
    pub glue_remark: String,
    pub add_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XxlJobRegistry {
    pub id: i32,
    pub registry_group: String,
    pub registry_key: String,
    pub registry_value: String,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XxlJobUser {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: i16,
    pub permission: Option<String>,
}
