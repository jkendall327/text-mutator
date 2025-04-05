use std::borrow::Cow;

use anyhow::bail;

#[derive(Clone, Debug)]
pub struct EnvironmentVariables {
    pub frontend_url: Cow<'static, str>,
    pub backend_url: Cow<'static, str>,
}

impl EnvironmentVariables {
    /// # Errors
    /// Errors if one of the environment variables is not found.
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            frontend_url: match dotenvy::var("MUTATOR_FRONTEND_URL") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing frontend URL: {err}"),
            },
            backend_url: match dotenvy::var("MUTATOR_BACKEND_URL") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing backend URL: {err}"),
            },
        })
    }
}
