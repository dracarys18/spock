mod errors;
mod spotify;
mod utils;

use nvim_oxi::{
    self as oxi,
    api::{self, opts::CreateCommandOpts, types::CommandArgs},
};

fn toggle() -> oxi::Result<()> {
    let toggle = |_: CommandArgs| {
        //TODO: Can't do this
        let mut spotify = spotify::Spotify::default();
        spotify.toggle().ok();
        Ok(())
    };
    api::create_user_command("SpotifyToggle", toggle, &CreateCommandOpts::default())?;
    Ok(())
}

#[oxi::module]
fn spoc() -> oxi::Result<()> {
    toggle()?;
    Ok(())
}
