#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::Rng;
use std::fs::File;
use std::io::Write;

static GLOBAL: i32 = 100;
static ATTACK_TEXT: [&str; 5] = [
    "你发动了平平无奇的攻击，怪物剩余血量：",
    "你一个滑铲，怪物剩余血量：",
    "你拿起宝剑跳劈，怪物剩余血量：",
    "你对怪物使用刺拳，怪物剩余血量：",
    "你对怪物使用了精神攻击，怪物剩余血量：",
];
static LEN: usize = ATTACK_TEXT.len();

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn attack(health: &str) -> (String, String) {
    let damage = rand::thread_rng().gen_range(1..10);
    let index = rand::thread_rng().gen_range(1..LEN);
    let health = match health.split('/').collect::<Vec<&str>>()[0].trim().parse::<i32>() {
        Ok(a) => a - damage,
        Err(_) => GLOBAL - damage,
    };
    if health < 0 {
        (("费尽千辛万苦，你终于打怪了小怪兽！\n**日志保存于当前目录下**\n").to_string(), "".to_string())
    } else {
        let mut s = String::from(ATTACK_TEXT[index]);
        let health = format!("{health} / {GLOBAL}");
        s.push_str(&health);
        (s, health)
    }
}

#[tauri::command]
fn save_log(log: &str) -> bool {
    let path = "log.txt";
    let mut output = match File::create(path) {
        Ok(a) => a,
        Err(_) => return false,
    };
    match write!(output, "{}", String::from(log)) {
        Ok(_) => return true,
        Err(_) => return false,
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![attack, save_log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
