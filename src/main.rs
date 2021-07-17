pub mod application;
pub mod field;
pub mod http;
pub mod models;
pub mod pub_sub;
pub mod plugin;


use log::info;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const BIRD: &'static str = "\x1b[48;5;15m                \x1b[38;5;138m▄\x1b[48;5;15m\x1b[38;5;52m▄\x1b[48;5;15m\x1b[38;5;235m▄\x1b[48;5;255m\x1b[38;5;234m▄\x1b[48;5;247m\x1b[38;5;234m▄\x1b[48;5;131m\x1b[38;5;233m▄\x1b[48;5;131m\x1b[38;5;233m▄\x1b[48;5;88m\x1b[38;5;233m▄\x1b[48;5;88m\x1b[38;5;233m▄\x1b[48;5;95m\x1b[38;5;234m▄\x1b[48;5;131m\x1b[38;5;234m▄\x1b[48;5;247m\x1b[38;5;235m▄\x1b[48;5;255m\x1b[38;5;88m▄\x1b[48;5;15m\x1b[38;5;88m▄\x1b[48;5;15m\x1b[38;5;88m▄\x1b[48;5;15m\x1b[38;5;138m▄\x1b[48;5;15m                \x1b[0m
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_ok() {
        pretty_env_logger::init();
    } else {
        pretty_env_logger::formatted_timed_builder().filter_level(log::LevelFilter::Info).try_init()?;
    }

    println!("{}", BIRD);

    info!("Starting {} v{} by {}...", NAME, VERSION, AUTHORS);

    let app = application::Application::new().await?;

    http::start(app).await;

    Ok(())
}
