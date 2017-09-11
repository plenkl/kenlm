
use std::ffi::CString;
use std::ops::Deref;
use bindings::root::*;

#[derive(Debug, Clone)]
struct RustState {
    _state: *mut kenlm_state,
}

impl RustState {
    fn new() -> Self {
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


#[derive(Debug)]
pub struct KenLM {
    model: kenlm_model,
    vocab: kenlm_vocabulary,
}


impl KenLM {
    pub fn from_file(filename: &str) -> Self {
        let fn_c = CString::new(filename).unwrap();
        unsafe {
            let model = load_kenlm_model(fn_c.as_ptr()).into();
            let vocab = kenlm_get_vocabulary(model);
            KenLM { model, vocab }
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
}


impl Drop for KenLM {
    fn drop(&mut self) {
        unsafe {
            destroy_kenlm_model(self.model);
        }
    }
}
