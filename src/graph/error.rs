use log::error;

#[allow(clippy::needless_pass_by_value)]
pub fn internalize_err(o: anyhow::Error) -> anyhow::Error {
    error!("Error when executing graphql request: {o}");
    anyhow::anyhow!("Internal server error")
}
