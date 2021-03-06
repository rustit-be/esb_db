// (c) 2018 Joost Yervante Damad <joost@damad.be>

use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Deserializer};

use model;
use model::State;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct System {
    pub id: i32,
    pub name: String,
    pub population:Option<i64>,
    pub allegiance_id: Option<i32>,
    pub allegiance:Option<String>,
    #[serde(deserialize_with = "null_default")]
    #[serde(default)]
    pub state_id: i32,
    #[serde(deserialize_with = "null_default")]
    #[serde(default)]
    pub state:State,
    pub government_id: Option<i32>,
    pub government:Option<String>,
    pub security_id: Option<i32>,
    pub security:Option<String>,
    pub primary_economy_id: Option<i32>,
    pub primary_economy:Option<String>,
    pub needs_permit: Option<bool>,
    pub power_state_id: Option<i32>,
    pub power_state:Option<String>,
    pub power:Option<String>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub simbad_ref: Option<String>,
    pub controlling_minor_faction_id: Option<i32>,
    pub controlling_minor_faction: Option<String>,
    pub reserve_type_id: Option<i32>,
    pub reserve_type:Option<String>,
    pub is_populated: Option<bool>,
    pub edsm_id: Option<i32>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub updated_at: DateTime<Utc>,
    pub minor_faction_presences:Vec<MinorFactionPresence>,
}

impl Into<model::System> for System {
    fn into(self) -> model::System {
        model::System {
            id:self.id,
            name:self.name,
            security_id:self.security_id,
            needs_permit:self.needs_permit,
            x:self.x,
            y:self.y,
            z:self.z,
            simbad_ref:self.simbad_ref,
            reserve_type_id:self.reserve_type_id,
            is_populated:self.is_populated,
            edsm_id:self.edsm_id,
            updated_at:Some(self.updated_at),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct MinorFactionPresence {
    #[serde(deserialize_with = "null_default")]
    #[serde(default)]
    pub state_id:i32,
    #[serde(deserialize_with = "null_default")]
    #[serde(default)]
    pub influence:f32,
    pub minor_faction_id:i32,
    #[serde(deserialize_with = "null_default")]
    #[serde(default)]
    pub state:State,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Faction {
    pub id:i32,
    pub name:String,
    pub allegiance_id: Option<i32>,
    pub allegiance: Option<String>,
    #[serde(deserialize_with = "null_default")]
    #[serde(default)]
    pub state_id: i32,
    #[serde(deserialize_with = "null_default")]
    #[serde(default)]
    pub state: State,
    pub government_id: Option<i32>,
    pub government: Option<String>,
    pub home_system_id: Option<i32>,
    pub is_player_faction: bool,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub updated_at: DateTime<Utc>,
}

impl Into<model::Faction> for Faction {
    fn into(self) -> model::Faction {
        model::Faction {
            id:self.id,
            name:self.name,
            allegiance_id:self.allegiance_id,
            government_id:self.government_id,
            home_system_id:self.home_system_id,
            is_player_faction:self.is_player_faction,
            updated_at:self.updated_at,
        }
    }
}

pub fn deserialize_datetime<'de, D>(deserializer:D) -> Result<DateTime<Utc>, D::Error>
    where D: Deserializer<'de>
{
    let i = i64::deserialize(deserializer)?;
    Ok(Utc.timestamp(i, 0))
}


pub fn null_default<'de, D, T>(deserializer:D) -> Result<T, D::Error>
    where D: Deserializer<'de>, T:Default+Deserialize<'de>
{
    match Option::<T>::deserialize(deserializer)? {
        None => Ok(T::default()),
        Some(a) => Ok(a),
    }
}
