// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::{sync::Mutex, string, vec};
use sqlite;
use tauri::{App, State, async_runtime::block_on};
use reqwest::Client;
use regex::Regex;
use std::collections::HashMap;
struct AppState{
    connect:Mutex<sqlite::Connection>
}
impl AppState {
    fn new()->AppState{
        AppState { connect:Mutex::new(sqlite::open("data.db").unwrap()) }
    }
}
fn main() {
    let state=AppState::new();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_print,get_prints,del_print,
            add_rule,get_rules,del_rule,get_rule_list,
            get_prints_data,
            ])
        .manage(state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
fn add_print(state:State<AppState>,url:&str,printname:&str,rulename:&str)->String{
    let connect=state.connect.lock().unwrap();
    let _a=connect.execute("Create TABLE prints (url TEXT,printname TEXT,rulename TEXT)");
    if let Ok(_) =_a  {
        println!("Create prints table succeed");
    }
    else{
        println!("已经有了 prints 表");
    }
    let _a=connect.execute(format!("INSERT INTO prints VALUES ('{}','{}','{}')",url,printname,rulename));
    if let Ok(_) =_a  {
        return format!("添加打印机{}成功",printname);
    }
    else{
        return format!("添加打印机{}失败",printname);
    }
}
#[tauri::command]
fn del_print(state:State<AppState>,url:&str)->String{
    let connect=state.connect.lock().unwrap();
    let _a=connect.execute(format!("DELETE FROM prints WHERE url = '{}' ;",url));
    if let Ok(_) =_a  {
        return format!("删除打印机{}成功",url);
    }
    else{
        return format!("删除打印机{}失败",url);
    }
}
#[tauri::command]
fn get_prints(state:State<AppState>)->Vec<Vec<String>>{
    let mut vs=Vec::new();
    let connect=state.connect.lock().unwrap();
    let query="SELECT * FROM prints" ;
    let _a=connect.iterate(query, |pairs| {
        let mut vss=Vec::new();
        let (mut url,mut printname,mut rulename)=(String::new(),String::new(),String::new());
        for &(name, value) in pairs.iter() {
            if name=="url"{
                url=value.unwrap().to_string();
            }
            if name=="printname"{
                printname=value.unwrap().to_string();
            }
            if name=="rulename"{
                rulename=value.unwrap().to_string();
            }
            if (url!="")&&(printname!="")&&(rulename!=""){
                vss.push(url.clone());
                vss.push(printname.clone());
                vss.push(rulename.clone());
                vs.push(vss.clone());
                vss.clear();
                url.clear();printname.clear();rulename.clear();
            }
        }
        true
    });
    if let Ok(_) =_a  {
    }
    else{
        println!("搜索prints表失败");
    }
    return vs;
}
#[tauri::command]
fn get_rules(state:State<AppState>)->Vec<Vec<String>>
{
    let mut vs=Vec::new();
    let connect=state.connect.lock().unwrap();
    let query="SELECT * FROM rules" ;
    let _a=connect.iterate(query, |pairs| {
        let mut vss=Vec::new();
        let (mut rule,mut rulename)=(String::new(),String::new());
        for &(name, value) in pairs.iter() {
            if name=="rule"{
                rule=value.unwrap().to_string();
            }
            if name=="rulename"{
                rulename=value.unwrap().to_string();
            }
            if (rule!="")&&(rulename!=""){
                vss.push(rulename.clone());
                vss.push(rule.clone());
                vs.push(vss.clone());
                vss.clear();
                rule.clear();rulename.clear();
            }
        }
        true
    });
    if let Ok(_) =_a  {
    }
    else{
        println!("搜索rules表失败");
    }
    return vs;
}
#[tauri::command]
fn get_rule_list(state:State<AppState>)->Vec<String>
{
    let mut output=Vec::new();
    let vs=get_rules(state);
    for i in vs{
        output.push(i[0].clone());
    }
    output
}
#[tauri::command]
fn add_rule(state:State<AppState>,rulename:&str,rule:&str)->String{
    let connect=state.connect.lock().unwrap();
    let _a=connect.execute("Create TABLE rules (rulename TEXT,rule TEXT)");
    if let Ok(_) =_a  {
        println!("Create prints table succeed");
    }
    else{
        println!("已经有了 rules 表");
    }
    let _a=connect.execute(format!("INSERT INTO rules VALUES ('{}','{}')",rulename,rule));
    if let Ok(_) =_a  {
        return format!("添加规则{}成功",rulename);
    }
    else{
        return format!("添加规则{}失败",rulename);
    }
}
#[tauri::command]
fn del_rule(state:State<AppState>,rulename:&str)->String{
    let connect=state.connect.lock().unwrap();
    let _a=connect.execute(format!("DELETE FROM rules WHERE rulename = '{}' ;",rulename));
    if let Ok(_) =_a  {
        return format!("删除规则{}成功",rulename);
    }
    else{
        return format!("删除规则{}失败",rulename);
    }
}
#[tauri::command]
async fn get_prints_data(state:State<'_,AppState>)->Result<Vec<Vec<Vec<String>>>,()>{
    let prints_data=get_prints(state.clone());
    let rules_data=get_rules(state.clone());
    let mut datalist=Vec::new();
    let mut output=Vec::new();
    //将rules_data转换为hashmap
    let mut rules_map=HashMap::new();
    for rdata in rules_data{
        rules_map.insert(rdata[0].clone(), rdata[1].clone());
    }
    for pdata in prints_data{
        //println!("debug:{:?}",pdata);
        let regex_strO=rules_map.get(pdata[2].as_str());
        if let Some(regex_str) =regex_strO  {
            datalist.push(vec![pdata[0].clone(),pdata[1].clone(),regex_str.to_string()]);
        }
        else {
            println!("执行 失败 {:?}",pdata);
        }
    }
    for i in datalist{
        output.push(get_print_data(i[0].clone(),i[1].clone(), i[2].clone()).await)
    }

    Ok(output)
}

async fn get_print_data(link:String,loacl:String,regex_str:String) -> Vec<Vec<String>>
{
    // 创建一个 actix-web 客户端
    //println!("DeBug: {} {} {}",link,loacl,regex_str);
    let client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(5))
    .build()
    .unwrap();

    // 定义要请求的 URL
    let url = link.as_str();
    let mut v:Vec<Vec<String>>=Vec::new();//数据存储位置
    // 发送请求并等待响应
    v.push(vec!["位置".to_string(),loacl.to_string()]);
    let res_result = client.get(url).send().await;
    if let Ok(res) =res_result  {
        let body=res.text().await;
        //println!("{}",body);
        if let Ok(x) = body {
            let restr=regex_str.as_str();
            let re = Regex::new(restr).unwrap();
            
            for  i in re.captures_iter(x.as_str()){
                let vi=vec![i.get(1).unwrap().as_str().to_string(),i.get(2).unwrap().as_str().to_string()];
                v.push(vi);
            }
            // 输出 HTML 内容
            return v;
        }
        else{
            v.push(vec!["连接超时".to_string(),"".to_string()]);
            return v;
        }
    }
    else{
        v.push(vec!["连接超时".to_string(),"".to_string()]);
        return v;
    }
}