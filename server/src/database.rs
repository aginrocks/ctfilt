use color_eyre::eyre::Result;
use mongodb::{Client, Database, bson::oid::ObjectId};
use partial_struct::Partial;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tower_sessions_redis_store::{
    RedisStore,
    fred::prelude::{ClientLike, Config, Pool},
};
use utoipa::ToSchema;
use visible::StructFields;

use crate::mongo_id::object_id_as_string_required;
use crate::settings::Settings;

macro_rules! database_object {
    ($name:ident $(<$($gen:tt),*>)? { $($field:tt)* }$(, $($omitfield:ident),*)?) => {
        #[derive(Partial, Debug, Serialize, Deserialize, ToSchema, Clone)]
        #[partial(omit(id $(, $($omitfield),* )?), derive(Debug, Serialize, Deserialize, ToSchema, Clone))]
        #[StructFields(pub)]
        pub struct $name $(<$($gen),*>)? {
            $($field)*
        }
    };
}

pub async fn init_database(settings: &Settings) -> Result<Database> {
    let client = Client::with_uri_str(&settings.db.connection_string).await?;
    let database = client.database(&settings.db.database_name);

    Ok(database)
}

pub async fn init_session_store(
    settings: &Settings,
) -> Result<SessionManagerLayer<RedisStore<Pool>>> {
    let config = Config::from_url(&settings.redis.connection_string)?;
    let pool = Pool::new(config, None, None, None, 6)?;

    let _redis_conn = pool.connect();
    pool.wait_for_connect().await?;

    let session_store = RedisStore::<Pool>::new(pool);

    // let client = SessionClient::with_uri_str(&settings.db.connection_string).await?;
    // let session_store = MongoDBStore::new(client, settings.db.database_name.clone());

    // let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(120)));

    Ok(session_layer)
}

database_object!(User {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    subject: String,
    name: String,
    email: String,
});

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ChallengeType {
    /// A fully static challenge with one answer
    Static,

    /// A challenge with a custom validator
    // Dynamic,

    /// A challenge that requires VPN use and is created per user
    Container,
}

// Had to use untagged enums for JsonSchema compatibility
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ChallengeFlag {}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
#[serde(rename = "static")]
pub struct Static;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
pub struct ChallengeFlagStatic {
    r#type: Static,
    /// The static flag for the challenge
    flag: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
#[serde(rename = "dynamic")]
pub struct Dynamic;

pub struct ChallengeFlagDynamic {
    r#type: Dynamic,

    /// Where the flag should be mounted inside the container
    mount_path: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
pub struct ChallengeMetadata {
    /// A short unique name for the challenge
    name: String,

    /// A URL-friendly unique identifier for the challenge
    slug: String,

    /// Markdown description of the challenge
    description: String,

    r#type: ChallengeType,
}

database_object!(Challenge {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(flatten)]
    metadata: ChallengeMetadata,
});
