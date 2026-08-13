// use crate::task;
// use std::io::Read;

// pub fn deserialize(path : &str) -> Vec<task::Task>{
//     let mut tasks : Vec<task::Task> = vec![];
//     let mut tasks_str = String::new();
//     let mut file = match std::fs::File::open(path){
//         Ok(fl) => fl,
//         Err(_) => return tasks,
//     };

//     file.read_to_string(&mut tasks_str).expect("Cannot read tasks.md");

//     let tasks_str : Vec<&str> = tasks_str.lines().collect();
//     let mut task = task::Task{
//         status : false,
//         time : "0".to_string(),
//         text : String::new(),
//         importance : 0,
//     };

//     for i in tasks_str{
//         if i.starts_with('#'){
//             println!("task");
//             task.status = i.contains('+');
//             task.time = i[i.find('/').expect("Syntax error : '/' must be before time")+2..i.len()].to_string();
//             continue;
//         }
//         let ch = i.find("TASK :").unwrap_or(256);
//         if ch < 256{
//             task.text = i[ch+7..i.len()]
//                 .to_string();
//             continue;
//         }
//         let ch = i.find("IMPORTANCE :").unwrap_or(256);
//         if ch < 256{
//             task.importance = i[ch+13..i.len()]
//                 .to_string()
//                 .parse()
//                 .unwrap_or(0);
//         }

//        if task.importance > 0{
//             tasks.push(task);
//             task = task::Task{
//                 status : false,
//                 time : "0".to_string(),
//                 text : String::new(),
//                 importance : 0,
//             };
//         }
//     }
//     tasks
// }
