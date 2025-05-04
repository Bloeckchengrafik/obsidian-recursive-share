use notify::{RecommendedWatcher, Watcher};
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;
use notify::event::DataChange::Any;
use notify::event::ModifyKind::Data;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct ConfigValues {
    port: u16,
    auth_tokens: Vec<String>,
    storage_path: String,
}

impl ConfigValues {
    fn load() -> Self {
        let conf = config::Config::builder()
            .add_source(config::File::with_name("settings.toml"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        let config_values: ConfigValues = conf.try_deserialize().unwrap();
        config_values
    }
}

pub struct Config {
    config: Arc<Mutex<ConfigValues>>,
}

impl Config {
    pub async fn new() -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let mutex = Arc::new(Mutex::new(ConfigValues::load()));
        let fut_mutex = mutex.clone();
        tokio::spawn(async move {
            let mut watcher = RecommendedWatcher::new(
                move |res| {
                    tx.blocking_send(res).unwrap();
                },
                notify::Config::default(),
            )
            .unwrap();

            watcher
                .watch(
                    Path::new("settings.toml"),
                    notify::RecursiveMode::NonRecursive,
                )
                .unwrap();

            while let Some(res) = rx.recv().await {
                match res {
                    Ok(event) => {
                        if event.kind == notify::EventKind::Modify(Data(Any)) {
                            let mut config = fut_mutex.lock().await;
                            *config = ConfigValues::load();
                        }
                    }
                    Err(e) => eprintln!("Error: {:?}", e),
                }
            }
        });

        Config { config: mutex }
    }

    pub async fn get_port(&self) -> u16 {
        let config = self.config.lock().await;
        config.port
    }

    pub async fn get_auth_tokens(&self) -> Vec<String> {
        let config = self.config.lock().await;
        config.auth_tokens.clone()
    }

    pub async fn get_storage_path(&self) -> String {
        let config = self.config.lock().await;
        config.storage_path.clone()
    }
}
