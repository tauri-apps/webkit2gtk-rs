// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
mod authentication_request;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
pub use self::authentication_request::{AuthenticationRequest, AuthenticationRequestExt, NONE_AUTHENTICATION_REQUEST};

#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
mod automation_session;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
pub use self::automation_session::AutomationSessionBuilder;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
pub use self::automation_session::{AutomationSession, AutomationSessionExt, NONE_AUTOMATION_SESSION};

mod back_forward_list;
pub use self::back_forward_list::{BackForwardList, BackForwardListExt, NONE_BACK_FORWARD_LIST};

mod back_forward_list_item;
pub use self::back_forward_list_item::{BackForwardListItem, BackForwardListItemExt, NONE_BACK_FORWARD_LIST_ITEM};

#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
mod color_chooser_request;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
pub use self::color_chooser_request::ColorChooserRequestBuilder;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
pub use self::color_chooser_request::{ColorChooserRequest, ColorChooserRequestExt, NONE_COLOR_CHOOSER_REQUEST};

mod context_menu;
pub use self::context_menu::{ContextMenu, ContextMenuExt, NONE_CONTEXT_MENU};

mod context_menu_item;
pub use self::context_menu_item::{ContextMenuItem, ContextMenuItemExt, NONE_CONTEXT_MENU_ITEM};

mod cookie_manager;
pub use self::cookie_manager::{CookieManager, CookieManagerExt, NONE_COOKIE_MANAGER};

mod device_info_permission_request;
pub use self::device_info_permission_request::{
    DeviceInfoPermissionRequest, NONE_DEVICE_INFO_PERMISSION_REQUEST,
};

mod download;
pub use self::download::DownloadBuilder;
pub use self::download::{Download, DownloadExt, NONE_DOWNLOAD};

#[cfg(any(feature = "v2_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
mod editor_state;
#[cfg(any(feature = "v2_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
pub use self::editor_state::{EditorState, EditorStateExt, NONE_EDITOR_STATE};

mod favicon_database;
pub use self::favicon_database::{FaviconDatabase, FaviconDatabaseExt, NONE_FAVICON_DATABASE};

mod file_chooser_request;
pub use self::file_chooser_request::{FileChooserRequest, FileChooserRequestExt, NONE_FILE_CHOOSER_REQUEST};

mod find_controller;
pub use self::find_controller::FindControllerBuilder;
pub use self::find_controller::{FindController, FindControllerExt, NONE_FIND_CONTROLLER};

mod form_submission_request;
pub use self::form_submission_request::{FormSubmissionRequest, FormSubmissionRequestExt, NONE_FORM_SUBMISSION_REQUEST};

#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
mod geolocation_manager;
#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
pub use self::geolocation_manager::{GeolocationManager, GeolocationManagerExt, NONE_GEOLOCATION_MANAGER};

mod geolocation_permission_request;
pub use self::geolocation_permission_request::{
    GeolocationPermissionRequest, NONE_GEOLOCATION_PERMISSION_REQUEST,
};

mod hit_test_result;
pub use self::hit_test_result::HitTestResultBuilder;
pub use self::hit_test_result::{HitTestResult, HitTestResultExt, NONE_HIT_TEST_RESULT};

#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
mod input_method_context;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::input_method_context::{InputMethodContext, InputMethodContextExt, NONE_INPUT_METHOD_CONTEXT};

#[cfg(any(feature = "v2_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
mod install_missing_media_plugins_permission_request;
#[cfg(any(feature = "v2_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
pub use self::install_missing_media_plugins_permission_request::{
    InstallMissingMediaPluginsPermissionRequest, InstallMissingMediaPluginsPermissionRequestExt,
    NONE_INSTALL_MISSING_MEDIA_PLUGINS_PERMISSION_REQUEST,
};

mod navigation_policy_decision;
pub use self::navigation_policy_decision::{
    NavigationPolicyDecision, NavigationPolicyDecisionExt, NONE_NAVIGATION_POLICY_DECISION,
};

#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
mod notification;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
pub use self::notification::{Notification, NotificationExt, NONE_NOTIFICATION};

mod notification_permission_request;
pub use self::notification_permission_request::{
    NotificationPermissionRequest, NONE_NOTIFICATION_PERMISSION_REQUEST,
};

#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
mod option_menu;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
pub use self::option_menu::{OptionMenu, OptionMenuExt, NONE_OPTION_MENU};

mod permission_request;
pub use self::permission_request::{PermissionRequest, PermissionRequestExt, NONE_PERMISSION_REQUEST};

mod plugin;
pub use self::plugin::{Plugin, PluginExt, NONE_PLUGIN};

mod pointer_lock_permission_request;
pub use self::pointer_lock_permission_request::{
    PointerLockPermissionRequest, NONE_POINTER_LOCK_PERMISSION_REQUEST,
};

