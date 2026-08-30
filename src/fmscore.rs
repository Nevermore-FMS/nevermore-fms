use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::{Arc, RwLock},
};

use directories::ProjectDirs;
use tokio_util::sync::CancellationToken;

use crate::{
    database::{init::open_main_db, main::interface::MainDbInterface}, field::Field, web::{self, openid::provider::OpenidProvider},
};

struct RawFMSCore {
    data_dir: PathBuf,
    field: Field,
    main_db: MainDbInterface,
    // event_db: EventDbInterface
    openid_provider: OpenidProvider,
}

#[derive(Clone)]
pub struct FMSCore {
    raw: Arc<RwLock<RawFMSCore>>,
}

impl FMSCore {
    // Public API -->

    pub fn field(&self) -> Field {
        let raw = self.raw.read().unwrap();
        raw.field.clone()
    }

    pub fn main_db(&self) -> MainDbInterface {
        let raw = self.raw.read().unwrap();
        raw.main_db.clone()
    }

    pub fn openid_provider(&self) -> OpenidProvider {
        let raw = self.raw.read().unwrap();
        raw.openid_provider.clone()
    }

    pub fn data_dir(&self) -> PathBuf {
        let raw = self.raw.read().unwrap();
        raw.data_dir.clone()
    }

    // Internal API -->

    pub(super) fn new(
        override_default_data_path: Option<PathBuf>,
        web_hostname: String,
        web_tls: bool,
    ) -> anyhow::Result<Self> {
        let data_dir_path = if let Some(custom_path) = override_default_data_path {
            custom_path
        } else if let Some(proj_dirs) = ProjectDirs::from("com", "edgarallanohms", "Nevermore-FMS")
        {
            proj_dirs.data_local_dir().to_path_buf()
        } else {
            anyhow::bail!("Unable to create data directory");
        };

        let main_db = open_main_db(&data_dir_path)?;

        let openid_provider = OpenidProvider::new(web_hostname, web_tls, &data_dir_path)?;

        let fms_core = RawFMSCore {
            data_dir: data_dir_path,
            field: Field::new(),
            main_db,
            openid_provider,
        };

        let fms_core = Self {
            raw: Arc::new(RwLock::new(fms_core)),
        };

        fms_core.field().set_fms_core(fms_core.clone()).unwrap();

        Ok(fms_core)
    }

    pub(super) async fn run(
        &self,
        ds_address: IpAddr,
        web_address: SocketAddr,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        let field = self.field();

        let res = tokio::try_join!(
            field.run(ds_address, cancellation_token.clone()),
            web::run(web_address, self.clone(), cancellation_token.clone())
        );

        if let Err(e) = res {
            return Err(e.context("FMSCore run terminated unexpectedly"));
        }

        Ok(())
    }
}
