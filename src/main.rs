use std::{process::{Command, Stdio}, str::FromStr};
// use cron::Schedule;
// use chrono::Utc;
use std::fs;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    // Get command line arguments
    let _args: Vec<String> = std::env::args().skip(1).collect();

    // Start CICD
    println!("{} : Starting CICD", get_ts());

    let mut version = String::new();
    let mut runing_instance = false;
    // let schedul = Schedule::from_str(" * * * * *").unwrap();
    // for sc in schedul.upcoming(Utc){

    // // }

    if let Ok(output) = execute( format!("git ls-remote {} main",get_config_file().await.unwrap().repo).to_string()).await {
        version = output.trim().split("refs").next().unwrap().to_string();
    }
    println!("{} : version initialized : {}", get_ts(),version);

    // Schedule cron job
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
    loop {
        interval.tick().await;

        if runing_instance {
            continue; ;
        }

        let mut currversion = String::new();

        if let Ok(output) = execute(format!("git ls-remote {} main",get_config_file().await.unwrap().repo).to_string()).await {
            currversion = output.trim().split("refs").next().unwrap().to_string();
        }

        if currversion == version {
            println!("{} : Checked {} No Updates", get_ts(), get_config_file().await.unwrap().repo);
            continue; 
        } 

        runing_instance =true;
        println!("{} : Repo Version Changed Starting Update", get_ts());
        
        if !build_and_setup().await {
            
            if let Some(clear_folder) = get_config_file().await.unwrap().clear_folder {
                execute(format!("rm -rf {}", clear_folder)).await.ok();
            }
            runing_instance=false;

            println!("{} : Error Ocured While Updating We Will Retry :)", get_ts());
            continue; 
        }
        
        if let Some(clear_folder) = get_config_file().await.unwrap().clear_folder {
            execute(format!("rm -rf {}", clear_folder)).await.ok();
        }
        runing_instance=false;
        version = currversion;
        println!("{} : Updates Finished", get_ts());

        continue; 
        
    }
}

async fn build_and_setup() -> bool {
    if let Ok(config) = get_config_file().await {
        let cloning = execute(format!("git clone {}", config.repo)).await.is_ok();
        if !cloning {
            println!("{} : Error Cloning {}", get_ts(), config.repo);
            return false;
        }
        println!("{} : cloned {}", get_ts(), config.repo);

        
        if let Some(build) = config.build {
            let mut success = true;
            for folder in build {
                if let Ok(_) = execute(folder.clone()).await {
                    println!("{} : Executed {}", get_ts(), folder);
                    continue;
                } 
                
                println!("{} : Error Executing {}", get_ts(), folder);
                success = false;
                break;
                
            }
            if !success {
                return false;
            }
        }

        if let Some(mouve) = config.mouve {
            let mut success = true;
            for folder in mouve {
                
                if let Ok(_) = execute(folder.clone()).await {
                    println!("{} : Executed {}", get_ts(), folder);
                    continue;
                } 
                
                println!("{} : Error Executing {}", get_ts(), folder);
                success = false;
                break;
                
            }
            if !success {
                return false;
            }
        }
    }
    return true
}

async fn execute(commande: String) -> Result<String, String> {

    match Command::new("sh").arg("-c").arg(commande).output() {
        Ok(output) => {
            if !output.status.success() {
                if let Some(clear_folder) = get_config_file().await.unwrap().clear_folder {
                    Command::new("sh").arg("-c").arg(format!("rm -rf {:?}",clear_folder));
                }
                
                return Err(String::from_utf8_lossy(&output.stderr).to_string())
                
            } 
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
            
        }
        Err(err) => {
            if let Some(clear_folder) = get_config_file().await.unwrap().clear_folder {
                Command::new("sh").arg("-c").arg(format!("rm -rf {:?}",clear_folder));
            }
            Err(err.to_string())
        }
    }
}

async fn get_config_file() -> Result<Config, Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut config = Config::default();
    for i in 0..args.len() {
        if args[i] == "--config" && i + 1 < args.len() {
            if let Ok(file_txt) = fs::read_to_string(&args[i + 1]) {
                if let Ok(file) = serde_json::from_str::<Config>(&file_txt) {
                    
                    config = file;
                }
            }
        }
    }

    Ok(config)
}

fn get_ts() -> String {
    chrono::Utc::now().to_rfc3339()
}

#[derive(Debug, Deserialize, Default)]
struct Config {
    repo: String,
    clear_folder: Option<String>,
    build: Option<Vec<String>>,
    mouve: Option<Vec<String>>,
}

// #[derive(Debug, Deserialize)]
// struct Folder {
//     work_dir: Option<String>,
//     cmd: Vec<String>,
// }
