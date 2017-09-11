#include "load.h"
#include "../lm/model.hh"
#include "../lm/virtual_interface.hh"
#include "../lm/state.hh"

using namespace lm::ngram;
kenlm_model load_kenlm_model(const char *file_name) {
    ModelType m = ModelType::PROBING;
    Config config = Config();
    try {
        lm::base::Model* model = lm::ngram::LoadVirtual(file_name, config, m);
        return reinterpret_cast<kenlm_model>(model);
    }
    catch (const std::exception& e) {
        return kenlm_model(NULL);
    }
    return kenlm_model(NULL);
}

void destroy_kenlm_model(kenlm_model model) {
    free(model);
}

kenlm_vocabulary kenlm_get_vocabulary(kenlm_model model) {
    lm::base::Model* m = reinterpret_cast<lm::base::Model*>(model);
    return reinterpret_cast<kenlm_vocabulary>(&m->BaseVocabulary());
}

unsigned int kenlm_vocabulary_index(kenlm_vocabulary vocab, const char* index) {
    const lm::ngram::Vocabulary* v = reinterpret_cast<const lm::ngram::Vocabulary*>(vocab);
    return v->Index(index);

}

unsigned int kenlm_vocabulary_begin_sentence(kenlm_vocabulary vocab) {
    const lm::ngram::Vocabulary* v = reinterpret_cast<const lm::ngram::Vocabulary*>(vocab);
    return v->BeginSentence();
}

unsigned int kenlm_vocabulary_end_sentence(kenlm_vocabulary vocab){
    const lm::ngram::Vocabulary* v = reinterpret_cast<const lm::ngram::Vocabulary*>(vocab);
    return v->EndSentence();
}


struct kenlm_state {
    using Type=lm::ngram::State;

    explicit kenlm_state(const Type &obj)
            : _state(obj) {}

    Type _state;
};


kenlm_state* kenlm_create_state() {
    return new kenlm_state(State());
}

void kenlm_destroy_state(kenlm_state* state) {
    delete state;
}

long unsigned int kenlm_hash_state(kenlm_state* state) {
    return lm::ngram::hash_value(state->_state);
}

void kenlm_model_begin_sentence_write(kenlm_model model, kenlm_state *state){
    lm::base::Model* m = reinterpret_cast<lm::base::Model*>(model);
    m->BeginSentenceWrite(&state->_state);
}

void kenlm_model_null_context_write(kenlm_model model, kenlm_state *state){
    lm::base::Model* m = reinterpret_cast<lm::base::Model*>(model);
    m->NullContextWrite(&state->_state);
}

float kenlm_model_base_score(kenlm_model model, kenlm_vocabulary vocab, kenlm_state* in_state, unsigned int word, kenlm_state* out_state) {
    lm::base::Model* m = reinterpret_cast<lm::base::Model*>(model);
    m->BaseScore(&in_state->_state, word, &out_state->_state);
    
}
