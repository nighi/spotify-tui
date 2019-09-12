mod album_list;
mod album_tracks;
mod common_key_events;
mod empty;
mod error_screen;
mod help_menu;
mod home;
mod input;
mod library;
mod playlist;
mod recently_played;
mod search_results;
mod select_device;
mod song_table;

use super::app::{ActiveBlock, App};
use termion::event::Key;

pub fn handle_app(app: &mut App, key: Key) {
    // Match events for different app states
    let current_route = app.get_current_route();
    match current_route.active_block {
        ActiveBlock::Artist => {
            // TODO
        }
        ActiveBlock::Input => {
            input::handler(key, app);
        }
        ActiveBlock::MyPlaylists => {
            playlist::handler(key, app);
        }
        ActiveBlock::SongTable => {
            song_table::handler(key, app);
        }
        ActiveBlock::HelpMenu => {
            help_menu::handler(key, app);
        }
        ActiveBlock::Error => {
            error_screen::handler(key, app);
        }
        ActiveBlock::SelectDevice => {
            select_device::handler(key, app);
        }
        ActiveBlock::SearchResultBlock => {
            search_results::handler(key, app);
        }
        ActiveBlock::Home => {
            home::handler(key, app);
        }
        ActiveBlock::AlbumList => {
            album_list::handler(key, app);
        }
        ActiveBlock::AlbumTracks => {
            album_tracks::handler(key, app);
        }
        ActiveBlock::Library => {
            library::handler(key, app);
        }
        ActiveBlock::Empty => {
            empty::handler(key, app);
        }
        ActiveBlock::RecentlyPlayed => {
            recently_played::handler(key, app);
        }
    }
}
