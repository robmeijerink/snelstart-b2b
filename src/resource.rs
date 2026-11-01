use std::marker::PhantomData;

use futures_util::stream::{self, Stream, TryStreamExt};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::client::Client;
use crate::error::{Error, Result};
use crate::odata::{MAX_PAGE, Query, Top};
use crate::types::Id;

pub trait Endpoint {
    type Model: DeserializeOwned + Send + 'static;
    type Entity;
    const PATH: &'static str;
}

pub trait Listable: Endpoint {}

pub trait Paged: Listable {}

pub trait Findable: Endpoint {}

pub trait Creatable: Endpoint {
    type Input: Serialize + Sync;
    type Output: DeserializeOwned;
}

pub trait Updatable: Endpoint {
    type Input: Serialize + Sync;
    type Output: DeserializeOwned;
}

pub trait Deletable: Endpoint {}

pub struct Resource<'c, E> {
    client: &'c Client,
    endpoint: PhantomData<fn() -> E>,
}

impl<'c, E: Endpoint> Resource<'c, E> {
    pub(crate) fn new(client: &'c Client) -> Self {
        Self {
            client,
            endpoint: PhantomData,
        }
    }

    fn item(id: Id<E::Entity>) -> String {
        format!("{}/{}", E::PATH, id.uuid())
    }
}

impl<E: Listable> Resource<'_, E> {
    pub async fn list(&self, query: &Query) -> Result<Vec<E::Model>> {
        self.client.get(E::PATH, query).await
    }
}

impl<'c, E: Paged + 'c> Resource<'c, E> {
    pub fn all(&self, query: Query) -> impl Stream<Item = Result<E::Model>> + 'c {
        struct Cursor {
            skip: u32,
            previous: Option<Vec<Value>>,
            done: bool,
        }

        let client = self.client;
        let start = Cursor {
            skip: query.start(),
            previous: None,
            done: false,
        };

        stream::try_unfold(start, move |mut cursor| {
            let query = query.clone();
            async move {
                if cursor.done {
                    return Ok(None);
                }
                let page: Vec<Value> = client
                    .get(E::PATH, &query.page(cursor.skip, Top::MAX))
                    .await?;
                let full = page.len() >= usize::from(MAX_PAGE);
                if full && cursor.previous.as_ref() == Some(&page) {
                    return Err(Error::PagingStalled { path: E::PATH });
                }
                cursor.skip += u32::try_from(page.len()).unwrap_or(u32::MAX);
                cursor.done = !full;
                let items = page
                    .iter()
                    .cloned()
                    .map(serde_json::from_value::<E::Model>)
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                cursor.previous = Some(page);
                Ok(Some((items, cursor)))
            }
        })
        .map_ok(|items| stream::iter(items.into_iter().map(Ok)))
        .try_flatten()
    }
}

impl<E: Findable> Resource<'_, E> {
    pub async fn find(&self, id: Id<E::Entity>) -> Result<E::Model> {
        self.client.get(&Self::item(id), &Query::new()).await
    }
}

impl<E: Creatable> Resource<'_, E> {
    pub async fn create(&self, input: &E::Input) -> Result<E::Output> {
        self.client.post(E::PATH, input).await
    }
}

impl<E: Updatable> Resource<'_, E> {
    pub async fn update(&self, id: Id<E::Entity>, input: &E::Input) -> Result<E::Output> {
        self.client.put(&Self::item(id), input).await
    }
}

impl<E: Deletable> Resource<'_, E> {
    pub async fn delete(&self, id: Id<E::Entity>) -> Result<()> {
        self.client.delete(&Self::item(id)).await
    }
}
