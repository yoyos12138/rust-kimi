use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub mod request;
pub mod response;


#[skip_serializing_none]
#[derive(Serialize)]
pub struct Test<T=()>
where 
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub name:String,
    pub obj:Option<T>,
}

fn _test() {
    let _t1:Test<>= Test{
        name:"SXC258".to_string(),
        obj:None,
    };
}