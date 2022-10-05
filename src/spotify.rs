#![allow(dead_code,unused_variables)]
use crate::{errors::SpocError, utils::run};

#[derive(Default)]
pub struct Spotify {
    pub curr_playing: String,
}

impl Spotify {
    pub fn currently_playing(&mut self) -> Result<(), SpocError> {
        let out = run("spt", ["playback", "-s", "-f", "%t by %a"])?;
        let cur_playing = std::string::String::from_utf8(out.stdout)?;
        self.curr_playing = cur_playing;
        Ok(())
    }
    pub fn toggle(&mut self) -> Result<(), SpocError> {
        let out = run("spt", ["playback", "--toggle"])?;
        self.curr_playing = std::string::String::from_utf8(out.stdout)?;
        Ok(())
    }
    pub fn like(&self) -> Result<(), SpocError> {
        run("spt", ["playback", "--like"])?;
        Ok(())
    }
    pub fn search(&self, term: impl ToString) {}
}
