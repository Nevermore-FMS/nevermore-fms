use image::GenericImageView;
use log::info;
use std::net::SocketAddr;
#[cfg(target_os = "linux")]
use std::path::Path;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};
use tao::{
    menu::{ContextMenu, MenuItemAttributes},
    window::{Fullscreen, Icon}, TrayId, dpi::LogicalSize,
};
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

use crate::UIWindow;

const PIXEL_SIZE: usize = std::mem::size_of::<u8>() * 4;

const BIRD_PNG: &'static [u8] = include_bytes!("logos/eao_bird_circle.png");

pub fn create_tray(http_addr: SocketAddr, fullscreen: bool) -> anyhow::Result<()> {
    let all_ip = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
    let all_ip_v6 = IpAddr::V6(Ipv6Addr::UNSPECIFIED);

    let address = if http_addr.ip() == all_ip || http_addr.ip() == all_ip_v6 {
        Ipv4Addr::LOCALHOST.to_string()
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

    let window_icon = get_icon()?;

    // linux require a menu so let's add only a open button
    let open_menu_id = MenuId::new("open_menu");
    let quit_menu_id = MenuId::new("quit_menu");

    let mut menu = ContextMenu::new();
    menu.add_item(MenuItemAttributes::new("Open UI").with_id(open_menu_id));
    menu.add_item(MenuItemAttributes::new("Quit").with_id(quit_menu_id));
    let _system_tray = SystemTrayBuilder::new(window_icon.clone(), Some(menu))
        .with_id(TrayId::new("nevermore-fms"))
        .with_tooltip("Nevermore FMS Background Agent")
        .build(&event_loop)
        .unwrap();

    let fullscreen = if fullscreen {
        Some(Fullscreen::Borderless(None))
    } else {
        None
    };

    // launch WRY process
    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        let mut create_window_or_focus = |title: &str, url: &str| -> anyhow::Result<()> {
            // if we already have one webview, let's focus instead of opening
            if !webviews.is_empty() {
                for window in webviews.values() {
                    window.window().set_focus();
                }
                return Ok(());
            }

            // create our new window / webview instance
            #[cfg(target_os = "windows")]
            let window_builder = WindowBuilder::new()
                .with_title(title)
                .with_window_icon(Some(window_icon.clone()))
                .with_taskbar_icon(Some(window_icon.clone()))
                .with_resizable(true)
                .with_maximized(fullscreen.is_none())
                .with_min_inner_size(LogicalSize::<f32>::from((400, 400)))
                .with_fullscreen(fullscreen.clone());

            #[cfg(not(target_os = "windows"))]
            let window_builder = WindowBuilder::new()
                .with_title(title)
                .with_window_icon(Some(window_icon.clone()))
                .with_resizable(true)
                .with_maximized(fullscreen.is_none())
                .with_min_inner_size(LogicalSize::<f32>::from((400, 400)))
                .with_fullscreen(fullscreen.clone());

            let window = window_builder.build(event_loop)?;

            let id = window.id();

            let webview = WebViewBuilder::new(window)?
                .with_url(url)?
                .build()?;

            webviews.insert(id, webview);
            Ok(())
        };

        match event {
            Event::NewEvents(StartCause::Init) => {
                info!("System tray has been registered.");
                create_window_or_focus(
                    "Nevermore FMS",
                    format!("http://{}:{}/", address, port).as_str(),
                ).expect("Couldn't start webview.");
            }
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
                create_window_or_focus(
                    "Nevermore FMS",
                    format!("http://{}:{}/", address, port).as_str(),
                ).expect("Couldn't start webview.");
            }
            // request to quit
            Event::MenuEvent { menu_id, .. } if menu_id == quit_menu_id => {
                *control_flow = ControlFlow::Exit
            }
            _ => (),
        }
    })
}

pub fn create_window(
    window_type: UIWindow,
    http_addr: SocketAddr,
    fullscreen: bool,
) -> anyhow::Result<()> {
    let all_ip = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
    let all_ip_v6 = IpAddr::V6(Ipv6Addr::UNSPECIFIED);

    let address = if http_addr.ip() == all_ip || http_addr.ip() == all_ip_v6 {
        Ipv4Addr::LOCALHOST.to_string()
    } else {
        http_addr.ip().to_string()
    };

    let port = http_addr.port().to_string();

    let (title, url) = match window_type {
        UIWindow::Admin => ("Nevermore FMS", format!("http://{}:{}/", address, port)),
    };
    let event_loop = EventLoop::new();

    let window_icon = get_icon()?;

    let fullscreen = if fullscreen {
        Some(Fullscreen::Borderless(None))
    } else {
        None
    };

    #[cfg(target_os = "windows")]
    let window_builder = WindowBuilder::new()
        .with_title(title)
        .with_window_icon(Some(window_icon.clone()))
        .with_taskbar_icon(Some(window_icon.clone()))
        .with_resizable(true)
        .with_fullscreen(fullscreen);

    #[cfg(not(target_os = "windows"))]
    let window_builder = WindowBuilder::new()
        .with_title(title)
        .with_window_icon(Some(window_icon.clone()))
        .with_resizable(true)
        .with_fullscreen(fullscreen);

    let window = window_builder.build(&event_loop).unwrap();

    let _ = WebViewBuilder::new(window)?
        .with_url(url.as_str())?
        .build()?;

    event_loop.run(move |event, _event_loop, control_flow| {
        match event {
            Event::NewEvents(StartCause::Init) => info!("Window has been created."),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn get_icon() -> anyhow::Result<Icon> {
    let window_image = image::load_from_memory(BIRD_PNG)?;

    let (width, height) = window_image.dimensions();

    let mut rgba = Vec::with_capacity((width * height) as usize * PIXEL_SIZE);

    for (_, _, pixel) in window_image.pixels() {
        rgba.extend_from_slice(&pixel.0);
    }

    let window_icon = Icon::from_rgba(rgba, width, height)?;

    Ok(window_icon)
}