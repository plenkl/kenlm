
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![feature(libc)]
#[cfg(test)]

mod kenlm;
mod bindings;
mod tests {
    use super::*;


    #[test]
    fn lm_load() {
        let kenlm_model = kenlm::KenLM::from_file("src/test.arpa").unwrap();
        println!{"Score : {:?}", kenlm_model.perplexity("screening a little")};
    }
}
