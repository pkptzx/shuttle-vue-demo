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
    let in_lang:InputLang = "zh-CN".into();
println!("{:?}",in_lang);
let out_lang:OutputLang = "zh-CN".into();
println!("{:?}",out_lang);
let out_lang:OutputLang = "ja".into();
println!("{:?}",out_lang.to_string());
// let s = qr::translate::translate_one_line("hi".to_string(), InputLang::Auto, OutputLang::SimplifiedChinese).await;
// println!("{}",s.unwrap());

let l:TST = "s-1".into();
println!("test: {:?}",l);


Ok(())
}

impl Into<TST> for &str {
    fn into(self) -> TST {
        println!("into: {}",self);
        let data = self.to_lowercase();
        // let data = self;
        // match &*caseless::default_case_fold_str(self) {
            match data.as_str() {
            "S-1" => TST::ONE,
            "S-2" => TST::TWO,
            "s-3" => TST::THREE,
            _ => TST::default(),
        }
    }
}