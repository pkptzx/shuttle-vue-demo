use std::str::FromStr;

use anyhow::{Result, Ok};
use qr::translate::{InputLang, OutputLang};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum TST {
    #[default]
    NOTGOOD, 
    ONE, 
    TWO, 
    THREE,
}
#[tokio::main]
async fn main() -> Result<()> {
    let inLang:InputLang = "zh-CN".into();
println!("{:?}",inLang);
let outlang:OutputLang = "zh-CN".into();
println!("{:?}",outlang);
let outlang:OutputLang = "ja".into();
println!("{:?}",outlang.to_string());
// let s = qr::translate::translate_one_line("hi".to_string(), InputLang::Auto, OutputLang::SimplifiedChinese).await;
// println!("{}",s.unwrap());

let s = "striNgsx";
let k = match &*caseless::default_case_fold_str(s) {
    "str-ing1" => TST::ONE,
   "striNgsx" => TST::TWO,
    "string-3" | "else" => TST::THREE,
    _ => TST::default(),
};
println!("------k: {:?}",k);

let l:TST = "s-1".into();
println!("test: {:?}",l);


Ok(())
}

impl Into<TST> for &str {
    fn into(self) -> TST {
        println!("into: {}",self);
        // let data = self.to_lowercase();
        // let data = self;
        match &*caseless::default_case_fold_str(self) {
            // match self {
            "S-1" => TST::ONE,
            "S-2" => TST::TWO,
            "s-3" => TST::THREE,
            _ => TST::default(),
        }
    }
}