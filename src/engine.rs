use ibus::{Engine, EngineExt, LookupTable, LookupTableExt, Text};

use glib::subclass::prelude::*;
use glib::{glib_object_impl, glib_object_subclass};

use emoji_searcher::{EmojiDb, EmojiSearcher};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use log::*;

use rand::distributions::{Distribution, Uniform};

fn swap_case(c: char) -> char {
    if c.is_uppercase() {
        c.to_lowercase().next().expect("Failed to swap case")
    } else {
        c.to_uppercase().next().expect("Failed to swap case")
    }
}

/// Randomizes case of all characters in s
fn randomize_string_case(s: &str) -> String {
    // Possibly inefficient copy to vector, but it's the easiest way to get
    // random indexes of characters
    let mut chars = s.chars().collect::<Vec<_>>();
    let mut rng = rand::thread_rng();

    let min = chars.len() / 4;
    let max = std::cmp::max(1, chars.len() * 3 / 4);
    let num_index_distribution = Uniform::from(min..max);
    let num_indexes = num_index_distribution.sample(&mut rng);

    let indexes = rand::seq::index::sample(&mut rng, chars.len(), num_indexes);
    indexes
        .into_iter()
        .for_each(|index| chars[index] = swap_case(chars[index]));

    chars.into_iter().collect()
}

/// Returns true if keyval/state combination has no effect on the text being
/// processed
fn keyval_is_ignored(keyval: u32, state: u32) -> bool {
    if state & ibus_sys::IBUS_CONTROL_MASK != 0 {
        return true;
    }
    let keychar = ibus::functions::keyval_to_unicode(keyval);

    keychar == std::char::from_u32(0).unwrap()
}

/// State related to text that is currently being processed
#[derive(Debug)]
struct ProcessingState {
    preedit_string: String,
    lookup_table: LookupTable,
    // In some cases we want to write out something different than what was
    // displayed in the lookup table. In those cases we map the index in the
    // lookup table to the text we actually want to display
    actual_text: HashMap<u32, String>,
}

/// Current state of engine
#[derive(Debug)]
enum EngineState {
    // State related to text that is currently being processed
    Processing(ProcessingState),
    // Indicates that we want to commit the text provided
    Selected(ibus::Text),
    // Default state. Ignored by engine
    Ignored,
}

/// Actual data of MemeboxEngine separated for interior mutability
struct Inner {
    engine_state: EngineState,
    emoji_searcher: EmojiSearcher,
}

/// Meme ibus engine. Allows for input of emojis and common formatted text of my
/// choosing
pub struct MemeboxEngine {
    // We know that only one thread can call into our engine, but rust doesn't
    // since it's an external C call
    inner: RefCell<Inner>,
}

impl MemeboxEngine {
    /// Creates a new engine
    fn new() -> MemeboxEngine {
        // FIXME: This should be cached locally so we don't have to reach out to
        // the net every time we start up. If the ibus daemon is started without
        // network it will crash
        let emoji_db = Rc::new(EmojiDb::from_web().unwrap());

        MemeboxEngine {
            inner: RefCell::new(Inner {
                engine_state: EngineState::Ignored,
                emoji_searcher: EmojiSearcher::new(emoji_db),
            }),
        }
    }

