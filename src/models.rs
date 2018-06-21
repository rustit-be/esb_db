use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::schema::system;
use super::schema::faction;

#[derive(Debug, Queryable)]
pub struct Allegiance {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct Economy {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct Government {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct Power {
    pub id: i32,
    pub name: String,
    pub allegiance_id: i32,
}

#[derive(Debug, Queryable)]
pub struct PowerState {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct ReserveType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct Security {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct State {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name="system"]
pub struct System {
    pub id: i32,
    pub name: String,
    pub allegiance_id: Option<i32>,
    pub state_id: Option<i32>,
    pub government_id: Option<i32>,
    pub security_id: Option<i32>,
    pub needs_permit: Option<bool>,
    pub power_state_id: Option<i32>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub simbad_ref: Option<String>,
    pub reserve_type_id: Option<i32>,
    pub is_populated: Option<bool>,
    pub edsm_id: Option<i32>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name="faction"]
pub struct Faction {
    pub id:i32,
    pub name:String,
    pub allegiance_id: Option<i32>,
    pub state_id: Option<i32>,
    pub government_id: Option<i32>,
    pub home_system_id: Option<i32>,
    pub is_player_faction: bool,
    pub updated_at: DateTime<Utc>,
}

impl Faction {
    pub fn exists(connection:&PgConnection, faction_id:i32) -> QueryResult<bool> {
        use schema::faction;
        use diesel::dsl::count_star;
        let c:i64 = faction::dsl::faction.filter(faction::dsl::id.eq(faction_id))
            .select(count_star())
            .first(connection)?;
        Ok(c > 0)
    }
}
