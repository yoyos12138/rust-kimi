use rust_kimi::{request::{KimiModel, Role}, Test};

fn main() {
    let model1 = KimiModel::MoonshotV1Auto;
    let str1 = serde_json::to_string(&model1).unwrap();
    println!("{}", str1);

    let _r2=Role::Assistant;
    let _str2=serde_json::to_string(&_r2).unwrap();
    println!("{}", _str2);
    
    let m3=KimiModel::Other("sadasda".to_string());
    let str3=serde_json::to_string(&m3).unwrap();
    println!("{}", str3);

    let t4:Test<>=Test{
        name:"111".to_string(),
        obj:Option::None
    };
    let str4=serde_json::to_string(&t4).unwrap();
    println!("{}", str4);
    let m5=KimiModel::Other("sadasda".to_string());
    let str5=serde_json::to_string(&m5).unwrap();
    println!("{}", str5);
    let m6:KimiModel=serde_json::from_str(&str5).unwrap();
    println!("{:?}", m6);
    
}
