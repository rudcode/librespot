use librespot::playback::player::PlayerEvent;
use log::info;
use std::collections::HashMap;
use std::io;
use std::process::Command;
use tokio_process::{Child, CommandExt};

fn run_program(program: &str, env_vars: HashMap<&str, String>) -> io::Result<Child> {
    let mut v: Vec<&str> = program.split_whitespace().collect();
    info!("Running {:?} with environment variables {:?}", v, env_vars);
    Command::new(&v.remove(0))
        .args(&v)
        .envs(env_vars.iter())
        .spawn_async()
}

pub fn run_program_on_events(event: PlayerEvent, onevent: &str) -> io::Result<Child> {
    let mut env_vars = HashMap::new();
    match event {
        PlayerEvent::Changed {
            old_track_id,
            new_track_id,
            track_meta,
            album_meta,
            artist_meta,
        } => {
            env_vars.insert("PLAYER_EVENT", "change".to_string());
            env_vars.insert("OLD_TRACK_ID", old_track_id.to_base62());
            env_vars.insert("TRACK_ID", new_track_id.to_base62());
            env_vars.insert("TRACK_NAME", track_meta.name.to_string());
            env_vars.insert("ARTIST_NAME", artist_meta.name.to_string());
            env_vars.insert("ALBUM_NAME", album_meta.name.to_string());
            env_vars.insert("ALBUM_COVER", album_meta.covers[0].to_string());
        }
        PlayerEvent::Started { 
            track_id,
            track_meta,
            album_meta,
            artist_meta,
        } => {
            env_vars.insert("PLAYER_EVENT", "start".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base62());
            env_vars.insert("TRACK_NAME", track_meta.name.to_string());
            env_vars.insert("ARTIST_NAME", artist_meta.name.to_string());
            env_vars.insert("ALBUM_NAME", album_meta.name.to_string());
            env_vars.insert("ALBUM_COVER", album_meta.covers[0].to_string());
        }
        PlayerEvent::Stopped { track_id } => {
            env_vars.insert("PLAYER_EVENT", "stop".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base62());
        }
    }
    run_program(onevent, env_vars)
}
