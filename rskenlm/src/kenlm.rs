use crate::bindings::root as c_api;
use std::ffi::CString;
use std::fmt;

#[derive(Debug)]
#[repr(C)]
pub struct State {
    pub state: *mut c_api::kenlm_state,
}

pub enum LMError {
    LoadError,
}

impl fmt::Debug for LMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error loading file")
    }
}

impl State {
    pub fn new() -> Self {
        unsafe {
            State {
                state: c_api::kenlm_create_state(),
            }
        }
    }
}

impl Default for State {
    fn default() -> State {
        State::new()
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            c_api::kenlm_destroy_state(self.state);
        }
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        unsafe {
            State {
                state: c_api::kenlm_copy_state(self.state),
            }
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct LanguageModel {
    model: c_api::kenlm_model,
    vocab: c_api::kenlm_vocabulary,
}

unsafe impl Send for LanguageModel {}
unsafe impl Sync for LanguageModel {}

impl LanguageModel {
    pub fn from_file(filename: &str) -> Result<Self, LMError> {
        let fn_c = CString::new(filename).unwrap();
        unsafe {
            let model = c_api::load_kenlm_model(fn_c.as_ptr()) as *mut ::std::os::raw::c_void;
            if !model.is_null() {
                let vocab = c_api::kenlm_get_vocabulary(model);
                Ok(LanguageModel { model, vocab })
            } else {
                Err(LMError::LoadError)
            }
        }
    }

    pub fn score(&self, sentence: &str, bos: bool, eos: bool) -> f32 {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        unsafe {
            let mut state = State::new();
            let mut total = 0f32;
            if bos {
                c_api::kenlm_model_begin_sentence_write(self.model, state.state);
            } else {
                c_api::kenlm_model_null_context_write(self.model, state.state);
            }

            let out_state = State::new();
            for word in words {
                let word_c = CString::new(word).unwrap();
                let wid = c_api::kenlm_vocabulary_index(self.vocab, word_c.as_ptr());
                total += c_api::kenlm_model_base_score(
                    self.model,
                    self.vocab,
                    state.state,
                    wid,
                    out_state.state,
                );
                state = out_state.clone();
            }

            if eos {
                let out_state = State::new();
                total += c_api::kenlm_model_base_score(
                    self.model,
                    self.vocab,
                    state.state,
                    c_api::kenlm_vocabulary_end_sentence(self.vocab),
                    out_state.state,
                );
            }

            total
        }
    }

    pub fn perplexity(&self, sentence: &str) -> f32 {
        let word_count = (sentence.split_whitespace().count() + 1) as f32;
        10f32.powf(-self.score(sentence, true, true) / word_count)
    }

    pub fn begin_sentence_write(&self, state: &mut State) {
        unsafe {
            c_api::kenlm_model_begin_sentence_write(self.model, state.state);
        }
    }

    pub fn null_context_write(&self, state: &mut State) {
        unsafe {
            c_api::kenlm_model_null_context_write(self.model, state.state);
        }
    }

    pub fn vocab_index(&self, word: &str) -> u32 {
        unsafe {
            let word_c = CString::new(word).unwrap();
            c_api::kenlm_vocabulary_index(self.vocab, word_c.as_ptr())
        }
    }

    pub fn base_score(&self, in_state: &State, word: &str, out_state: &mut State) -> f32 {
        unsafe {
            let word_c = CString::new(word).unwrap();
            let wid = c_api::kenlm_vocabulary_index(self.vocab, word_c.as_ptr());

            c_api::kenlm_model_base_score(
                self.model,
                self.vocab,
                in_state.state,
                wid,
                out_state.state,
            )
        }
    }
}

impl Drop for LanguageModel {
    fn drop(&mut self) {
        unsafe {
            c_api::destroy_kenlm_model(self.model);
        }
    }
}
