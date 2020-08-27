use neovim_lib::{Neovim, NeovimApi, Session};
use std::convert::TryInto;

enum Messages {
    NamedTuple,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "namedtuple" => Messages::NamedTuple,
            _ => Messages::Unknown(event),
        }
    }
}

/// EventHandler receives RPC requests.
pub struct EventHandler {
    nvim: Neovim,
}

// TODO: checkout https://github.com/boxofrox/neovim-scorched-earth to get some inspiration
impl EventHandler {
    pub fn new() -> EventHandler {
        // unwrap safe because new_parent always returns Ok
        let mut session = Session::new_parent().unwrap();
        session.set_infinity_timeout();
        let nvim = Neovim::new(session);

        EventHandler { nvim }
    }

    pub fn handle_events(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, _values) in receiver {
            match Messages::from(event) {
                Messages::NamedTuple => {
                    let class_tmp = "from typing import NamedTuple\n\
                                     \n\
                                     class Tmp(NamedTuple):";
                    self.write_to_cur_buffer(class_tmp);
                }

                // Handle any "Unknown" messages.
                Messages::Unknown(ev) => {
                    self.nvim
                        .command(&format!("echo \"Invalid command: {}\"", ev))
                        .unwrap();
                }
            }
        }
    }

    fn write_to_cur_buffer(&mut self, text: &str) {
        let win = self.nvim.get_current_win().unwrap();
        let (line, _) = win.get_cursor(&mut self.nvim).unwrap();
        let buffer = self.nvim.get_current_buf().unwrap();

        for (i, l) in text.split("\n").enumerate() {
            let i: i64 = i.try_into().unwrap();
            let replacement = vec![l.to_owned()];

            match buffer.set_lines(&mut self.nvim, line + i, line + i, true, replacement) {
                Err(e) => self.nvim.command(&format!("echo \"{}\"", e)),
                Ok(f) => Ok(f),
            }
            .unwrap();
        }
    }
}