    /// Updates the current engine state based off the key inputs provided by ibus
    fn update_engine_state(inner: &mut Inner, keyval: u32, key_state: u32) {
        if let EngineState::Selected(_) = inner.engine_state {
            inner.engine_state = EngineState::Ignored;
        }

        if key_state & ibus_sys::IBUS_RELEASE_MASK != 0 {
            return;
        }

        let keychar = ibus::functions::keyval_to_unicode(keyval);
        let engine_state = &mut inner.engine_state;
        let emoji_searcher = &mut inner.emoji_searcher;

        match engine_state {
            EngineState::Ignored => {
                debug!("{}, {}", key_state, keychar);
                if (key_state & ibus_sys::IBUS_SHIFT_MASK != 0)
                    && (key_state & ibus_sys::IBUS_CONTROL_MASK != 0)
                    && keychar == 'E'
                {
                    *engine_state = EngineState::Processing(ProcessingState {
                        preedit_string: String::new(),
                        lookup_table: LookupTable::new(10, 0, true, true),
                        actual_text: HashMap::new(),
                    });
                }
            }
            EngineState::Processing(state) => {
                if keyval as i32 == ibus_sys::IBUS_KEY_Return
                    || keyval as i32 == ibus_sys::IBUS_KEY_Tab
                {
                    let output_idx = state.lookup_table.get_cursor_pos();
                    let output_text = if let Some(text) = state.actual_text.get(&output_idx) {
                        Text::from_string(text)
                    } else {
                        state.lookup_table.get_candidate(output_idx).unwrap()
                    };
                    *engine_state = EngineState::Selected(output_text);
                    return;
                } else if keyval as i32 == ibus_sys::IBUS_KEY_Escape {
                    *engine_state = EngineState::Ignored;
                    return;
                } else if keyval as i32 == ibus_sys::IBUS_KEY_Up {
                    state.lookup_table.cursor_up();
                    return;
                } else if keyval as i32 == ibus_sys::IBUS_KEY_Down {
                    state.lookup_table.cursor_down();
                    return;
                } else if keyval_is_ignored(keyval, key_state) {
                    return;
                } else if keyval as i32 == ibus_sys::IBUS_KEY_BackSpace {
                    state.preedit_string.pop();
                } else {
                    state.preedit_string.push(keychar);
                }

                let emojis = emoji_searcher.search(state.preedit_string.clone()).take(8);

                state.lookup_table.clear();
                state.actual_text.clear();

                for emoji in emojis {
                    state
                        .lookup_table
                        .append_candidate(&Text::from_string(&format!(
                            "{}: {}",
                            emoji.emoji, emoji.matched_tag
                        )));

                    let idx = state.lookup_table.get_number_of_candidates() - 1;
                    state.actual_text.insert(idx, emoji.emoji.clone());
                }

                state
                    .lookup_table
                    .append_candidate(&Text::from_string(&randomize_string_case(
                        &state.preedit_string,
                    )));

                state
                    .lookup_table
                    .append_candidate(&Text::from_string("🔲"));

                let idx = state.lookup_table.get_number_of_candidates() - 1;
                state
                    .actual_text
                    .insert(idx, crate::cube_drawer::draw(&state.preedit_string));
            }
            EngineState::Selected(_) => unreachable!(),
        }
    }

    /// Renders the current engine state to ibus
    fn render(&self, inner: &mut Inner) {
        // NOTE: inner could be borrowed here but most callers have already
        // borrowed it. Passing it as argument prevents an accidental double
        // borrow
        match &inner.engine_state {
            EngineState::Processing(state) => {
                let preedit = Text::from_string(&state.preedit_string);
                self.get_instance().update_preedit_text(&preedit, 0, true);

                self.get_instance().update_lookup_table(
                    &state.lookup_table,
                    state.lookup_table.get_number_of_candidates() > 0,
                );
            }
            EngineState::Selected(text) => {
                self.get_instance().hide_preedit_text();
                self.get_instance().hide_lookup_table();
                self.get_instance().commit_text(text);
            }
            EngineState::Ignored => {
                self.get_instance().hide_preedit_text();
                self.get_instance().hide_lookup_table();
            }
        }
    }

    /// ibus callback to process an incoming key
    fn process_key_event(&self, keyval: u32, _keycode: u32, state: u32) -> bool {
        let inner = &mut *self.inner.borrow_mut();

        debug!("{:?}", inner.engine_state);

        Self::update_engine_state(inner, keyval, state);

        debug!("{:?}", inner.engine_state);

        self.render(inner);

        match inner.engine_state {
            EngineState::Processing(_) | EngineState::Selected(_) => true,
            _ => false,
        }
    }

