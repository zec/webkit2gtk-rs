// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

#[cfg(feature = "v2_2")]
use AuthenticationRequest;
use BackForwardList;
use BackForwardListItem;
#[cfg(feature = "v2_8")]
use ColorChooserRequest;
use Download;
#[cfg(feature = "v2_10")]
use EditorState;
use FindController;
use FormSubmissionRequest;
use HitTestResult;
use InsecureContentEvent;
use LoadEvent;
#[cfg(feature = "v2_6")]
use NavigationAction;
#[cfg(feature = "v2_8")]
use Notification;
use PermissionRequest;
use PolicyDecision;
use PolicyDecisionType;
use PrintOperation;
use Settings;
use URIRequest;
use WebContext;
use WebInspector;
use WebResource;
use WindowProperties;
use ffi;
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gtk;
use gtk_ffi;
use java_script_core;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct WebView(Object<ffi::WebKitWebView>): [
        gtk::Container => gtk_ffi::GtkContainer,
        gtk::Widget => gtk_ffi::GtkWidget,
    ];

    match fn {
        get_type => || ffi::webkit_web_view_get_type(),
    }
}

impl WebView {
    pub fn new() -> WebView {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::webkit_web_view_new()).downcast_unchecked()
        }
    }

    pub fn new_with_context(context: &WebContext) -> WebView {
        skip_assert_initialized!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::webkit_web_view_new_with_context(context.to_glib_none().0)).downcast_unchecked()
        }
    }

    #[cfg(feature = "v2_6")]
    pub fn new_with_settings(settings: &Settings) -> WebView {
        skip_assert_initialized!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::webkit_web_view_new_with_settings(settings.to_glib_none().0)).downcast_unchecked()
        }
    }

    //#[cfg(feature = "v2_6")]
    //pub fn new_with_user_content_manager(user_content_manager: /*Ignored*/&UserContentManager) -> WebView {
    //    unsafe { TODO: call ffi::webkit_web_view_new_with_user_content_manager() }
    //}

    //pub fn can_execute_editing_command(&self, command: &str, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: /*Unknown conversion*//*Unimplemented*/AsyncReadyCallback, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::webkit_web_view_can_execute_editing_command() }
    //}

    //pub fn can_execute_editing_command_finish<T: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &T, error: /*Ignored*/Option<Error>) -> bool {
    //    unsafe { TODO: call ffi::webkit_web_view_can_execute_editing_command_finish() }
    //}

    pub fn can_go_back(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_view_can_go_back(self.to_glib_none().0))
        }
    }

    pub fn can_go_forward(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_view_can_go_forward(self.to_glib_none().0))
        }
    }

    pub fn can_show_mime_type(&self, mime_type: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_view_can_show_mime_type(self.to_glib_none().0, mime_type.to_glib_none().0))
        }
    }

    pub fn download_uri(&self, uri: &str) -> Option<Download> {
        unsafe {
            from_glib_full(ffi::webkit_web_view_download_uri(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    pub fn execute_editing_command(&self, command: &str) {
        unsafe {
            ffi::webkit_web_view_execute_editing_command(self.to_glib_none().0, command.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_10")]
    pub fn execute_editing_command_with_argument(&self, command: &str, argument: &str) {
        unsafe {
            ffi::webkit_web_view_execute_editing_command_with_argument(self.to_glib_none().0, command.to_glib_none().0, argument.to_glib_none().0);
        }
    }

    pub fn get_back_forward_list(&self) -> Option<BackForwardList> {
        unsafe {
            from_glib_none(ffi::webkit_web_view_get_back_forward_list(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v2_8")]
    //pub fn get_background_color(&self, rgba: /*Ignored*/gdk::RGBA) {
    //    unsafe { TODO: call ffi::webkit_web_view_get_background_color() }
    //}

    pub fn get_context(&self) -> Option<WebContext> {
        unsafe {
            from_glib_none(ffi::webkit_web_view_get_context(self.to_glib_none().0))
        }
    }

    pub fn get_custom_charset(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_web_view_get_custom_charset(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_10")]
    pub fn get_editor_state(&self) -> Option<EditorState> {
        unsafe {
            from_glib_none(ffi::webkit_web_view_get_editor_state(self.to_glib_none().0))
        }
    }

    pub fn get_estimated_load_progress(&self) -> f64 {
        unsafe {
            ffi::webkit_web_view_get_estimated_load_progress(self.to_glib_none().0)
        }
    }

    //pub fn get_favicon(&self) -> /*Ignored*/Option<cairo::Surface> {
    //    unsafe { TODO: call ffi::webkit_web_view_get_favicon() }
    //}

    pub fn get_find_controller(&self) -> Option<FindController> {
        unsafe {
            from_glib_none(ffi::webkit_web_view_get_find_controller(self.to_glib_none().0))
        }
    }

    pub fn get_inspector(&self) -> Option<WebInspector> {
        unsafe {
            from_glib_none(ffi::webkit_web_view_get_inspector(self.to_glib_none().0))
        }
    }

    pub fn get_javascript_global_context(&self) -> Option<java_script_core::GlobalContext> {
        unsafe {
            from_glib_full(ffi::webkit_web_view_get_javascript_global_context(self.to_glib_none().0))
        }
    }

    pub fn get_main_resource(&self) -> Option<WebResource> {
        unsafe {
            from_glib_none(ffi::webkit_web_view_get_main_resource(self.to_glib_none().0))
        }
    }

    pub fn get_page_id(&self) -> u64 {
        unsafe {
            ffi::webkit_web_view_get_page_id(self.to_glib_none().0)
        }
    }

    //#[cfg(feature = "v2_12")]
    //pub fn get_session_state(&self) -> /*Ignored*/Option<WebViewSessionState> {
    //    unsafe { TODO: call ffi::webkit_web_view_get_session_state() }
    //}

    pub fn get_settings(&self) -> Option<Settings> {
        unsafe {
            from_glib_none(ffi::webkit_web_view_get_settings(self.to_glib_none().0))
        }
    }

    //pub fn get_snapshot(&self, region: SnapshotRegion, options: SnapshotOptions, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: /*Unknown conversion*//*Unimplemented*/AsyncReadyCallback, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::webkit_web_view_get_snapshot() }
    //}

    //pub fn get_snapshot_finish<T: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &T, error: /*Ignored*/Option<Error>) -> /*Ignored*/Option<cairo::Surface> {
    //    unsafe { TODO: call ffi::webkit_web_view_get_snapshot_finish() }
    //}

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_web_view_get_title(self.to_glib_none().0))
        }
    }

    //pub fn get_tls_info(&self, certificate: /*Ignored*/gio::TlsCertificate) -> Option</*Ignored*/gio::TlsCertificateFlags> {
    //    unsafe { TODO: call ffi::webkit_web_view_get_tls_info() }
    //}

    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_web_view_get_uri(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v2_6")]
    //pub fn get_user_content_manager(&self) -> /*Ignored*/Option<UserContentManager> {
    //    unsafe { TODO: call ffi::webkit_web_view_get_user_content_manager() }
    //}

    pub fn get_window_properties(&self) -> Option<WindowProperties> {
        unsafe {
            from_glib_none(ffi::webkit_web_view_get_window_properties(self.to_glib_none().0))
        }
    }

    pub fn get_zoom_level(&self) -> f64 {
        unsafe {
            ffi::webkit_web_view_get_zoom_level(self.to_glib_none().0)
        }
    }

    pub fn go_back(&self) {
        unsafe {
            ffi::webkit_web_view_go_back(self.to_glib_none().0);
        }
    }

    pub fn go_forward(&self) {
        unsafe {
            ffi::webkit_web_view_go_forward(self.to_glib_none().0);
        }
    }

    pub fn go_to_back_forward_list_item(&self, list_item: &BackForwardListItem) {
        unsafe {
            ffi::webkit_web_view_go_to_back_forward_list_item(self.to_glib_none().0, list_item.to_glib_none().0);
        }
    }

    pub fn is_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_view_is_editable(self.to_glib_none().0))
        }
    }

    pub fn is_loading(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_view_is_loading(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_8")]
    pub fn is_playing_audio(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_view_is_playing_audio(self.to_glib_none().0))
        }
    }

    pub fn load_alternate_html(&self, content: &str, content_uri: &str, base_uri: Option<&str>) {
        unsafe {
            ffi::webkit_web_view_load_alternate_html(self.to_glib_none().0, content.to_glib_none().0, content_uri.to_glib_none().0, base_uri.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v2_6")]
    //pub fn load_bytes(&self, bytes: /*Ignored*/&glib::Bytes, mime_type: Option<&str>, encoding: Option<&str>, base_uri: Option<&str>) {
    //    unsafe { TODO: call ffi::webkit_web_view_load_bytes() }
    //}

    pub fn load_html(&self, content: &str, base_uri: Option<&str>) {
        unsafe {
            ffi::webkit_web_view_load_html(self.to_glib_none().0, content.to_glib_none().0, base_uri.to_glib_none().0);
        }
    }

    pub fn load_plain_text(&self, plain_text: &str) {
        unsafe {
            ffi::webkit_web_view_load_plain_text(self.to_glib_none().0, plain_text.to_glib_none().0);
        }
    }

    pub fn load_request(&self, request: &URIRequest) {
        unsafe {
            ffi::webkit_web_view_load_request(self.to_glib_none().0, request.to_glib_none().0);
        }
    }

    pub fn load_uri(&self, uri: &str) {
        unsafe {
            ffi::webkit_web_view_load_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_4")]
    pub fn new_with_related_view(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_full(ffi::webkit_web_view_new_with_related_view(self.to_glib_none().0))
        }
    }

    pub fn reload(&self) {
        unsafe {
            ffi::webkit_web_view_reload(self.to_glib_none().0);
        }
    }

    pub fn reload_bypass_cache(&self) {
        unsafe {
            ffi::webkit_web_view_reload_bypass_cache(self.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v2_12")]
    //pub fn restore_session_state(&self, state: /*Ignored*/&WebViewSessionState) {
    //    unsafe { TODO: call ffi::webkit_web_view_restore_session_state() }
    //}

    //pub fn run_javascript(&self, script: &str, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: /*Unknown conversion*//*Unimplemented*/AsyncReadyCallback, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::webkit_web_view_run_javascript() }
    //}

    //pub fn run_javascript_finish<T: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &T, error: /*Ignored*/Option<Error>) -> Option<JavascriptResult> {
    //    unsafe { TODO: call ffi::webkit_web_view_run_javascript_finish() }
    //}

    //pub fn run_javascript_from_gresource(&self, resource: &str, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: /*Unknown conversion*//*Unimplemented*/AsyncReadyCallback, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::webkit_web_view_run_javascript_from_gresource() }
    //}

    //pub fn run_javascript_from_gresource_finish<T: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &T, error: /*Ignored*/Option<Error>) -> Option<JavascriptResult> {
    //    unsafe { TODO: call ffi::webkit_web_view_run_javascript_from_gresource_finish() }
    //}

    //pub fn save(&self, save_mode: SaveMode, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: /*Unknown conversion*//*Unimplemented*/AsyncReadyCallback, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::webkit_web_view_save() }
    //}

    //pub fn save_finish<T: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &T, error: /*Ignored*/Option<Error>) -> /*Ignored*/Option<gio::InputStream> {
    //    unsafe { TODO: call ffi::webkit_web_view_save_finish() }
    //}

    //pub fn save_to_file<T: IsA</*Ignored*/gio::File>>(&self, file: &T, save_mode: SaveMode, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: /*Unknown conversion*//*Unimplemented*/AsyncReadyCallback, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::webkit_web_view_save_to_file() }
    //}

    //pub fn save_to_file_finish<T: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &T, error: /*Ignored*/Option<Error>) -> bool {
    //    unsafe { TODO: call ffi::webkit_web_view_save_to_file_finish() }
    //}

    //#[cfg(feature = "v2_8")]
    //pub fn set_background_color(&self, rgba: /*Ignored*/&gdk::RGBA) {
    //    unsafe { TODO: call ffi::webkit_web_view_set_background_color() }
    //}

    pub fn set_custom_charset(&self, charset: Option<&str>) {
        unsafe {
            ffi::webkit_web_view_set_custom_charset(self.to_glib_none().0, charset.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_8")]
    pub fn set_editable(&self, editable: bool) {
        unsafe {
            ffi::webkit_web_view_set_editable(self.to_glib_none().0, editable.to_glib());
        }
    }

    pub fn set_settings(&self, settings: &Settings) {
        unsafe {
            ffi::webkit_web_view_set_settings(self.to_glib_none().0, settings.to_glib_none().0);
        }
    }

    pub fn set_zoom_level(&self, zoom_level: f64) {
        unsafe {
            ffi::webkit_web_view_set_zoom_level(self.to_glib_none().0, zoom_level);
        }
    }

    pub fn stop_loading(&self) {
        unsafe {
            ffi::webkit_web_view_stop_loading(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_12")]
    pub fn try_close(&self) {
        unsafe {
            ffi::webkit_web_view_try_close(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_2")]
    pub fn connect_authenticate<F: Fn(&WebView, &AuthenticationRequest) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView, &AuthenticationRequest) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "authenticate",
                transmute(authenticate_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_close<F: Fn(&WebView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "close",
                transmute(close_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //pub fn connect_context_menu<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored event: Gdk.Event
    //}

    pub fn connect_context_menu_dismissed<F: Fn(&WebView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "context-menu-dismissed",
                transmute(context_menu_dismissed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v2_6")]
    pub fn connect_create<F: Fn(&WebView, &NavigationAction) -> gtk::Widget + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView, &NavigationAction) -> gtk::Widget + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "create",
                transmute(create_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_decide_policy<F: Fn(&WebView, &PolicyDecision, PolicyDecisionType) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView, &PolicyDecision, PolicyDecisionType) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "decide-policy",
                transmute(decide_policy_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_enter_fullscreen<F: Fn(&WebView) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "enter-fullscreen",
                transmute(enter_fullscreen_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_insecure_content_detected<F: Fn(&WebView, InsecureContentEvent) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView, InsecureContentEvent) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "insecure-content-detected",
                transmute(insecure_content_detected_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_leave_fullscreen<F: Fn(&WebView) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "leave-fullscreen",
                transmute(leave_fullscreen_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_load_changed<F: Fn(&WebView, LoadEvent) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView, LoadEvent) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "load-changed",
                transmute(load_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //pub fn connect_load_failed<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored error: GLib.Error
    //}

    //#[cfg(feature = "v2_6")]
    //pub fn connect_load_failed_with_tls_errors<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored certificate: Gio.TlsCertificate
    //    Ignored errors: Gio.TlsCertificateFlags
    //}

    pub fn connect_mouse_target_changed<F: Fn(&WebView, &HitTestResult, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView, &HitTestResult, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "mouse-target-changed",
                transmute(mouse_target_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_permission_request<F: Fn(&WebView, &PermissionRequest) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView, &PermissionRequest) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "permission-request",
                transmute(permission_request_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_print<F: Fn(&WebView, &PrintOperation) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView, &PrintOperation) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "print",
                transmute(print_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_ready_to_show<F: Fn(&WebView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "ready-to-show",
                transmute(ready_to_show_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_resource_load_started<F: Fn(&WebView, &WebResource, &URIRequest) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView, &WebResource, &URIRequest) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "resource-load-started",
                transmute(resource_load_started_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_run_as_modal<F: Fn(&WebView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "run-as-modal",
                transmute(run_as_modal_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v2_8")]
    pub fn connect_run_color_chooser<F: Fn(&WebView, &ColorChooserRequest) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView, &ColorChooserRequest) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "run-color-chooser",
                transmute(run_color_chooser_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //pub fn connect_run_file_chooser<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored request: WebKit2.FileChooserRequest
    //}

    //pub fn connect_script_dialog<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored dialog: WebKit2.ScriptDialog
    //}

    #[cfg(feature = "v2_8")]
    pub fn connect_show_notification<F: Fn(&WebView, &Notification) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView, &Notification) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-notification",
                transmute(show_notification_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_submit_form<F: Fn(&WebView, &FormSubmissionRequest) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView, &FormSubmissionRequest) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "submit-form",
                transmute(submit_form_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_web_process_crashed<F: Fn(&WebView) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebView) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "web-process-crashed",
                transmute(web_process_crashed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_2")]
unsafe extern "C" fn authenticate_trampoline(this: *mut ffi::WebKitWebView, request: *mut ffi::WebKitAuthenticationRequest, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebView, &AuthenticationRequest) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(request)).to_glib()
}

unsafe extern "C" fn close_trampoline(this: *mut ffi::WebKitWebView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&WebView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn context_menu_dismissed_trampoline(this: *mut ffi::WebKitWebView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&WebView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v2_6")]
unsafe extern "C" fn create_trampoline(this: *mut ffi::WebKitWebView, navigation_action: *mut ffi::WebKitNavigationAction, f: glib_ffi::gpointer) -> *mut gtk_ffi::GtkWidget {
    callback_guard!();
    let f: &Box_<Fn(&WebView, &NavigationAction) -> gtk::Widget + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(navigation_action)).to_glib_full()
}

unsafe extern "C" fn decide_policy_trampoline(this: *mut ffi::WebKitWebView, decision: *mut ffi::WebKitPolicyDecision, decision_type: ffi::WebKitPolicyDecisionType, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebView, &PolicyDecision, PolicyDecisionType) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(decision), from_glib(decision_type)).to_glib()
}

unsafe extern "C" fn enter_fullscreen_trampoline(this: *mut ffi::WebKitWebView, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebView) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}

unsafe extern "C" fn insecure_content_detected_trampoline(this: *mut ffi::WebKitWebView, event: ffi::WebKitInsecureContentEvent, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&WebView, InsecureContentEvent) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(event))
}

unsafe extern "C" fn leave_fullscreen_trampoline(this: *mut ffi::WebKitWebView, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebView) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}

unsafe extern "C" fn load_changed_trampoline(this: *mut ffi::WebKitWebView, load_event: ffi::WebKitLoadEvent, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&WebView, LoadEvent) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(load_event))
}

unsafe extern "C" fn mouse_target_changed_trampoline(this: *mut ffi::WebKitWebView, hit_test_result: *mut ffi::WebKitHitTestResult, modifiers: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&WebView, &HitTestResult, u32) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(hit_test_result), modifiers)
}

unsafe extern "C" fn permission_request_trampoline(this: *mut ffi::WebKitWebView, request: *mut ffi::WebKitPermissionRequest, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebView, &PermissionRequest) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(request)).to_glib()
}

unsafe extern "C" fn print_trampoline(this: *mut ffi::WebKitWebView, print_operation: *mut ffi::WebKitPrintOperation, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebView, &PrintOperation) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(print_operation)).to_glib()
}

unsafe extern "C" fn ready_to_show_trampoline(this: *mut ffi::WebKitWebView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&WebView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn resource_load_started_trampoline(this: *mut ffi::WebKitWebView, resource: *mut ffi::WebKitWebResource, request: *mut ffi::WebKitURIRequest, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&WebView, &WebResource, &URIRequest) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(resource), &from_glib_none(request))
}

unsafe extern "C" fn run_as_modal_trampoline(this: *mut ffi::WebKitWebView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&WebView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v2_8")]
unsafe extern "C" fn run_color_chooser_trampoline(this: *mut ffi::WebKitWebView, request: *mut ffi::WebKitColorChooserRequest, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebView, &ColorChooserRequest) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(request)).to_glib()
}

#[cfg(feature = "v2_8")]
unsafe extern "C" fn show_notification_trampoline(this: *mut ffi::WebKitWebView, notification: *mut ffi::WebKitNotification, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebView, &Notification) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(notification)).to_glib()
}

unsafe extern "C" fn submit_form_trampoline(this: *mut ffi::WebKitWebView, request: *mut ffi::WebKitFormSubmissionRequest, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&WebView, &FormSubmissionRequest) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(request))
}

unsafe extern "C" fn web_process_crashed_trampoline(this: *mut ffi::WebKitWebView, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebView) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}
