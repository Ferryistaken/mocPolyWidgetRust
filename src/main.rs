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


fn main() {
    // execute command, will return an array of u8 (ascii codes)
    let commandOut = Command::new("mocp").arg("-i").output().expect("Failed to execute command");
    let mut output = makeString(&commandOut.stdout);
    // this is for ease of use
    output = output.replace("\n", "||");
    let mut lines: Vec<&str> = output.split("||").collect();

    //if output.contains("The server is not running!") {
    //    println!("No song currently playing");
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
        println!("    ♫      {}  {}  {}", name, status, time);
    }

}
