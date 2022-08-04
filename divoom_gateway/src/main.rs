mod server;

use crate::server::*;
use clap::Parser;
use divoom::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliOptions {
    #[clap(help = "Device address.", value_parser)]
    device_address: String,

    #[clap(short = 's', long = "server", help = "Server address.", value_parser)]
    server_address: Option<String>,

    #[clap(short = 'p', long = "port", help = "Server port.", value_parser)]
    server_port: Option<u16>,

    #[clap(
        short = 'c',
        long = "config",
        help = "Gateway config file.",
        value_parser
    )]
    config_file_path: Option<String>,

    #[clap(
        short = 't',
        long = "animation-template-dir",
        help = "Animation template file paths.",
        value_parser
    )]
    animation_template_dir: Option<String>,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DivoomGatewayConfig {
    #[serde(default)]
    pub device_address: String,

    #[serde(default)]
    pub server_address: String,

    #[serde(default)]
    pub server_port: u16,

    #[serde(default)]
    pub schedules: Vec<DivoomScheduleConfigCronJob>,

    #[serde(default)]
    pub animation_template_dir: String,
}

impl DivoomGatewayConfig {
    pub fn fill_default(&mut self) {
        if self.server_address.is_empty() {
            self.server_address = "127.0.0.1".to_string();
        }

        if self.server_port == 0 {
            self.server_port = 20821;
        }

        if self.animation_template_dir.is_empty() {
            self.animation_template_dir = "./animation-templates".to_string();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let config = load_gateway_config()?;

    let schedule_count = config.schedules.len();
    let mut schedule_manager: DivoomScheduleManager;
    if schedule_count != 0 {
        let animation_template_manager: Arc<DivoomAnimationTemplateManager>;
        if Path::new(&config.animation_template_dir).is_dir() {
            animation_template_manager = Arc::new(DivoomAnimationTemplateManager::from_dir(&config.animation_template_dir)
                .map_err(|e| std::io::Error::new(ErrorKind::NotFound, format!("Failed to load templates: Error = {:?}", e)))?);
        } else {
            animation_template_manager = Arc::new(DivoomAnimationTemplateManager::new(&config.animation_template_dir).unwrap());
        }

        schedule_manager =
            DivoomScheduleManager::from_config(config.device_address.clone(), config.schedules, animation_template_manager)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

        println!(
            "Found {} schedules in gateway config, starting divoom scheduler device {}.",
            schedule_count, config.device_address
        );

        schedule_manager.start();
    }

    let url = format!("http://{}:{}", config.server_address, config.server_port);
    println!(
        "Starting divoom gateway on: {} for device {}.",
        url, config.device_address
    );
    println!(
        "Please open your browser with URL: {} and happy divooming!",
        url
    );

    let api_server = ApiServer::new(
        config.server_address,
        config.server_port,
        config.device_address,
    );
    api_server.start().await
}

fn load_gateway_config() -> std::io::Result<DivoomGatewayConfig> {
    let args = CliOptions::parse();

    let mut config = load_gateway_config_from_file(&args.config_file_path)?;

    config.device_address = args.device_address;

    if let Some(server_address) = args.server_address {
        config.server_address = server_address;
    }

    if let Some(server_port) = args.server_port {
        config.server_port = server_port;
    }

    if let Some(animation_template_dir) = args.animation_template_dir {
        config.animation_template_dir = animation_template_dir;
    }

    config.fill_default();

    Ok(config)
}

fn load_gateway_config_from_file(
    config_file_path: &Option<String>,
) -> std::io::Result<DivoomGatewayConfig> {
    let config = match config_file_path {
        None => DivoomGatewayConfig {
            device_address: "".to_string(),
            server_address: "".to_string(),
            server_port: 0,
            schedules: vec![],
            animation_template_dir: "".to_string()
        },

        Some(path) => {
            let config_file = File::open(path)?;
            let mut config: DivoomGatewayConfig = serde_yaml::from_reader(config_file)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            config
        }
    };

    Ok(config)
}
