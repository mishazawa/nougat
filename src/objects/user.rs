use crate::{
    database::DatabaseHandle,
    utils::{Error, config},
};
use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::actor::PersonType,
    protocol::{public_key::PublicKey, verification::verify_domains_match},
    traits::{Actor, Object},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use url::Url;

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub ap_id: ObjectId<User>,
    pub inbox: Url,
    pub outbox: Url,
    last_refreshed_at: DateTime<Utc>,
    pub public_key: String,
    pub private_key: Option<String>,
    pub followers: Vec<Url>,
    pub local: bool,
}

impl User {
    pub fn new(hostname: &str, name: &str) -> Result<User, Error> {
        let config = config(); // TODO: change
        let ap_id = Url::parse(&format!("https://{}/user/{}", hostname, &name))?.into();
        let inbox = Url::parse(&format!("https://{}/user/{}/inbox", hostname, &name))?;
        let outbox = Url::parse(&format!("https://{}/user/{}/outbox", hostname, &name))?;

        Ok(User {
            name: name.to_string(),
            ap_id,
            inbox,
            outbox,
            last_refreshed_at: Utc::now(),
            local: true,
            public_key: config.public_key.clone(),
            private_key: Some(config.private_key.clone()),
            followers: vec![],
        })
    }

    fn last_refreshed_at(&self) -> Option<DateTime<Utc>> {
        Some(self.last_refreshed_at)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "type")]
    kind: PersonType,
    preferred_username: String,
    id: ObjectId<User>,
    inbox: Url,
    name: String,
    outbox: Url,
    public_key: PublicKey,
}

#[async_trait::async_trait]
impl Object for User {
    type DataType = DatabaseHandle;
    type Kind = Person;
    type Error = Error;

    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let users = data.users.lock().unwrap();
        let res = users
            .clone()
            .into_iter()
            .find(|u| u.ap_id.inner() == &object_id);
        Ok(res)
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)?;
        Ok(())
    }

    async fn from_json(
        json: Self::Kind,
        _data: &Data<Self::DataType>,
    ) -> Result<Self, Self::Error> {
        Ok(User {
            name: json.preferred_username,
            ap_id: json.id,
            inbox: json.inbox,
            outbox: json.outbox,
            public_key: json.public_key.public_key_pem,
            private_key: None,
            last_refreshed_at: Utc::now(),
            followers: vec![],
            local: false,
        })
    }

    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Person {
            preferred_username: self.name.clone(),
            kind: Default::default(),
            id: self.ap_id.clone(),
            inbox: self.inbox.clone(),
            public_key: __self.public_key(),
            name: self.name.clone(),
            outbox: self.outbox.clone(),
        })
    }
}

impl Actor for User {
    fn id(&self) -> Url {
        self.ap_id.inner().clone()
    }

    fn public_key_pem(&self) -> &str {
        &self.public_key
    }

    fn private_key_pem(&self) -> Option<String> {
        self.private_key.clone()
    }

    fn inbox(&self) -> Url {
        self.inbox.clone()
    }
}
