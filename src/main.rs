#![allow(non_snake_case)]
use std::process::Command;

fn makeString(stdout: &Vec<u8>) -> String {
    // executing a command returns an array of u8 instead of chars, this simply turns them into
    // chars and then returns all of them as a string
    let string =  String::from_utf8_lossy(stdout).to_string();
    return string
}

fn splitUntil(string: &mut String, indicator: String) -> String {
    let string = string.replace(&indicator, "");
    return string;
}

fn getName(string: &mut String) -> String {
    let pathComponents: Vec<&str> = string.split("/").collect();

    let name = pathComponents[pathComponents.len() -1].to_string();
    return name;
}

fn makeSeconds(minutes: &String) -> i32 {
    let splitted: Vec<&str> = minutes.rsplit(":").collect();

    let minutes = splitted[splitted.len() -1].to_string().parse::<i32>().unwrap();
    let seconds = splitted[0].to_string().parse::<i32>().unwrap();

    return minutes * 60 + seconds;
}

fn makeStatusBar(totalTime: i32, currentTime: i32) -> String {
    // [ ――ᐅ――――――― ] ─
    // I honestly have no idea of how to do this in a better way, this is extremely hacky
    let percentage = (currentTime * 10) / totalTime;
    let progressBar: String = String::from("[");
    let progressBar = match percentage {
        0 => progressBar + &String::from(" ▶───────── ]"),
        1 => progressBar + &String::from(" ─▶──────── ]"),
        2 => progressBar + &String::from(" ──▶─────── ]"),
        3 => progressBar + &String::from(" ───▶────── ]"),
        4 => progressBar + &String::from(" ────▶───── ]"),
        5 => progressBar + &String::from(" ─────▶──── ]"),
        6 => progressBar + &String::from(" ──────▶─── ]"),
        7 => progressBar + &String::from(" ───────▶── ]"),
        8 => progressBar + &String::from(" ────────▶ ]"),
        9 => progressBar + &String::from(" ────────▶ ]"),
        10 => progressBar + &String::from(" ───────▶ ]"),
        _ => return String::from("[     :(     ]")
    };
    return progressBar;
}


fn main() {
    // execute command, will return an array of u8 (ascii codes)
    let progressBarOn = true;
    let commandOut = Command::new("mocp").arg("-i").output().expect("Failed to execute command");
    let mut output = makeString(&commandOut.stdout);
    // this is for ease of use
    output = output.replace("\n", "||");
    let mut lines: Vec<&str> = output.split("||").collect();

    //TODO: this sometimes randomly gets triggered in the middle of songs, don't know how" ─
    if output == "" {
        println!("    ♫    Server not running ");
    } else if lines.len() == 2 {
        println!("    ♫    No song is playing");
    } else if lines.len() > 6 {
        // STATUS ==============================================================================
        let mut status = splitUntil(&mut lines[0].to_string(), String::from("State: "));
        if status == "PLAY" {
            status = '▶'.to_string();
        } else if status == "PAUSE" {
            status = '⏸'.to_string();
        }

        // FILE NAME ===========================================================================
        let mut filePath = splitUntil(&mut lines[1].to_string(), String::from("File: "));
        let name = getName(&mut filePath);

        // SONG TIME ===========================================================================

        // Total Time ============
        let totalTime = splitUntil(&mut lines[6].to_string(), String::from("TotalTime: "));

        // Current Time ==========
        let currentTime = splitUntil(&mut lines[9].to_string(), String::from("CurrentTime: "));

        // Time ==================
        let time = String::from("[") + &currentTime + &String::from(" / ") + &totalTime + &']'.to_string();

        // FINAL MESSAGE ======================================================================
        if progressBarOn == true {
        println!("    ♫      {}  {}  {}  {}", name, status, time, makeStatusBar(makeSeconds(&totalTime), makeSeconds(&currentTime)));
        } else {
        println!("    ♫      {}  {}  {} ", name, status, time);
        }

    }

}
