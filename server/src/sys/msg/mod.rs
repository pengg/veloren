pub mod character_screen;
pub mod general;
pub mod in_game;
pub mod ping;
pub mod register;

use crate::client::Client;
use serde::de::DeserializeOwned;

/// handles all send msg and calls a handle fn
/// Aborts when a error occurred returns cnt of successful msg otherwise
pub(in crate::sys::msg) fn try_recv_all<M, F>(
    client: &Client,
    stream_id: u8,
    mut f: F,
) -> Result<u64, crate::error::Error>
where
    M: DeserializeOwned,
    F: FnMut(&Client, M) -> Result<(), crate::error::Error>,
{
    let mut cnt = 0u64;
    loop {
        let msg = match client.recv(stream_id) {
            Ok(Some(msg)) => msg,
            Ok(None) => break Ok(cnt),
            Err(e) => break Err(e.into()),
        };
        if let Err(e) = f(client, msg) {
            break Err(e);
        }
        cnt += 1;
    }
}
