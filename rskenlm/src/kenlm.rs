use bindings::root::*;
use std::ffi::CString;
use std::ops::Deref;
use std::fmt;
use std::ptr;

#[derive(Debug)]
pub struct RustState {
    pub _state: *mut kenlm_state,
}

pub enum LMError {
    LoadError,
}

impl fmt::Debug for LMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error loading file")
    }
}


impl RustState {
    pub fn new() -> Self {
        unsafe { RustState { _state: kenlm_create_state().into() } }
    }
}

impl Drop for RustState {
    fn drop(&mut self) {
        unsafe {
            kenlm_destroy_state(self._state);
        }
    }
}

impl Clone for RustState {
    fn clone(&self) -> Self {
        unsafe {
            let rs = RustState { _state: kenlm_copy_state(self._state) };
            return rs;
        }
    }
}

#[derive(Debug)]
pub struct KenLM {
    model: kenlm_model,
    vocab: kenlm_vocabulary,
}

unsafe impl Send for KenLM { }
unsafe impl Sync for KenLM { }

impl KenLM {
    pub fn from_file(filename: &str) -> Result<Self, LMError> {;
        let fn_c = CString::new(filename).unwrap();
        unsafe {
            let model = load_kenlm_model(fn_c.as_ptr()) as *mut ::std::os::raw::c_void;
            if !model.is_null() {
                let vocab = kenlm_get_vocabulary(model);
                return Ok(KenLM { model, vocab });
            }
            return Err(LMError::LoadError);
        }
    }

    pub fn score(&self, sentence: &str, bos: bool, eos: bool) -> f32 {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        unsafe {
            let mut state = RustState::new();
            let mut total = 0f32;
            if bos {
                kenlm_model_begin_sentence_write(self.model, state._state);
            } else {
                kenlm_model_null_context_write(self.model, state._state);
            }

            let mut out_state = RustState::new();
            for word in words {
                let word_c = CString::new(word).unwrap();
                let wid = kenlm_vocabulary_index(self.vocab, word_c.as_ptr());
                total += kenlm_model_base_score(self.model,
                                                self.vocab,
                                                state._state,
                                                wid,
                                                out_state._state);
                state = out_state.clone();

            }

            if eos {
                let mut out_state = RustState::new();
                total += kenlm_model_base_score(self.model,
                                                self.vocab,
                                                state._state,
                                                kenlm_vocabulary_end_sentence(self.vocab),
                                                out_state._state);
            }

            return total;
        }
    }

    pub fn perplexity(&self, sentence: &str) -> f32 {
        let word_count = (sentence.split_whitespace().count() + 1) as f32;
        return 10f32.powf(-self.score(sentence, true, true) / word_count);
    }

    pub fn begin_sentence_write(&self, state: &mut RustState) {
        unsafe {
            kenlm_model_begin_sentence_write(self.model, state._state);
        }
    }

    pub fn null_context_write(&self, state: &mut RustState) {
        unsafe {
            kenlm_model_null_context_write(self.model, state._state);
        }
    }
    
    pub fn vocab_index(&self, word: &str) -> u32 {
        unsafe {
            let word_c = CString::new(word).unwrap();
            kenlm_vocabulary_index(self.vocab, word_c.as_ptr())
        }
    }

    pub fn base_score(&self,
                      in_state: &mut RustState,
                      word: &str,
                      out_state: &mut RustState)
                      -> f32 {
        unsafe {
            let word_c = CString::new(word).unwrap();
            let wid = kenlm_vocabulary_index(self.vocab, word_c.as_ptr());

            kenlm_model_base_score(self.model,
                                   self.vocab,
                                   in_state._state,
                                   wid,
                                   out_state._state)
        }

    }
}


impl Drop for KenLM {
    fn drop(&mut self) {
        unsafe {
            destroy_kenlm_model(self.model);
        }
    }
}
