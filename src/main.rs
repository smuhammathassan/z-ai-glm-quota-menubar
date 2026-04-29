#![allow(deprecated, unexpected_cfgs, unsafe_op_in_unsafe_fn)]

use std::{path::PathBuf, ptr};

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivationPolicyAccessory, NSControl, NSMenu, NSMenuItem,
    NSStatusBar, NSStatusItem, NSTextField, NSVariableStatusItemLength, NSWindow,
};
use cocoa::base::{NO, YES, id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use z_ai_quota_menubar::client::fetch_quota;
use z_ai_quota_menubar::keychain::{read_api_key, write_api_key};
use z_ai_quota_menubar::quota::{QuotaSnapshot, menu_bar_text, quota_left_label};

struct AppState {
    status_item: id,
    menu: id,
    target: id,
    logo_image: id,
    last_snapshot: Option<QuotaSnapshot>,
    last_error: Option<String>,
}

static mut APP_STATE: *mut AppState = ptr::null_mut();

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyAccessory);

        let target = new_target();
        install_edit_menu(target);
        let status_item =
            NSStatusBar::systemStatusBar(nil).statusItemWithLength_(NSVariableStatusItemLength);
        let menu = NSMenu::new(nil).autorelease();
        let logo_image = load_logo_image();

        let state = Box::new(AppState {
            status_item,
            menu,
            target,
            logo_image,
            last_snapshot: None,
            last_error: None,
        });
        APP_STATE = Box::into_raw(state);

        rebuild_menu();
        refresh_quota();

        let _: id = msg_send![class!(NSTimer), scheduledTimerWithTimeInterval:60.0
            target:target
            selector:sel!(refreshNow:)
            userInfo:nil
            repeats:YES
        ];

        app.run();
    }
}

extern "C" fn refresh_now(_: &Object, _: Sel, _: id) {
    unsafe {
        refresh_quota();
    }
}

extern "C" fn set_api_key(_: &Object, _: Sel, _: id) {
    unsafe {
        if let Some(api_key) = prompt_api_key() {
            let state = &mut *APP_STATE;
            state.last_error = write_api_key(&api_key).err();
            refresh_quota();
        }
    }
}

extern "C" fn quit_app(_: &Object, _: Sel, _: id) {
    unsafe {
        let app = NSApp();
        let _: () = msg_send![app, terminate:nil];
    }
}

extern "C" fn noop(_: &Object, _: Sel, _: id) {}

