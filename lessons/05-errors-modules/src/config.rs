// config 模块是本课的“工程边界”：它负责把外部字符串解析成强类型配置。
// 对外只暴露 AppConfig、ConfigError、load_from_str，内部解析细节保持私有。

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct AppConfig {
    pub app_name: String,
    pub port: u16,
    pub enable_cache: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    MissingField(&'static str),
    InvalidLine { line: usize, content: String },
    InvalidPort(String),
    InvalidBool(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField(field) => write!(formatter, "缺少字段 `{field}`"),
            Self::InvalidLine { line, content } => {
                write!(formatter, "第 {line} 行不是 key = value 格式：{content}")
            }
            Self::InvalidPort(value) => write!(formatter, "端口必须是 0-65535 的数字：{value}"),
            Self::InvalidBool(value) => {
                write!(formatter, "布尔值必须是 true/false/yes/no/1/0：{value}")
            }
        }
    }
}

impl Error for ConfigError {}

pub fn load_from_str(input: &str) -> Result<AppConfig, ConfigError> {
    let mut app_name = None;
    let mut port = None;
    let mut enable_cache = None;

    for (index, raw_line) in input.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| ConfigError::InvalidLine {
                line: line_number,
                content: line.to_string(),
            })?;

        let key = key.trim();
        let value = value.trim();

        match key {
            "app_name" => app_name = Some(value.to_string()),
            "port" => port = Some(parse_port(value)?),
            "enable_cache" => enable_cache = Some(parse_bool(value)?),
            _ => {
                return Err(ConfigError::InvalidLine {
                    line: line_number,
                    content: line.to_string(),
                });
            }
        }
    }

    Ok(AppConfig {
        app_name: app_name.ok_or(ConfigError::MissingField("app_name"))?,
        port: port.ok_or(ConfigError::MissingField("port"))?,
        enable_cache: enable_cache.ok_or(ConfigError::MissingField("enable_cache"))?,
    })
}

fn parse_port(value: &str) -> Result<u16, ConfigError> {
    value
        .parse::<u16>()
        .map_err(|_| ConfigError::InvalidPort(value.to_string()))
}

fn parse_bool(value: &str) -> Result<bool, ConfigError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "yes" | "1" => Ok(true),
        "false" | "no" | "0" => Ok(false),
        _ => Err(ConfigError::InvalidBool(value.to_string())),
    }
}
