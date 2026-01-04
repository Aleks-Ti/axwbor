fn main() {
    // Этот код выполняется перед сборкой

    // Показать предупреждение
    println!("cargo:warning=This is a build script warning");

    // Пересобрать при изменении файла
    println!("cargo:rerun-if-changed=src/config.toml");

    // Пересобрать при изменении переменной окружения
    println!("cargo:rerun-if-env-changed=DATABASE_URL");
}
