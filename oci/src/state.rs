use std::collections::HashMap;

pub struct State{
    pub Version:String,
    pub ID:String,
    pub Status:String,
    pub Pid:i32,
    pub Bundle:String,
    pub Annotations:HashMap<String,String>,
}