pub mod application;
pub mod field;
pub mod http;
pub mod models;
pub mod plugin;
pub mod pub_sub;
pub mod session;
pub mod ui;

use std::net::SocketAddr;

use clap::{AppSettings, ArgEnum, Clap};
use log::info;

#[cfg(feature = "developer")]
use log::warn;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const BIRD: &'static str = "\n\x1b[48;5;15m                \x1b[38;5;138m▄\x1b[48;5;15m\x1b[38;5;52m▄\x1b[48;5;15m\x1b[38;5;235m▄\x1b[48;5;255m\x1b[38;5;234m▄\x1b[48;5;247m\x1b[38;5;234m▄\x1b[48;5;131m\x1b[38;5;233m▄\x1b[48;5;131m\x1b[38;5;233m▄\x1b[48;5;88m\x1b[38;5;233m▄\x1b[48;5;88m\x1b[38;5;233m▄\x1b[48;5;95m\x1b[38;5;234m▄\x1b[48;5;131m\x1b[38;5;234m▄\x1b[48;5;247m\x1b[38;5;235m▄\x1b[48;5;255m\x1b[38;5;88m▄\x1b[48;5;15m\x1b[38;5;88m▄\x1b[48;5;15m\x1b[38;5;88m▄\x1b[48;5;15m\x1b[38;5;138m▄\x1b[48;5;15m                \x1b[0m
\x1b[48;5;15m            \x1b[38;5;88m▄\x1b[48;5;15m\x1b[38;5;88m▄\x1b[48;5;88m \x1b[38;5;237m▄\x1b[48;5;95m\x1b[38;5;7m▄\x1b[48;5;181m\x1b[38;5;239m▄\x1b[48;5;181m           \x1b[48;5;52m\x1b[38;5;181m▄\x1b[48;5;88m\x1b[38;5;52m▄\x1b[48;5;88m   \x1b[48;5;15m\x1b[38;5;88m▄\x1b[48;5;15m\x1b[38;5;88m▄\x1b[48;5;15m            \x1b[0m
\x1b[48;5;15m         \x1b[38;5;1m▄\x1b[48;5;138m\x1b[38;5;88m▄\x1b[48;5;88m  \x1b[38;5;235m▄\x1b[48;5;233m\x1b[38;5;247m▄\x1b[48;5;181m \x1b[38;5;248m▄\x1b[48;5;181m\x1b[38;5;247m▄\x1b[48;5;181m\x1b[38;5;238m▄\x1b[48;5;181m\x1b[38;5;248m▄\x1b[48;5;181m     \x1b[38;5;7m▄\x1b[48;5;239m\x1b[38;5;181m▄\x1b[48;5;181m \x1b[38;5;236m▄\x1b[48;5;181m  \x1b[48;5;235m\x1b[38;5;138m▄\x1b[48;5;88m     \x1b[48;5;138m\x1b[38;5;88m▄\x1b[48;5;15m\x1b[38;5;88m▄\x1b[48;5;15m         \x1b[0m
\x1b[48;5;15m       \x1b[38;5;88m▄\x1b[48;5;88m \x1b[38;5;52m▄\x1b[48;5;88m\x1b[38;5;234m▄\x1b[48;5;234m \x1b[48;5;233m\x1b[38;5;234m▄\x1b[48;5;239m\x1b[38;5;247m▄\x1b[48;5;247m\x1b[38;5;248m▄\x1b[48;5;248m\x1b[38;5;247m▄\x1b[48;5;247m   \x1b[48;5;240m\x1b[38;5;247m▄\x1b[48;5;247m\x1b[38;5;102m▄\x1b[48;5;247m\x1b[38;5;245m▄\x1b[48;5;247m  \x1b[48;5;138m\x1b[38;5;246m▄\x1b[48;5;181m\x1b[38;5;235m▄\x1b[48;5;181m\x1b[38;5;233m▄\x1b[48;5;181m \x1b[48;5;138m\x1b[38;5;7m▄\x1b[48;5;240m\x1b[38;5;181m▄\x1b[48;5;181m\x1b[38;5;138m▄\x1b[48;5;181m\x1b[38;5;95m▄\x1b[48;5;233m\x1b[38;5;236m▄\x1b[48;5;88m       \x1b[48;5;15m\x1b[38;5;88m▄\x1b[48;5;15m       \x1b[0m
\x1b[48;5;15m     \x1b[38;5;1m▄\x1b[48;5;1m\x1b[38;5;88m▄\x1b[48;5;88m\x1b[38;5;234m▄\x1b[48;5;52m\x1b[38;5;234m▄\x1b[48;5;234m  \x1b[38;5;247m▄\x1b[48;5;247m\x1b[38;5;248m▄\x1b[48;5;145m\x1b[38;5;248m▄\x1b[48;5;248m  \x1b[38;5;247m▄\x1b[48;5;247m\x1b[38;5;234m▄\x1b[48;5;246m\x1b[38;5;235m▄\x1b[48;5;248m\x1b[38;5;234m▄\x1b[48;5;247m\x1b[38;5;235m▄\x1b[48;5;239m\x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m    \x1b[48;5;233m\x1b[38;5;234m▄\x1b[48;5;95m\x1b[38;5;234m▄\x1b[48;5;181m\x1b[38;5;240m▄\x1b[48;5;95m \x1b[48;5;181m \x1b[48;5;234m \x1b[48;5;88m         \x1b[48;5;15m\x1b[38;5;88m▄\x1b[48;5;15m     \x1b[0m
\x1b[48;5;15m    \x1b[48;5;138m\x1b[38;5;88m▄\x1b[48;5;88m\x1b[38;5;1m▄\x1b[48;5;52m\x1b[38;5;233m▄\x1b[48;5;234m   \x1b[48;5;241m\x1b[38;5;145m▄\x1b[48;5;248m\x1b[38;5;145m▄\x1b[48;5;248m \x1b[48;5;145m\x1b[38;5;248m▄\x1b[48;5;248m\x1b[38;5;237m▄\x1b[48;5;243m\x1b[38;5;235m▄\x1b[48;5;235m \x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m  \x1b[48;5;235m  \x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;235m \x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m   \x1b[48;5;233m\x1b[38;5;234m▄\x1b[48;5;238m\x1b[38;5;234m▄\x1b[48;5;236m\x1b[38;5;233m▄\x1b[48;5;88m\x1b[38;5;52m▄\x1b[48;5;88m          \x1b[48;5;138m\x1b[38;5;88m▄\x1b[48;5;15m    \x1b[0m
\x1b[48;5;15m   \x1b[48;5;88m \x1b[38;5;235m▄\x1b[48;5;233m\x1b[38;5;234m▄\x1b[48;5;234m  \x1b[38;5;242m▄\x1b[48;5;248m\x1b[38;5;145m▄\x1b[48;5;145m \x1b[38;5;248m▄\x1b[48;5;248m\x1b[38;5;235m▄\x1b[48;5;236m\x1b[38;5;235m▄\x1b[48;5;235m   \x1b[38;5;255m▄\x1b[48;5;254m\x1b[38;5;15m▄\x1b[48;5;15m \x1b[38;5;239m▄\x1b[48;5;15m\x1b[38;5;251m▄\x1b[48;5;238m\x1b[38;5;15m▄\x1b[48;5;234m \x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;234m        \x1b[48;5;233m\x1b[38;5;234m▄\x1b[48;5;233m\x1b[38;5;234m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;88m\x1b[38;5;233m▄\x1b[48;5;88m\x1b[38;5;234m▄\x1b[48;5;88m       \x1b[48;5;15m   \x1b[0m
\x1b[48;5;15m  \x1b[48;5;88m \x1b[38;5;233m▄\x1b[48;5;233m\x1b[38;5;234m▄\x1b[48;5;234m \x1b[38;5;235m▄\x1b[48;5;236m\x1b[38;5;248m▄\x1b[48;5;145m \x1b[38;5;248m▄\x1b[48;5;248m\x1b[38;5;234m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;235m     \x1b[48;5;15m  \x1b[48;5;233m \x1b[48;5;0m \x1b[38;5;232m▄\x1b[48;5;15m \x1b[48;5;145m\x1b[38;5;251m▄\x1b[48;5;234m          \x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;234m\x1b[38;5;236m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;88m\x1b[38;5;234m▄\x1b[48;5;88m\x1b[38;5;234m▄\x1b[48;5;88m\x1b[38;5;1m▄\x1b[48;5;88m   \x1b[48;5;15m  \x1b[0m
\x1b[48;5;15m \x1b[48;5;102m\x1b[38;5;234m▄\x1b[48;5;234m    \x1b[48;5;240m\x1b[38;5;248m▄\x1b[48;5;248m \x1b[48;5;145m\x1b[38;5;246m▄\x1b[48;5;235m \x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;235m     \x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;15m\x1b[38;5;239m▄\x1b[48;5;15m  \x1b[48;5;242m\x1b[38;5;15m▄\x1b[48;5;255m\x1b[38;5;15m▄\x1b[48;5;15m\x1b[38;5;255m▄\x1b[48;5;59m\x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m   \x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;234m  \x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;235m \x1b[38;5;236m▄\x1b[48;5;236m  \x1b[48;5;235m \x1b[48;5;236m \x1b[38;5;235m▄\x1b[48;5;236m\x1b[38;5;235m▄\x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;234m\x1b[38;5;236m▄\x1b[48;5;235m \x1b[48;5;88m\x1b[38;5;234m▄\x1b[48;5;88m \x1b[48;5;138m\x1b[38;5;88m▄\x1b[48;5;15m \x1b[0m
\x1b[48;5;15m\x1b[38;5;255m▄\x1b[48;5;234m    \x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;248m\x1b[38;5;145m▄\x1b[48;5;145m \x1b[48;5;239m\x1b[38;5;247m▄\x1b[48;5;235m  \x1b[48;5;234m \x1b[48;5;235m     \x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;235m \x1b[48;5;248m\x1b[38;5;235m▄\x1b[48;5;252m\x1b[38;5;235m▄\x1b[48;5;59m\x1b[38;5;235m▄\x1b[48;5;235m   \x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;236m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m  \x1b[48;5;235m\x1b[38;5;233m▄\x1b[48;5;236m\x1b[38;5;234m▄\x1b[48;5;236m\x1b[38;5;235m▄\x1b[48;5;236m\x1b[38;5;52m▄\x1b[48;5;236m\x1b[38;5;235m▄\x1b[48;5;235m\x1b[38;5;1m▄\x1b[48;5;236m\x1b[38;5;1m▄\x1b[48;5;236m\x1b[38;5;1m▄\x1b[48;5;236m\x1b[38;5;1m▄\x1b[48;5;236m\x1b[38;5;52m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;236m\x1b[38;5;234m▄\x1b[48;5;236m\x1b[38;5;234m▄\x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;235m\x1b[38;5;233m▄\x1b[48;5;88m \x1b[48;5;15m\x1b[38;5;255m▄\x1b[0m
\x1b[48;5;247m\x1b[38;5;241m▄\x1b[48;5;234m     \x1b[48;5;248m\x1b[38;5;234m▄\x1b[48;5;248m\x1b[38;5;243m▄\x1b[48;5;145m\x1b[38;5;235m▄\x1b[48;5;235m   \x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;235m              \x1b[48;5;234m  \x1b[38;5;1m▄\x1b[48;5;1m\x1b[38;5;88m▄\x1b[48;5;88m            \x1b[48;5;1m\x1b[38;5;88m▄\x1b[48;5;234m\x1b[38;5;88m▄\x1b[48;5;235m\x1b[38;5;52m▄\x1b[48;5;1m\x1b[38;5;88m▄\x1b[48;5;247m\x1b[38;5;131m▄\x1b[0m
\x1b[48;5;238m\x1b[38;5;235m▄\x1b[48;5;234m        \x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;234m  \x1b[48;5;235m             \x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;88m\x1b[38;5;233m▄\x1b[48;5;88m\x1b[38;5;1m▄\x1b[48;5;88m                \x1b[48;5;95m\x1b[38;5;88m▄\x1b[0m
\x1b[48;5;235m\x1b[38;5;238m▄\x1b[48;5;234m              \x1b[48;5;235m       \x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;235m    \x1b[38;5;236m▄\x1b[48;5;236m \x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;233m\x1b[38;5;236m▄\x1b[48;5;1m\x1b[38;5;234m▄\x1b[48;5;88m\x1b[38;5;52m▄\x1b[48;5;88m              \x1b[38;5;131m▄\x1b[0m
\x1b[48;5;241m\x1b[38;5;247m▄\x1b[48;5;234m                  \x1b[48;5;235m        \x1b[48;5;236m\x1b[38;5;235m▄\x1b[48;5;236m    \x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;233m\x1b[38;5;236m▄\x1b[48;5;1m\x1b[38;5;234m▄\x1b[48;5;88m\x1b[38;5;234m▄\x1b[48;5;88m           \x1b[48;5;131m\x1b[38;5;247m▄\x1b[0m
\x1b[48;5;255m\x1b[38;5;15m▄\x1b[48;5;234m                    \x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;234m  \x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;234m\x1b[38;5;233m▄\x1b[48;5;234m\x1b[38;5;88m▄\x1b[48;5;234m\x1b[38;5;88m▄\x1b[48;5;235m\x1b[38;5;88m▄\x1b[48;5;236m\x1b[38;5;52m▄\x1b[48;5;236m\x1b[38;5;234m▄\x1b[48;5;236m\x1b[38;5;235m▄\x1b[48;5;236m  \x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;234m\x1b[38;5;236m▄\x1b[48;5;88m\x1b[38;5;234m▄\x1b[48;5;88m\x1b[38;5;1m▄\x1b[48;5;88m        \x1b[48;5;255m\x1b[38;5;15m▄\x1b[0m
\x1b[48;5;15m \x1b[48;5;234m\x1b[38;5;8m▄\x1b[48;5;234m                     \x1b[38;5;236m▄\x1b[48;5;234m\x1b[38;5;237m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;88m\x1b[38;5;1m▄\x1b[48;5;88m      \x1b[48;5;1m\x1b[38;5;88m▄\x1b[48;5;234m\x1b[38;5;88m▄\x1b[48;5;234m\x1b[38;5;88m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;233m \x1b[48;5;1m\x1b[38;5;234m▄\x1b[48;5;88m      \x1b[38;5;138m▄\x1b[48;5;15m \x1b[0m
\x1b[48;5;15m  \x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m            \x1b[38;5;235m▄\x1b[48;5;234m   \x1b[38;5;254m▄\x1b[48;5;242m\x1b[38;5;15m▄\x1b[48;5;15m\x1b[38;5;255m▄\x1b[48;5;15m\x1b[38;5;234m▄\x1b[48;5;15m\x1b[38;5;234m▄\x1b[48;5;255m\x1b[38;5;234m▄\x1b[48;5;15m\x1b[38;5;234m▄\x1b[48;5;15m\x1b[38;5;239m▄\x1b[48;5;224m\x1b[38;5;15m▄\x1b[48;5;1m\x1b[38;5;15m▄\x1b[48;5;88m\x1b[38;5;95m▄\x1b[48;5;88m                \x1b[48;5;15m  \x1b[0m
\x1b[48;5;15m   \x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m  \x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m     \x1b[48;5;235m   \x1b[48;5;234m \x1b[48;5;102m\x1b[38;5;15m▄\x1b[48;5;15m \x1b[38;5;243m▄\x1b[48;5;234m     \x1b[48;5;233m \x1b[48;5;138m\x1b[38;5;88m▄\x1b[48;5;15m  \x1b[48;5;88m\x1b[38;5;138m▄\x1b[48;5;88m              \x1b[48;5;15m   \x1b[0m
\x1b[48;5;15m    \x1b[48;5;235m\x1b[38;5;246m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;235m     \x1b[48;5;234m      \x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;15m  \x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;235m \x1b[48;5;234m     \x1b[48;5;1m\x1b[38;5;234m▄\x1b[48;5;255m\x1b[38;5;254m▄\x1b[48;5;15m \x1b[48;5;224m\x1b[38;5;255m▄\x1b[48;5;88m            \x1b[38;5;138m▄\x1b[48;5;15m    \x1b[0m
\x1b[48;5;15m     \x1b[48;5;236m\x1b[38;5;15m▄\x1b[48;5;236m \x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;235m        \x1b[38;5;234m▄\x1b[48;5;234m \x1b[48;5;15m\x1b[38;5;255m▄\x1b[48;5;15m \x1b[48;5;235m\x1b[38;5;242m▄\x1b[48;5;235m \x1b[48;5;234m     \x1b[38;5;235m▄\x1b[48;5;15m  \x1b[48;5;181m\x1b[38;5;95m▄\x1b[48;5;88m           \x1b[38;5;15m▄\x1b[48;5;15m     \x1b[0m
\x1b[48;5;15m       \x1b[48;5;236m\x1b[38;5;15m▄\x1b[48;5;236m \x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;235m    \x1b[38;5;234m▄\x1b[48;5;234m  \x1b[38;5;236m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;15m\x1b[38;5;237m▄\x1b[48;5;15m \x1b[48;5;235m\x1b[38;5;15m▄\x1b[48;5;234m\x1b[38;5;237m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m  \x1b[38;5;145m▄\x1b[48;5;242m\x1b[38;5;15m▄\x1b[48;5;15m\x1b[38;5;252m▄\x1b[48;5;188m\x1b[38;5;234m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;235m\x1b[38;5;233m▄\x1b[48;5;88m        \x1b[38;5;15m▄\x1b[48;5;15m       \x1b[0m
\x1b[48;5;15m         \x1b[48;5;236m\x1b[38;5;15m▄\x1b[48;5;235m\x1b[38;5;246m▄\x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;235m  \x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;248m \x1b[48;5;242m\x1b[38;5;15m▄\x1b[48;5;235m\x1b[38;5;15m▄\x1b[48;5;236m\x1b[38;5;15m▄\x1b[48;5;242m\x1b[38;5;15m▄\x1b[48;5;255m\x1b[38;5;15m▄\x1b[48;5;235m\x1b[38;5;234m▄\x1b[48;5;235m \x1b[38;5;238m▄\x1b[48;5;15m \x1b[48;5;237m\x1b[38;5;15m▄\x1b[48;5;235m\x1b[38;5;15m▄\x1b[48;5;236m\x1b[38;5;15m▄\x1b[48;5;255m\x1b[38;5;15m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;233m▄\x1b[48;5;88m    \x1b[38;5;138m▄\x1b[48;5;88m\x1b[38;5;15m▄\x1b[48;5;15m         \x1b[0m
\x1b[48;5;15m            \x1b[48;5;235m\x1b[38;5;15m▄\x1b[48;5;235m\x1b[38;5;15m▄\x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;235m  \x1b[48;5;8m\x1b[38;5;235m▄\x1b[48;5;145m\x1b[38;5;235m▄\x1b[48;5;7m\x1b[38;5;234m▄\x1b[48;5;7m\x1b[38;5;235m▄\x1b[48;5;249m\x1b[38;5;235m▄\x1b[48;5;249m\x1b[38;5;235m▄\x1b[48;5;236m\x1b[38;5;235m▄\x1b[48;5;235m \x1b[48;5;239m\x1b[38;5;234m▄\x1b[48;5;248m\x1b[38;5;234m▄\x1b[48;5;249m\x1b[38;5;234m▄\x1b[48;5;250m\x1b[38;5;234m▄\x1b[48;5;249m\x1b[38;5;234m▄\x1b[48;5;248m\x1b[38;5;234m▄\x1b[48;5;88m  \x1b[48;5;1m\x1b[38;5;88m▄\x1b[48;5;88m\x1b[38;5;15m▄\x1b[48;5;88m\x1b[38;5;15m▄\x1b[48;5;15m            \x1b[0m
\x1b[48;5;15m                \x1b[48;5;102m\x1b[38;5;15m▄\x1b[48;5;235m\x1b[38;5;15m▄\x1b[48;5;235m\x1b[38;5;15m▄\x1b[48;5;235m\x1b[38;5;255m▄\x1b[48;5;235m\x1b[38;5;247m▄\x1b[48;5;235m\x1b[38;5;241m▄\x1b[48;5;235m\x1b[38;5;238m▄\x1b[48;5;235m\x1b[38;5;236m▄\x1b[48;5;234m\x1b[38;5;235m▄\x1b[48;5;234m\x1b[38;5;238m▄\x1b[48;5;234m\x1b[38;5;59m▄\x1b[48;5;234m\x1b[38;5;246m▄\x1b[48;5;234m\x1b[38;5;255m▄\x1b[48;5;234m\x1b[38;5;15m▄\x1b[48;5;234m\x1b[38;5;15m▄\x1b[48;5;138m\x1b[38;5;15m▄\x1b[48;5;15m                \x1b[0m
";
#[cfg(feature = "developer")]
const DEV_MESSAGE: &'static str = "Development Mode is enabled. Plugins can be modified remotely without authentication, DO NOT USE THIS IN PRODUCTION.";

#[derive(ArgEnum, PartialEq, Debug, Clone)]
pub enum UIWindow {
    Admin,
    Devtools,
    GraphqlPlayground,
    RefereePanel,
}

/// An alternative FIRST FMS designed around extensibility and compatibility.
#[derive(Clap)]
#[clap(version = VERSION, author = AUTHORS)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets the uri used to access the SQL Database using sqlx.
    #[clap(short, long, default_value = "main.db", env = "NEVERMORE_DB_URI")]
    db_uri: String,

    /// Sets the listening address of the http server.
    #[clap(
        short,
        long,
        default_value = "0.0.0.0:8000",
        env = "NEVERMORE_LISTEN_ADDR"
    )]
    listen_addr: String,

    // Defines whether a webview and tray should be created.
    #[clap(short, long)]
    system_tray: bool,

    // Opens only a specific window on startup, and stops once that window is closed.
    #[clap(arg_enum, short, long, env = "NEVERMORE_UI_WINDOW")]
    window: Option<UIWindow>,
}

fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_ok() {
        pretty_env_logger::try_init()?;
    } else {
        pretty_env_logger::formatted_timed_builder()
            .filter_level(log::LevelFilter::Warn)
            .filter_level(log::LevelFilter::Info)
            .try_init()?;
    }

    let opts = Opts::parse();

    info!("{}", BIRD);

    info!("Starting {} v{} by {}...", NAME, VERSION, AUTHORS);

    #[cfg(feature = "developer")]
    warn!("{}", DEV_MESSAGE);

    let rt = tokio::runtime::Runtime::new().unwrap();

    let http_addr: SocketAddr = opts.listen_addr.parse()?;


    let mut window = opts.window.clone();
    if let Some(window) = window.take() {
        rt.spawn(async_main(opts, http_addr.clone()));
    
        ui::create_window(window, http_addr)?;
    } else {
        if opts.system_tray {
            rt.spawn(async_main(opts, http_addr.clone()));
    
            ui::create_tray(http_addr)?;
        } else {
            rt.block_on(async_main(opts, http_addr));
        };
    }

    Ok(())
}

// Starts all Tokio based services.
async fn async_main(opts: Opts, http_addr: SocketAddr) {
    let app = application::Application::new(Some(opts.db_uri)).await;
    let app = if app.is_ok() {
        app.unwrap()
    } else {
        panic!(
            "Error while creating application, couldn't start Nevermore: {:?}",
            app.err()
        );
    };

    http::start(app, http_addr).await;
}
