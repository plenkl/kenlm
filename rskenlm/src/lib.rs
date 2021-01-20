#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![crate_type = "lib"]

mod bindings;
pub mod kenlm;

pub use crate::kenlm::{
    LanguageModel,
    State,
};

#[cfg(test)]
mod tests {
    use super::kenlm;

    #[test]
    fn lm_load() {
        let model = kenlm::LanguageModel::from_file("src/test.arpa").unwrap();
        println! {"Score : {:?}", model.perplexity("screening a little")};
    }
}
