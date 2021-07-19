#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
use std::net::SocketAddr;

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub fn create_tray(http_addr: SocketAddr) -> wry::Result<()> {
    use std::{collections::HashMap, net::{IpAddr, Ipv4Addr, Ipv6Addr}};
    #[cfg(target_os = "linux")]
    use std::path::Path;
    use tao::menu::{ContextMenu, MenuItemAttributes};
    #[cfg(target_os = "macos")]
    use wry::application::platform::macos::{ActivationPolicy, EventLoopExtMacOS};
    #[cfg(target_os = "linux")]
    use wry::application::platform::unix::WindowBuilderExtUnix;
    #[cfg(target_os = "windows")]
    use wry::application::platform::windows::WindowBuilderExtWindows;
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            menu::MenuId,
            system_tray::SystemTrayBuilder,
            window::{WindowBuilder, WindowId},
        },
        webview::{WebView, WebViewBuilder},
    };
    use log::info;

    let all_ip = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
    let all_ip_v6 = IpAddr::V6(Ipv6Addr::UNSPECIFIED);

    let address = if http_addr.ip() == all_ip || http_addr.ip() == all_ip_v6  {
        Ipv4Addr::UNSPECIFIED.to_string()
    } else {
        http_addr.ip().to_string()
    };

    let port = http_addr.port().to_string();

    // Build our event loop
    #[cfg(target_os = "macos")]
    let mut event_loop = EventLoop::new();

    #[cfg(not(target_os = "macos"))]
    let event_loop = EventLoop::new();

    // launch macos app without menu and without dock icon
    // should be set at launch
    #[cfg(target_os = "macos")]
    event_loop.set_activation_policy(ActivationPolicy::Accessory);

    let mut webviews: HashMap<WindowId, WebView> = HashMap::new();

    // Windows require Vec<u8> ICO file
    #[cfg(target_os = "windows")]
    let icon = include_bytes!("logos/eao_bird_circle.ico").to_vec();
    // macOS require Vec<u8> PNG file
    #[cfg(target_os = "macos")]
    let icon = include_bytes!("logos/eao_bird_circle.png").to_vec();
    // Linux require Pathbuf to PNG file
    #[cfg(target_os = "linux")]
    let icon = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/logos/eao_bird_circle.png");

    // linux require a menu so let's add only a open button
    let open_menu_id = MenuId::new("open_menu");
    let open_ref_menu_id = MenuId::new("open_ref_menu");
    let open_devtools_menu_id = MenuId::new("open_devtools_menu");
    let quit_menu_id = MenuId::new("quit_menu");

    let mut menu = ContextMenu::new();
    menu.add_item(MenuItemAttributes::new("Open").with_id(open_menu_id));
    menu.add_item(MenuItemAttributes::new("Open Referee Panel").with_id(open_ref_menu_id));
    #[cfg(target_os = "windows")]
    menu.add_item(MenuItemAttributes::new("Open Devtools").with_id(open_devtools_menu_id)); // Devtools only works in a Chromium webview. Might want to look into polyfilling this.
    menu.add_item(MenuItemAttributes::new("Quit").with_id(quit_menu_id));
    let _system_tray = SystemTrayBuilder::new(icon, Some(menu))
        .build(&event_loop)
        .unwrap();

    // launch WRY process
    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        let mut create_window_or_focus = |title: &str, url: &str| {
            // if we already have one webview, let's focus instead of opening
            if !webviews.is_empty() {
                for window in webviews.values() {
                    window.window().set_focus();
                }
                return;
            }

            // create our new window / webview instance
            let window_builder = WindowBuilder::new()
                .with_title(title);

            #[cfg(any(target_os = "windows", target_os = "linux"))]
            {
                window_builder = window_builder.with_skip_taskbar(true);
            }

            let window = window_builder.build(event_loop).unwrap();

            let id = window.id();

            let webview = WebViewBuilder::new(window)
                .unwrap()
                .with_url(url)
                .unwrap()
                .build()
                .unwrap();

            webviews.insert(id, webview);
        };

        match event {
            Event::NewEvents(StartCause::Init) => info!("System tray has been registered."),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                // Remove webview from our hashmap
                webviews.remove(&window_id);
            }
            // open a new admin window
            Event::MenuEvent { menu_id, .. } if menu_id == open_menu_id => {
                create_window_or_focus("Nevermore FMS", format!("http://{}:{}/", address, port).as_str());
            }
            // open a new referee window
            Event::MenuEvent { menu_id, .. } if menu_id == open_ref_menu_id => {
                create_window_or_focus("Nevermore FMS Referee Panel", format!("http://{}:{}/referee", address, port).as_str());
            }
            // open a new devtools window
            Event::MenuEvent { menu_id, .. } if menu_id == open_devtools_menu_id => {
                create_window_or_focus("Nevermore FMS Devtools", format!("http://{}:{}/devtools/?ws={}:{}/inspector", address, port, address, port).as_str());
            }
            // request to quit
            Event::MenuEvent { menu_id, .. } if menu_id == quit_menu_id => {
                *control_flow = ControlFlow::Exit
            }
            _ => (),
        }
    })
}
