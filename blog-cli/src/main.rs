use blog_client::{BlogClient, BlogClientError, Transport};
use clap::{Parser, Subcommand};

const TOKEN_FILE_NAME: &str = ".blog_cli_token";

/// Возвращает путь к файлу токена: ~/.blog_cli_token
fn get_token_path() -> Result<std::path::PathBuf, BlogClientError> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Не удалось определить домашнюю директорию",
            )
        })
        .map_err(BlogClientError::Io)?;
    Ok(home_dir.join(TOKEN_FILE_NAME))
}

/// Сохраняет токен в файл
async fn save_token(token: &str) -> Result<(), BlogClientError> {
    let path = get_token_path()?;
    tokio::fs::write(&path, token).await?;
    println!("Токен сохранён в {}", path.display());
    Ok(())
}

/// Загружает токен из файла
async fn load_token() -> Result<String, BlogClientError> {
    let path = get_token_path()?;
    let contents = tokio::fs::read_to_string(&path).await?;
    let token = contents.trim().to_string();
    if token.is_empty() {
        Err(BlogClientError::TokenMissing)
    } else {
        Ok(token)
    }
}

#[derive(Parser)]
#[command(name = "blog", about = "Блог CLI-утилита", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Авторизация пользователя
    Login {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    /// Регистрация пользователя
    Register {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        password: String,
    },
    /// Создание поста
    Create {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: String,
    },
    /// Демонстрация поста
    Get {
        #[arg(short, long)]
        id: i64,
    },
    /// Обновление поста
    Update {
        #[arg(short, long)]
        id: i64,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        content: Option<String>,
    },
    /// Удаление поста
    Delete {
        #[arg(short, long)]
        id: i64,
    },
    /// Демонстрация списка постов
    List {
        #[arg(short, long)]
        limit: Option<i32>,
        #[arg(short, long)]
        offset: Option<i32>,
    },
}

#[tokio::main]
async fn main() -> Result<(), BlogClientError> {
    let cli = Cli::parse();
    let mut client = BlogClient::new(Transport::http_default()).await?;
    match cli.command {
        Commands::Login { username, password } => {
            let token = client
                .login(username, password)
                .await
                .expect("Авторизация не удалась");
            save_token(&token).await?;
        }
        Commands::Register {
            username,
            email,
            password,
        } => {
            client
                .register(username, email, password)
                .await
                .expect("Регистрация не удалась");
        }
        Commands::List { limit, offset } => {
            client
                .list_posts(limit.unwrap_or(10i32), offset.unwrap_or(0i32))
                .await
                .expect("Получение списка постов не удалось");
        }
        Commands::Get { id } => {
            client
                .get_post(id)
                .await
                .expect("Получение поста не удалось");
        }
        _ => {
            let token = load_token().await?;
            client.set_token(token);
            match cli.command {
                Commands::Create { title, content } => {
                    client
                        .create_post(title, content)
                        .await
                        .expect("Создание поста не удалось");
                }
                Commands::Update { id, title, content } => {
                    client
                        .update_post(id, title, content)
                        .await
                        .expect("Обновление поста не удалось");
                }
                Commands::Delete { id } => {
                    client
                        .delete_post(id)
                        .await
                        .expect("Удаление поста не удалось");
                }
                _ => {}
            }
        }
    }
    Ok(())
}
