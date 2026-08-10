use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::{Arc, RwLock},
};

use tokio_util::sync::CancellationToken;

use crate::{
    database::{init::open_main_db, main::interface::MainDbInterface},
    field::Field,
    web,
};

struct RawFMSCore {
    field: Field,
    main_db: MainDbInterface,
    // event_db: EventDbInterface
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

    // Internal API -->

    pub(super) fn new(override_default_data_path: Option<PathBuf>) -> anyhow::Result<Self> {
        let main_db = open_main_db(override_default_data_path)?;

        let fms_core = RawFMSCore {
            field: Field::new(),
            main_db,
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