mod policy_decision;
pub use self::policy_decision::{PolicyDecision, PolicyDecisionExt, NONE_POLICY_DECISION};

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
mod print_custom_widget;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::print_custom_widget::PrintCustomWidgetBuilder;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::print_custom_widget::{PrintCustomWidget, PrintCustomWidgetExt, NONE_PRINT_CUSTOM_WIDGET};

mod print_operation;
pub use self::print_operation::PrintOperationBuilder;
pub use self::print_operation::{PrintOperation, PrintOperationExt, NONE_PRINT_OPERATION};

mod response_policy_decision;
pub use self::response_policy_decision::{ResponsePolicyDecision, ResponsePolicyDecisionExt, NONE_RESPONSE_POLICY_DECISION};

mod security_manager;
pub use self::security_manager::{SecurityManager, SecurityManagerExt, NONE_SECURITY_MANAGER};

mod settings;
pub use self::settings::SettingsBuilder;
pub use self::settings::{Settings, SettingsExt, NONE_SETTINGS};

mod uri_request;
pub use self::uri_request::URIRequestBuilder;
pub use self::uri_request::{URIRequest, URIRequestExt, NONE_URI_REQUEST};

mod uri_response;
pub use self::uri_response::{URIResponse, URIResponseExt, NONE_URI_RESPONSE};

mod uri_scheme_request;
pub use self::uri_scheme_request::{URISchemeRequest, URISchemeRequestExt, NONE_URI_SCHEME_REQUEST};

#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
mod user_content_manager;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::user_content_manager::{UserContentManager, UserContentManagerExt, NONE_USER_CONTENT_MANAGER};

mod user_media_permission_request;
pub use self::user_media_permission_request::{
    UserMediaPermissionRequest, UserMediaPermissionRequestExt, NONE_USER_MEDIA_PERMISSION_REQUEST,
};

#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
mod user_message;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::user_message::UserMessageBuilder;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::user_message::{UserMessage, UserMessageExt, NONE_USER_MESSAGE};

mod web_context;
pub use self::web_context::WebContextBuilder;
pub use self::web_context::{WebContext, WebContextExt, NONE_WEB_CONTEXT};

mod web_inspector;
pub use self::web_inspector::{WebInspector, WebInspectorExt, NONE_WEB_INSPECTOR};

mod web_resource;
pub use self::web_resource::{WebResource, WebResourceExt, NONE_WEB_RESOURCE};

mod web_view;
pub use self::web_view::WebViewBuilder;
pub use self::web_view::{WebView, WebViewExt, NONE_WEB_VIEW};

mod web_view_base;
pub use self::web_view_base::{WebViewBase, NONE_WEB_VIEW_BASE};

#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
mod website_data_access_permission_request;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
pub use self::website_data_access_permission_request::{
    WebsiteDataAccessPermissionRequest, WebsiteDataAccessPermissionRequestExt, NONE_WEBSITE_DATA_ACCESS_PERMISSION_REQUEST,
};

#[cfg(any(feature = "v2_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
mod website_data_manager;
#[cfg(any(feature = "v2_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
pub use self::website_data_manager::WebsiteDataManagerBuilder;
#[cfg(any(feature = "v2_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
pub use self::website_data_manager::{WebsiteDataManager, WebsiteDataManagerExt, NONE_WEBSITE_DATA_MANAGER};

#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
mod website_policies;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
pub use self::website_policies::WebsitePoliciesBuilder;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
pub use self::website_policies::{WebsitePolicies, WebsitePoliciesExt, NONE_WEBSITE_POLICIES};

mod window_properties;
pub use self::window_properties::WindowPropertiesBuilder;
pub use self::window_properties::{WindowProperties, WindowPropertiesExt, NONE_WINDOW_PROPERTIES};

#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
mod application_info;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
pub use self::application_info::ApplicationInfo;

#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
mod credential;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
pub use self::credential::Credential;

#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
mod geolocation_position;
#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
pub use self::geolocation_position::GeolocationPosition;

#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
mod itp_first_party;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
pub use self::itp_first_party::ITPFirstParty;

#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
mod itp_third_party;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
pub use self::itp_third_party::ITPThirdParty;

#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
mod input_method_underline;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::input_method_underline::InputMethodUnderline;

mod javascript_result;
pub use self::javascript_result::JavascriptResult;

mod mime_info;
pub use self::mime_info::MimeInfo;

#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
mod navigation_action;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::navigation_action::NavigationAction;

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
mod network_proxy_settings;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::network_proxy_settings::NetworkProxySettings;

#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
mod option_menu_item;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
pub use self::option_menu_item::OptionMenuItem;

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
mod security_origin;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::security_origin::SecurityOrigin;

#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
mod user_script;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::user_script::UserScript;

