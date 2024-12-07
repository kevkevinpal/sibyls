use super::{PriceFeed, PriceFeedError, Result};
use crate::AssetPair;
use async_trait::async_trait;
use log::{debug, info};
use reqwest::Client;
use time::OffsetDateTime;
use chrono::Local;

pub struct AlphaAvantage {}

type Response = Vec<Vec<f64>>;

#[async_trait]
impl PriceFeed for AlphaAvantage {
    fn id(&self) -> &'static str {
        "alpha_avantage"
    }

    fn translate_asset_pair(&self, asset_pair: AssetPair) -> Result<&'static str> {
        match asset_pair {
            AssetPair::BTCUSD => Err(PriceFeedError::InternalError(
                "alpha avantage does not support BTCUSD".to_string(),
            )),
            AssetPair::BTCUSDT => Err(PriceFeedError::InternalError(
                "alpha avantage does not support BTCUSDT".to_string(),
            )),
            AssetPair::MSTRUSD => Ok("MSTR"),
        }
    }

    async fn retrieve_price(&self, asset_pair: AssetPair, instant: OffsetDateTime) -> Result<f64> {
        let client = Client::new();
        let asset_pair_translation = self.translate_asset_pair(asset_pair).unwrap();
        let start_time: i64 = instant.unix_timestamp();

        info!("sending alpha avantage http request {asset_pair} {instant}");
        let res: Response = client
            .get(format!(
                "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey=3AGK3NIEIP5EK4K6",
                asset_pair_translation
            ))
            .query(&[("start", &start_time.to_string())])
            .send()
            .await?
            .json()
            .await?;

        if res.is_empty() {
            return Err(PriceFeedError::InternalError(
                "Invalid response from Alpha Avantage".to_string(),
            ));
        }

        debug!("received Alpha Avantage response: {:#?}", res);
        let price: f64 = 0.0;
        //let price = res
        //      .get(1)
        //      .ok_or_else(|| PriceFeedError::PriceNotAvailableError(asset_pair, instant))?
        //      .get(0)
        //      .ok_or_else(|| PriceFeedError::PriceNotAvailableError(asset_pair, instant))?
        //      .get(3)
        //      .ok_or_else(|| PriceFeedError::PriceNotAvailableError(asset_pair, instant))?
        //      .as_str()
        //      .parse()
        //      .unwrap();
        info!("MSTR price: {price}");
        Ok(price)
    }
}

#[cfg(test)]
mod tests {
    use crate::AssetPair::MSTRUSD;

    use super::*;

    #[tokio::test]
    async fn retrieve() {
        let feed = AlphaAvantage {};
        let price = feed.retrieve_price(MSTRUSD, OffsetDateTime::now_utc()).await;
        match price {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "{:#?}", &price),
        }
    }
}
