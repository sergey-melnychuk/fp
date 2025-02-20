use std::sync::Arc;

use crate::common::{Ack, Address, PubKey, Receipt, State, Transfer};
use crate::error::{Error, Result};

#[derive(Clone)]
pub struct Client {
    url: String,
    http: Arc<reqwest::Client>,
}

impl Client {
    pub fn new(url: String, http: Arc<reqwest::Client>) -> Self {
        Self { url, http }
    }

    pub async fn id(&self) -> Result<PubKey> {
        let url = format!("{}/id", self.url);
        let json = self.http.get(url).send().await?.text().await?;
        let id = serde_json::from_str(&json)?;
        Ok(id)
    }

    pub async fn lookup(&self, src: &Address) -> Result<State> {
        let url = format!("{}/state/{}", self.url, hex::encode(src.0));
        let json = self.http.get(url).send().await?.text().await?;
        let state = serde_json::from_str(&json)?;
        Ok(state)
    }

    pub async fn accept(&self, tx: &Transfer) -> Result<Ack> {
        tx.check()?;
        let url = format!("{}/accept", self.url);
        let res = self.http.post(url).json(tx).send().await?;
        match res.error_for_status_ref() {
            Ok(_) => {
                let json = res.text().await?;
                let ret = serde_json::from_str(&json)?;
                Ok(ret)
            }
            Err(e) => {
                let msg = res.text().await?;
                let code = e.status().unwrap_or_default();
                Err(Error::Generic(format!("{code}: {msg}").into()))
            }
        }
    }

    pub async fn confirm(&self, receipt: &Receipt) -> Result<()> {
        let url = format!("{}/confirm", self.url);
        let res = self.http.post(url).json(receipt).send().await?;
        match res.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = res.text().await?;
                let code = e.status().unwrap_or_default();
                Err(Error::Generic(format!("{code}: {msg}").into()))
            }
        }
    }
}