    /// ibus callback to process a cursor up event
    fn cursor_up(&self) {
        debug!("Cursor up");

        let inner = &mut *self.inner.borrow_mut();

        if let EngineState::Processing(ref mut state) = inner.engine_state {
            state.lookup_table.cursor_up();
        }

        self.render(inner);
    }

    /// ibus callback to process a cursor down event
    fn cursor_down(&self) {
        debug!("Cursor down");

        let inner = &mut *self.inner.borrow_mut();

        if let EngineState::Processing(ref mut state) = inner.engine_state {
            state.lookup_table.cursor_down();
        }

        self.render(inner);
    }

    /// ibus callback to process a focus in event
    /// Engine state is reset on focus in
    fn focus_in(&self) {
        debug!("Focus in");
        let inner = &mut *self.inner.borrow_mut();
        inner.engine_state = EngineState::Ignored;
        self.render(inner);
    }

    /// ibus callback to process a focus out event
    /// Engine state is reset on focus out
    fn focus_out(&self) {
        debug!("Focus out");
        let inner = &mut *self.inner.borrow_mut();
        inner.engine_state = EngineState::Ignored;
        self.render(inner);
    }

    /// ibus callback to process a candidate selection
    fn candidate_clicked(&self, index: u32, _button: u32, _state: u32) {
        let inner = &mut *self.inner.borrow_mut();

        if let EngineState::Processing(state) = &inner.engine_state {
            inner.engine_state =
                EngineState::Selected(state.lookup_table.get_candidate(index).unwrap());
        }
        self.render(inner);
    }

    /// ibus callback to reset engine state
    fn reset(&self) {
        let inner = &mut *self.inner.borrow_mut();
        inner.engine_state = EngineState::Ignored;
        self.render(inner);
    }
}

impl ObjectImpl for MemeboxEngine {
    glib_object_impl!();
}

impl ObjectSubclass for MemeboxEngine {
    glib_object_subclass!();

    const NAME: &'static str = "MemeboxEngine";
    type ParentType = Engine;
    type Instance = glib::subclass::simple::InstanceStruct<Self>;
    type Class = glib::subclass::simple::ClassStruct<Self>;

    fn new() -> Self {
        MemeboxEngine::new()
    }
}

macro_rules! gen_c_binding{
    ( $name:ident, $ret: ty $(, $arg:ident : $ty:ty )* ) => {
        unsafe extern "C" fn $name(
            ptr: *mut ibus_sys::IBusEngine,
            $($arg: $ty),* ) -> $ret {
                let instance = &*(ptr as *mut <MemeboxEngine as ObjectSubclass>::Instance);
                let imp = instance.get_impl();

                imp. $name ( $($arg),* ) as $ret
            }
    };
    ( $name:ident $(, $arg:ident : $ty:ty )* ) => {
        unsafe extern "C" fn $name(
            ptr: *mut ibus_sys::IBusEngine,
            $($arg: $ty),* ) {
                let instance = &*(ptr as *mut <MemeboxEngine as ObjectSubclass>::Instance);
                let imp = instance.get_impl();

                imp. $name ( $($arg),* );
            }
    };
}

gen_c_binding!(
    process_key_event,
    i32,
    keyval: u32,
    keycode: u32,
    state: u32
);
gen_c_binding!(cursor_up);
gen_c_binding!(cursor_down);
gen_c_binding!(focus_in);
gen_c_binding!(focus_out);
gen_c_binding!(candidate_clicked, index: u32, button: u32, state: u32);
gen_c_binding!(reset);

unsafe impl IsSubclassable<MemeboxEngine> for ibus::EngineClass {
    fn override_vfuncs(&mut self) {
        unsafe {
            let klass = &mut *(self as *mut Self as *mut ibus_sys::IBusEngineClass);
            klass.process_key_event = Some(process_key_event);
            klass.cursor_up = Some(cursor_up);
            klass.cursor_down = Some(cursor_down);
            klass.focus_in = Some(focus_in);
            klass.focus_out = Some(focus_out);
            klass.candidate_clicked = Some(candidate_clicked);
            klass.reset = Some(reset);
        }
    }
}