unsafe fn new_target() -> id {
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("ZaiMenuTarget", superclass).expect("class registered once");
    decl.add_method(
        sel!(refreshNow:),
        refresh_now as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(
        sel!(setApiKey:),
        set_api_key as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(sel!(quitApp:), quit_app as extern "C" fn(&Object, Sel, id));
    decl.add_method(sel!(noop:), noop as extern "C" fn(&Object, Sel, id));
    let class = decl.register();
    msg_send![class, new]
}

unsafe fn install_edit_menu(target: id) {
    let main_menu = NSMenu::new(nil).autorelease();
    let app_menu_item = NSMenuItem::new(nil).autorelease();
    let edit_menu_item = NSMenuItem::new(nil).autorelease();
    main_menu.addItem_(app_menu_item);
    main_menu.addItem_(edit_menu_item);

    let app_menu = NSMenu::new(nil).autorelease();
    app_menu.addItem_(NSMenuItem::separatorItem(nil));
    add_action_item(app_menu, "Quit", sel!(quitApp:), target);
    app_menu_item.setSubmenu_(app_menu);

    let edit_menu = NSMenu::new(nil).autorelease();
    add_menu_command(edit_menu, "Cut", sel!(cut:), "x");
    add_menu_command(edit_menu, "Copy", sel!(copy:), "c");
    add_menu_command(edit_menu, "Paste", sel!(paste:), "v");
    add_menu_command(edit_menu, "Select All", sel!(selectAll:), "a");
    edit_menu_item.setSubmenu_(edit_menu);

    let app = NSApp();
    let _: () = msg_send![app, setMainMenu:main_menu];
}

unsafe fn refresh_quota() {
    let state = &mut *APP_STATE;
    match read_api_key() {
        Some(api_key) => match fetch_quota(&api_key) {
            Ok(snapshot) => {
                state.last_snapshot = Some(snapshot);
                state.last_error = None;
            }
            Err(error) => state.last_error = Some(error),
        },
        None => {
            state.last_snapshot = None;
            state.last_error = None;
        }
    }
    rebuild_menu();
}

unsafe fn rebuild_menu() {
    let state = &mut *APP_STATE;
    let _: () = msg_send![state.menu, removeAllItems];

    let title =
        NSString::alloc(nil).init_str(&format!(" {}", menu_bar_text(state.last_snapshot.as_ref())));
    let button = state.status_item.button();
    button.setTitle_(title);
    if state.logo_image != nil {
        let _: () = msg_send![button, setImage:state.logo_image];
        let _: () = msg_send![button, setImagePosition:2_u64];
    }
    state.status_item.setMenu_(state.menu);

    if read_api_key().is_some() {
        if let Some(snapshot) = &state.last_snapshot {
            add_disabled_item(
                state.menu,
                &format!(
                    "Zread quota: {}",
                    quota_left_label(snapshot.time_left_percent)
                ),
            );
            add_disabled_item(
                state.menu,
                &format!(
                    "Token quota: {}",
                    quota_left_label(snapshot.token_left_percent)
                ),
            );
            add_disabled_item(
                state.menu,
                &format!(
                    "Zread reset: {}",
                    snapshot.time_reset.as_deref().unwrap_or("--")
                ),
            );
            add_disabled_item(
                state.menu,
                &format!(
                    "Token reset: {}",
                    snapshot.token_reset.as_deref().unwrap_or("--")
                ),
            );
        } else {
            add_disabled_item(state.menu, "Zread quota: --% left");
            add_disabled_item(state.menu, "Token quota: --% left");
            add_disabled_item(state.menu, "Zread reset: --");
            add_disabled_item(state.menu, "Token reset: --");
        }

        if let Some(error) = &state.last_error {
            add_disabled_item(state.menu, &format!("Error: {}", concise_error(error)));
        }

        state.menu.addItem_(NSMenuItem::separatorItem(nil));
        add_action_item(state.menu, "Refresh now", sel!(refreshNow:), state.target);
    }

    add_action_item(state.menu, "Set API key", sel!(setApiKey:), state.target);
    add_action_item(state.menu, "Quit", sel!(quitApp:), state.target);
}

unsafe fn add_disabled_item(menu: id, title: &str) {
    let item = NSMenuItem::alloc(nil)
        .initWithTitle_action_keyEquivalent_(
            NSString::alloc(nil).init_str(title),
            sel!(noop:),
            NSString::alloc(nil).init_str(""),
        )
        .autorelease();
    item.setEnabled_(NO);
    menu.addItem_(item);
}

unsafe fn add_action_item(menu: id, title: &str, action: Sel, target: id) {
    let item = NSMenuItem::alloc(nil)
        .initWithTitle_action_keyEquivalent_(
            NSString::alloc(nil).init_str(title),
            action,
            NSString::alloc(nil).init_str(""),
        )
        .autorelease();
    item.setTarget_(target);
    menu.addItem_(item);
}

unsafe fn add_menu_command(menu: id, title: &str, action: Sel, key: &str) {
    let item = NSMenuItem::alloc(nil)
        .initWithTitle_action_keyEquivalent_(
            NSString::alloc(nil).init_str(title),
            action,
            NSString::alloc(nil).init_str(key),
        )
        .autorelease();
    menu.addItem_(item);
}

unsafe fn prompt_api_key() -> Option<String> {
    let alert: id = msg_send![class!(NSAlert), alloc];
    let alert: id = msg_send![alert, init];
    let _: () = msg_send![alert, setMessageText:NSString::alloc(nil).init_str("Set Z.ai API key")];
    let _: () = msg_send![alert, setInformativeText:NSString::alloc(nil).init_str("The key is stored in macOS Keychain.")];
    let _: id = msg_send![alert, addButtonWithTitle:NSString::alloc(nil).init_str("Save")];
    let _: id = msg_send![alert, addButtonWithTitle:NSString::alloc(nil).init_str("Cancel")];

    let input: id = msg_send![class!(NSTextField), alloc];
    let input: id = msg_send![input, initWithFrame:NSRect::new(
        NSPoint::new(0.0, 0.0),
        NSSize::new(560.0, 24.0),
    )];
    let initial_value = read_api_key().unwrap_or_default();
    input.setStringValue_(NSString::alloc(nil).init_str(&initial_value));
    let _: () = msg_send![
        input,
        setPlaceholderString:NSString::alloc(nil).init_str("Paste the full Z.ai API key")
    ];
    let _: () = msg_send![alert, setAccessoryView:input];
    let _: () = msg_send![alert, layout];
    let window: id = msg_send![alert, window];
    let _: () = msg_send![window, makeFirstResponder:input];

    let response: i64 = msg_send![alert, runModal];
    if response != 1000 {
        return None;
    }

    let value: id = msg_send![input, stringValue];
    let api_key = nsstring_to_string(value).trim().to_string();
    if api_key.is_empty() {
        return None;
    }
    if api_key.len() < 32 {
        show_message(
            "API key looks too short",
            "Paste the full Z.ai API key before saving.",
        );
        return None;
    }
    Some(api_key)
}

unsafe fn show_message(title: &str, message: &str) {
    let alert: id = msg_send![class!(NSAlert), alloc];
    let alert: id = msg_send![alert, init];
    let _: () = msg_send![alert, setMessageText:NSString::alloc(nil).init_str(title)];
    let _: () = msg_send![alert, setInformativeText:NSString::alloc(nil).init_str(message)];
    let _: id = msg_send![alert, addButtonWithTitle:NSString::alloc(nil).init_str("OK")];
    let _: i64 = msg_send![alert, runModal];
}

unsafe fn nsstring_to_string(value: id) -> String {
    let c_string: *const std::os::raw::c_char = msg_send![value, UTF8String];
    std::ffi::CStr::from_ptr(c_string)
        .to_string_lossy()
        .into_owned()
}

fn concise_error(error: &str) -> String {
    let first_line = error.lines().next().unwrap_or(error).trim();
    if first_line.len() > 48 {
        format!("{}...", &first_line[..48])
    } else {
        first_line.to_string()
    }
}

unsafe fn load_logo_image() -> id {
    let Some(logo_path) = bundled_logo_path() else {
        return nil;
    };
    let image: id = msg_send![class!(NSImage), alloc];
    let image: id = msg_send![
        image,
        initWithContentsOfFile:NSString::alloc(nil).init_str(&logo_path.to_string_lossy())
    ];
    if image != nil {
        let _: () = msg_send![image, setSize:NSSize::new(18.0, 18.0)];
    }
    image
}

fn bundled_logo_path() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let contents = exe.parent()?.parent()?;
    let bundled = contents.join("Resources").join("z-ai-logo.png");
    if bundled.exists() {
        return Some(bundled);
    }
    let local = std::env::current_dir()
        .ok()?
        .join("assets")
        .join("z-ai-logo.png");
    local.exists().then_some(local)
}
