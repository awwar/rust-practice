fn main() {
    // Перезапускать сборку, если изменилась переменная DEBUG
    println!("cargo:rerun-if-env-changed=DEBUG");

    // Проверяем значение переменной окружения
    if std::env::var("DEBUG").unwrap_or_default() == "1" {
        // Включаем кастомный флаг конфигурации
        println!("cargo:rustc-cfg=debug_enabled");
    }
}