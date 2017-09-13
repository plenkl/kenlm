#ifndef C_API_LOAD_H
#define C_API_LOAD_H

#ifdef __cplusplus
extern "C" {
#endif


struct kenlm_config;
struct kenlm_state;

typedef void* kenlm_model;
typedef const void* kenlm_vocabulary;

kenlm_vocabulary kenlm_get_vocabulary(kenlm_model);
unsigned int kenlm_vocabulary_index(kenlm_vocabulary, const char*);
unsigned int kenlm_vocabulary_begin_sentence(kenlm_vocabulary);
unsigned int kenlm_vocabulary_end_sentence(kenlm_vocabulary);
unsigned int kenlm_vocabulary_not_found(kenlm_vocabulary);


kenlm_model load_kenlm_model(const char *); 
void destroy_kenlm_model(kenlm_model); 

kenlm_state*  kenlm_create_state();
kenlm_state*  kenlm_copy_state(kenlm_state*);
void kenlm_destroy_state(kenlm_state*);
long unsigned int kenlm_hash_state(kenlm_state*);
void kenlm_model_begin_sentence_write(kenlm_model model, kenlm_state *state);
void kenlm_model_null_context_write(kenlm_model model, kenlm_state *state);
float kenlm_model_base_score(kenlm_model model, kenlm_vocabulary vocab, kenlm_state* in_state, unsigned int word, kenlm_state* out_state); 


#ifdef __cplusplus
} /* end extern "C" */
#endif
#endif
