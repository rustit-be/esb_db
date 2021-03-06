// (c) 2018 Joost Yervante Damad <joost@damad.be>

extern crate badlog;
#[macro_use]
extern crate log;
extern crate clap;
extern crate diesel;
extern crate esb_db;
extern crate serde_json;

use self::esb_db::*;
use self::model::*;
use self::diesel::prelude::*;

use std::fs::File;

fn main() {
    badlog::minimal(Some("DEBUG"));
    let a = clap::App::new("load_factions")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joost Yervante Damad <joost@damad.be>")
        .arg(clap::Arg::with_name("FILENAME")
             .required(true)
             .index(1)
             .help("JSON file name"))
        .arg(clap::Arg::with_name("force")
             .short("f")
             .long("force")
             .help("force overwrite"))
        ;

    let m = a.get_matches();
    let n = m.value_of("FILENAME").unwrap();
    let force = m.is_present("force");
    
    let f = File::open(&n).unwrap();
    let json_factions:Vec<eddb::Faction> = serde_json::from_reader(f).unwrap();
    info!("{} factions loaded into memory.", json_factions.len());
    
    use esb_db::schema::faction::dsl::*;
    use esb_db::schema::faction;

    let connection = establish_connection();
    let mut c_stored:i32 = 0;
    let mut c_updated:i32 = 0;
    for json_faction in json_factions {

        let state_id = json_faction.state_id;
        let state_name = json_faction.state.clone();
        let mut f:Faction = json_faction.into();

        // insert or update faction as needed
        
        let existing_faction_opt = Faction::exists(&connection, f.id).expect("Error finding faction");
        
        if let Some(existing_faction) = existing_faction_opt {
            if existing_faction.updated_at < f.updated_at || force {
                let _:Faction = diesel::update(faction.filter(id.eq(f.id)))
                    .set(&f)
                    .get_result(&connection)
                    .expect("Error updating faction");
                c_updated += 1;
            }
        } else {
            let _:Faction = diesel::insert_into(faction::table)
                .values(&f)
                .get_result(&connection)
                .expect("Error saving faction");
            c_stored += 1;
        }

        // update faction state if needed
        use esb_db::schema::faction_state::dsl::{stamp, faction_state, faction_id};
        let result = faction_state.filter(faction_id.eq(f.id))
            .order(stamp.desc())
            .first::<FactionState>(&connection)
            .optional()
            .expect("Error loading faction state");
        let (insert,first) = if let Some(res) = result {
            (res.state_id != state_id, false)
        } else {
            (true,true)
        };
        if insert {
            let i = FactionStateInsert {
                faction_id:f.id,
                state_id:state_id,
                stamp:f.updated_at,
            };
            diesel::insert_into(esb_db::schema::faction_state::table)
                .values(&i)
                .execute(&connection)
                .expect("Error saving faction_state");
            if !first {
                info!("Faction {} new state {:?}.", f.name, state_name);
            }
        }
    }
    info!("{} factions stored.", c_stored);
    info!("{} factions updated.", c_updated);
}
