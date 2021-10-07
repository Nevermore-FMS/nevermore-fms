use deno_core::{include_js_files, op_async, Extension, OpState, Resource, ResourceId};
use futures::AsyncWriteExt;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::vec;
use tokio::sync::Mutex;
use thrussh::client::{Config, Handle, Handler, Session, connect};
use thrussh_keys::key::PublicKey;

pub fn init() -> Extension {
    Extension::builder()
        .js(include_js_files!(
            prefix "deno:extensions/nevermore-ssh",
            "runtime/js/05-ssh.js",
        ))
        .ops(vec![
            ("op_create_ssh_client", op_async(op_create_ssh_client)),
            ("op_authenticate_client_with_password", op_async(op_authenticate_client_with_password)),
            ("op_exec_client", op_async(op_exec_client))
        ])
        .build()
}

struct Client {
}

impl Handler for Client {
   type Error = anyhow::Error;
   type FutureUnit = futures::future::Ready<Result<(Self, Session), anyhow::Error>>;
   type FutureBool = futures::future::Ready<Result<(Self, bool), anyhow::Error>>;

   fn finished_bool(self, b: bool) -> Self::FutureBool {
       futures::future::ready(Ok((self, b)))
   }
   fn finished(self, session: Session) -> Self::FutureUnit {
       futures::future::ready(Ok((self, session)))
   }
   fn check_server_key(self, _: &PublicKey) -> Self::FutureBool {
       self.finished_bool(true)
   }
}

// Events -->

struct SessionResource {
    session: Arc<Mutex<Handle<Client>>>,
}

impl Resource for SessionResource {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectArgs {
    addr: String
}

pub async fn op_create_ssh_client(
    state: Rc<RefCell<OpState>>,
    args: ConnectArgs,
    _: (),
) -> anyhow::Result<ResourceId> {
    let session = connect(Arc::new(Config::default()), args.addr, Client{}).await?;
    let mut borrowed_state = state.try_borrow_mut()?;
    let id = borrowed_state.resource_table.add(SessionResource{
        session: Arc::new(Mutex::new(session)),
    });

    Ok(id)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordAuthArgs {
    session: ResourceId, 
    username: String,
    password: String
}

pub async fn op_authenticate_client_with_password(
    state: Rc<RefCell<OpState>>,
    args: PasswordAuthArgs,
    _: (),
) -> anyhow::Result<bool> {
    let session_resource = state.try_borrow()?.resource_table.get::<SessionResource>(args.session)?;

    let is_ok = session_resource.session.clone().lock().await.authenticate_password(args.username, args.password).await?;

    Ok(is_ok)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecClientArgs {
    session: ResourceId, 
    wants_reply: bool,
    command: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecClientResult {
    data: Vec<u8>, 
    exit_status: u32
}

pub async fn op_exec_client(
    state: Rc<RefCell<OpState>>,
    args: ExecClientArgs,
    _: (),
) -> anyhow::Result<ExecClientResult> {
    let session_resource = state.try_borrow()?.resource_table.get::<SessionResource>(args.session)?;

    let mut channel = session_resource.session.clone().lock().await.channel_open_session().await?;

    channel.exec(args.wants_reply, args.command).await?;

    let mut output = Vec::new();

    let mut exit_status_returned = 0;
    
    while let Some(msg) = channel.wait().await {
        match msg {
            thrussh::ChannelMsg::Data { ref data } => {
                output.write_all(&data).await?;
            }
            thrussh::ChannelMsg::ExitStatus { exit_status } => {
                exit_status_returned = exit_status;
            }
            _ => {}
        }
    }

    Ok(ExecClientResult{
        data: output.clone(),
        exit_status: exit_status_returned
    })
}