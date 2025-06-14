extern crate chrono;
extern crate serde_json;
use chrono::prelude::*;
use regex::Regex;
use serde_json::json;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

fn file_exist(pth: &String) -> bool {
    let p = Path::new(pth);
    p.is_file() && p.extension() != None && p.extension().unwrap().to_ascii_lowercase() == "json"
}

fn convert_date_to_iso(data: String) -> (String, i32) {
    let mut content: &str = &data.clone().to_owned();
    let mut count = 0;

    let re = Regex::new(r#"(?m)\{"\$date":\{"\$numberLong":"(-?[0-9]+)"\}\}"#).unwrap();

    for cap in re.captures_iter(&content) {
        // let ts = extract_timestamp(&cap[0]);
        let ts: i64 = cap[1].to_string().parse().unwrap();
        let datetime = DateTime::<Utc>::from_timestamp(ts / 1000, 0).expect("Invalid timestamp");
        let newdate = datetime.format("%Y-%m-%dT%H:%M:%SZ");
        let mut tformat: String = "ISODate(".to_string();
        tformat.push_str(&newdate.to_string());
        tformat.push_str(&")".to_string());
        // println!("uuu {:?}", ts);
        let k: String = re.replace(content, &tformat).to_string();
        content = Box::leak(k.into_boxed_str());
        count += 1;
    }
    // let mut file = File::create("hello.txt").expect("Error encountered while creating file!");

    (content.to_string(), count)
}
//{
//   "$oid": "62cfc562cff47e000170bb87"
// }
fn oid_to_id(data: String) -> (String, i32) {
    let mut content: &str = &data.clone().to_owned();
    let mut count = 0;
    let re = Regex::new(r#"\{"\$oid":"([a-zA-Z0-9]+)"\}"#).unwrap();
    for cap in re.captures_iter(&content) {
        let val = cap[1].to_string();
        let k: String = re.replace(content, json!(&val).to_string()).to_string();
        content = Box::leak(k.into_boxed_str());
        count += 1;
    }
    (content.to_string(), count)
}

fn numlong_to_int(data: String) -> (String, i32) {
    let mut content: &str = &data.clone().to_owned();
    let mut count = 0;
    let re = Regex::new(r#"\{"\$numberLong":"(-?[0-9]+)"\}"#).unwrap();
    for cap in re.captures_iter(&content) {
        let val = cap[1].to_string();
        let k: String = re.replace(content, &val).to_string();
        content = Box::leak(k.into_boxed_str());
        count += 1;
    }
    (content.to_string(), count)
}
fn numint_to_int(data: String) -> (String, i32) {
    let mut content: &str = &data.clone().to_owned();
    let mut count = 0;
    let re = Regex::new(r#"\{"\$numberInt":"(-?[0-9]+)"\}"#).unwrap();
    for cap in re.captures_iter(&content) {
        let val = cap[1].to_string();
        let k: String = re.replace(content, &val).to_string();
        content = Box::leak(k.into_boxed_str());
        count += 1;
    }
    (content.to_string(), count)
}
fn numdouble_to_double(data: String) -> (String, i32) {
    let mut content: &str = &data.clone().to_owned();
    let mut count = 0;
    let re = Regex::new(r#"\{"\$numberDouble":"(-?\d+\.?\d*)"\}"#).unwrap();
    for cap in re.captures_iter(&content) {
        let val = cap[1].to_string();
        let k: String = re.replace(content, &val).to_string();
        content = Box::leak(k.into_boxed_str());
        count += 1;
    }
    (content.to_string(), count)
}

// fn modify(data: String) -> (&'static str, i32) {
//     let mut content: &str = &data.clone().to_owned();
//     let mut count = 0;

//     (content, count)
// }

fn read_file(src: &String) -> std::io::Result<String> {
    let path = Path::new(src);
    let file = File::open(path).expect("file not exist");
    let buf_reader = BufReader::new(file);
    let mut cnt = "".to_string();
    let mut lcount = 1;
    let mut is_prettify: bool = true;
    for line in buf_reader.lines() {
        let mut l = line.unwrap().trim().to_string();
        if lcount == 1 && l.len() != 1 {
            is_prettify = false;
        }
        if is_prettify {
            l = l.replacen(": ", ":", l.len());
        }
        cnt.push_str(&l);

        lcount += 1;
    }
    Ok(cnt)
}

fn main() {
    let cmd_args: Vec<String> = std::env::args().collect();
    if cmd_args.len() < 2 {
        println!("Args expected!!");
        return;
    }

    let mut op = String::from("output.json");
    let src = &cmd_args[1].replace("src=", "");
    if cmd_args.len() == 3 {
        let temp = &cmd_args[2].replace("o=", "");
        op = String::from(temp);
    }
    if !file_exist(src) {
        println!("correct path expected!!");
        return;
    }

    let before = Instant::now();

    let orig = match read_file(&src) {
        Err(why) => panic!("couldn't read {}: {}", src, why),
        Ok(k) => k,
    };

    let d = convert_date_to_iso(orig);
    let j = numlong_to_int(d.0);
    let k = numint_to_int(j.0);
    let l = numdouble_to_double(k.0);
    let t = oid_to_id(l.0);

    let c = d.1 + j.1 + k.1 + l.1 + t.1;
    println!("Elapsed time: {:?}", before.elapsed());

    let mut file = File::create(op).expect("Error encountered while creating file!");
    file.write_all(&t.0.as_bytes())
        .expect("Error while writing to file");

    println!("done with changes :{:?} {}", c, t.0.len());
}