#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
mod user_style_sheet;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::user_style_sheet::UserStyleSheet;

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
mod web_view_session_state;
#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
pub use self::web_view_session_state::WebViewSessionState;

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
mod website_data;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::website_data::WebsiteData;

mod enums;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
pub use self::enums::AuthenticationScheme;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::enums::AutomationBrowsingContextPresentation;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
pub use self::enums::AutoplayPolicy;
pub use self::enums::CacheModel;
pub use self::enums::ContextMenuAction;
pub use self::enums::CookieAcceptPolicy;
pub use self::enums::CookiePersistentStorage;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
pub use self::enums::CredentialPersistence;
pub use self::enums::DownloadError;
pub use self::enums::FaviconDatabaseError;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::enums::HardwareAccelerationPolicy;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::enums::InputPurpose;
pub use self::enums::InsecureContentEvent;
pub use self::enums::JavascriptError;
pub use self::enums::LoadEvent;
pub use self::enums::NavigationType;
pub use self::enums::NetworkError;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::enums::NetworkProxyMode;
pub use self::enums::PluginError;
pub use self::enums::PolicyDecisionType;
pub use self::enums::PolicyError;
pub use self::enums::PrintError;
pub use self::enums::PrintOperationResponse;
#[cfg(any(feature = "v2_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
pub use self::enums::ProcessModel;
pub use self::enums::SaveMode;
pub use self::enums::ScriptDialogType;
pub use self::enums::SnapshotError;
pub use self::enums::SnapshotRegion;
pub use self::enums::TLSErrorsPolicy;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
pub use self::enums::UserContentFilterError;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::enums::UserContentInjectedFrames;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::enums::UserMessageError;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::enums::UserScriptInjectionTime;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::enums::UserStyleLevel;
#[cfg(any(feature = "v2_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
pub use self::enums::WebProcessTerminationReason;

mod flags;
#[cfg(any(feature = "v2_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
pub use self::flags::EditorTypingAttributes;
pub use self::flags::FindOptions;
pub use self::flags::HitTestResultContext;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::flags::InputHints;
pub use self::flags::SnapshotOptions;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::flags::WebsiteDataTypes;

#[doc(hidden)]
pub mod traits {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    pub use super::authentication_request::AuthenticationRequestExt;
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    pub use super::automation_session::AutomationSessionExt;
    pub use super::back_forward_list::BackForwardListExt;
    pub use super::back_forward_list_item::BackForwardListItemExt;
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    pub use super::color_chooser_request::ColorChooserRequestExt;
    pub use super::context_menu::ContextMenuExt;
    pub use super::context_menu_item::ContextMenuItemExt;
    pub use super::cookie_manager::CookieManagerExt;
    pub use super::download::DownloadExt;
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    pub use super::editor_state::EditorStateExt;
    pub use super::favicon_database::FaviconDatabaseExt;
    pub use super::file_chooser_request::FileChooserRequestExt;
    pub use super::find_controller::FindControllerExt;
    pub use super::form_submission_request::FormSubmissionRequestExt;
    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    pub use super::geolocation_manager::GeolocationManagerExt;
    pub use super::hit_test_result::HitTestResultExt;
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    pub use super::input_method_context::InputMethodContextExt;
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    pub use super::install_missing_media_plugins_permission_request::InstallMissingMediaPluginsPermissionRequestExt;
    pub use super::navigation_policy_decision::NavigationPolicyDecisionExt;
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    pub use super::notification::NotificationExt;
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    pub use super::option_menu::OptionMenuExt;
    pub use super::permission_request::PermissionRequestExt;
    pub use super::plugin::PluginExt;
    pub use super::policy_decision::PolicyDecisionExt;
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    pub use super::print_custom_widget::PrintCustomWidgetExt;
    pub use super::print_operation::PrintOperationExt;
    pub use super::response_policy_decision::ResponsePolicyDecisionExt;
    pub use super::security_manager::SecurityManagerExt;
    pub use super::settings::SettingsExt;
    pub use super::uri_request::URIRequestExt;
    pub use super::uri_response::URIResponseExt;
    pub use super::uri_scheme_request::URISchemeRequestExt;
    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    pub use super::user_content_manager::UserContentManagerExt;
    pub use super::user_media_permission_request::UserMediaPermissionRequestExt;
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    pub use super::user_message::UserMessageExt;
    pub use super::web_context::WebContextExt;
    pub use super::web_inspector::WebInspectorExt;
    pub use super::web_resource::WebResourceExt;
    pub use super::web_view::WebViewExt;
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    pub use super::website_data_access_permission_request::WebsiteDataAccessPermissionRequestExt;
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    pub use super::website_data_manager::WebsiteDataManagerExt;
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    pub use super::website_policies::WebsitePoliciesExt;
    pub use super::window_properties::WindowPropertiesExt;
}
