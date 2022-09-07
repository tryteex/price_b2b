use std::{process, fs::OpenOptions, io::Write};

use chrono::Local;

#[derive(Debug)]
pub struct Log {
    pid: u32,
    file: String,
}

impl Log {
    pub fn new(dir: &str) -> Log {
        Log {
            pid: process::id(),
            file: format!("{}/error.log", dir),
        }
    }

    pub fn write(&self, err: u32, text: &str) {
        
        let time = Local::now().format("%Y.%m.%d %H:%M:%S%.9f %:z").to_string();
        let str;
        if text.len() > 0 {
            str = format!("{} PID={} Помилка {}: {}. Опис: {}\n", time, self.pid, err, self.get_error(err), text);
        } else {
            str = format!("{} PID={} Помилка {}: {}.\n", time, self.pid, err, self.get_error(err));
        };
        eprint!("{}", &str);
        if let Ok(mut file) = OpenOptions::new().create(true).write(true).append(true).open(&self.file) {
            file.write_all(str.as_bytes()).unwrap();
        };
    }

    pub fn exit(&self, err: u32, text: &str) -> ! {
        self.write(err, text);
        process::exit(1);
    }

    fn get_error(&self, err: u32) -> String {
        match err {
            100 => "Відсутній файл конфігурації".to_owned(),
            101 => "Файл конфігурації має невірний формат".to_owned(),
            102 => "В файлі конфігурації відсутній параметр 'port' (Порт TCP запуску сервера генерації прайсів B2B)".to_owned(),
            103 => "В файлі конфігурації параметр 'port' має невірний формат (Число від 1 до 65536)".to_owned(),
            104 => "В файлі конфігурації відсутній параметр 'time_zone'".to_owned(),
            105 => "В файлі конфігурації параметр 'time_zone' має невірний формат (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)".to_owned(),
            106 => "В файлі конфігурації відсутній параметр 'max' (Кількість потоків на обробку прайсів)".to_owned(),
            107 => "В файлі конфігурації параметр 'max' має невірний формат (Число від 1 до 255)".to_owned(),
            108 => "В файлі конфігурації відсутній параметр 'db_log_host' (Хост бази даних MySql логістики товарів)".to_owned(),
            109 => "В файлі конфігурації параметр 'db_log_host' має невірний формат".to_owned(),
            110 => "В файлі конфігурації відсутній параметр 'db_log_port' (Порт бази даних MySql логістики товарів)".to_owned(),
            111 => "В файлі конфігурації параметр 'db_log_port' має невірний формат (Число від 1 до 65536)".to_owned(),
            112 => "В файлі конфігурації відсутній параметр 'db_log_user' (Користувач бази даних MySql логістики товарів)".to_owned(),
            113 => "В файлі конфігурації параметр 'db_log_user' має невірний формат".to_owned(),
            114 => "В файлі конфігурації відсутній параметр 'db_log_pwd' (Пароль користувача бази даних MySql логістики товарів)".to_owned(),
            115 => "В файлі конфігурації параметр 'db_log_pwd' має невірний формат".to_owned(),
            116 => "В файлі конфігурації відсутній параметр 'db_log_name' (Назва бази даних MySql логістики товарів)".to_owned(),
            117 => "В файлі конфігурації параметр 'db_log_name' має невірний формат".to_owned(),
            118 => "В файлі конфігурації відсутній параметр 'db_b2b_host' (Хост бази даних MySql B2B портала)".to_owned(),
            119 => "В файлі конфігурації параметр 'db_b2b_host' має невірний формат".to_owned(),
            120 => "В файлі конфігурації відсутній параметр 'db_b2b_port' (Порт бази даних MySql B2B портала)".to_owned(),
            121 => "В файлі конфігурації параметр 'db_b2b_port' має невірний формат (Число від 1 до 65536)".to_owned(),
            122 => "В файлі конфігурації відсутній параметр 'db_b2b_user' (Користувач бази даних MySql B2B портала)".to_owned(),
            123 => "В файлі конфігурації параметр 'db_b2b_user' має невірний формат".to_owned(),
            124 => "В файлі конфігурації відсутній параметр 'db_b2b_pwd' (Пароль користувача бази даних MySql B2B портала)".to_owned(),
            125 => "В файлі конфігурації параметр 'db_b2b_pwd' має невірний формат".to_owned(),
            126 => "В файлі конфігурації відсутній параметр 'db_b2b_name' (Назва бази даних MySql B2B портала)".to_owned(),
            127 => "В файлі конфігурації параметр 'db_b2b_name' має невірний формат".to_owned(),
            128 => "В файлі конфігурації відсутній параметр 'db_local_host' (Хост бази даних MySql логіювання запитів)".to_owned(),
            129 => "В файлі конфігурації параметр 'db_local_host' має невірний формат".to_owned(),
            130 => "В файлі конфігурації відсутній параметр 'db_local_port' (Порт бази даних MySql логіювання запитів)".to_owned(),
            131 => "В файлі конфігурації параметр 'db_local_port' має невірний формат (Число від 1 до 65536)".to_owned(),
            132 => "В файлі конфігурації відсутній параметр 'db_local_user' (Користувач бази даних MySql логіювання запитів)".to_owned(),
            133 => "В файлі конфігурації параметр 'db_local_user' має невірний формат".to_owned(),
            134 => "В файлі конфігурації відсутній параметр 'db_local_pwd' (Пароль користувача бази даних MySql логіювання запитів)".to_owned(),
            135 => "В файлі конфігурації параметр 'db_local_pwd' має невірний формат".to_owned(),
            136 => "В файлі конфігурації відсутній параметр 'db_local_name' (Назва бази даних MySql логіювання запитів)".to_owned(),
            137 => "В файлі конфігурації параметр 'db_local_name' має невірний формат".to_owned(),
            138 => "В файлі конфігурації відсутній параметр 'irc' (Порт TCP управління сервера генерації прайсів B2B)".to_owned(),
            139 => "В файлі конфігурації параметр 'irc' має невірний формат (Число від 1 до 65536)".to_owned(),
            140 => "В файлі конфігурації відсутній параметр 'salt'".to_owned(),
            141 => "В файлі конфігурації параметр 'salt' має невірний формат".to_owned(),
            
            200 => "Помилка запуска сервера".to_owned(),
            201 => "Помилка при з’єднання до IRC сервера".to_owned(),
            202 => "Не вдалося приднатися до IRC сервера".to_owned(),
            203 => "Помилка відправлення сигналу до IRC сервера".to_owned(),
            204 => "Не вдалося встановити timeout читання даних від IRC сервера".to_owned(),
            205 => "Не вдалося прочитати дані від IRC сервера".to_owned(),
            206 => "Дані відсутні при читанні від IRC сервера".to_owned(),
            207 => "Невірні дані отримані при читанні від IRC сервера".to_owned(),
            208 => "Невірні дані отримані при читанні від IRC сервера".to_owned(),
      
            300 => "Відсутні права для запуска IRC сервера".to_owned(),
            301 => "Сокет IRC занятий".to_owned(),
            302 => "Сокет IRC недоступний для користування".to_owned(),
            303 => "Неможливо відкрити сокет IRC".to_owned(),
            304 => "Неможливо встановити неблокуючий режим для сокет IRC".to_owned(),

            400 => "Відсутні права для запуска TCP сервера".to_owned(),
            401 => "Сокет TCP занятий".to_owned(),
            402 => "Сокет TCP недоступний для користування".to_owned(),
            403 => "Неможливо відкрити сокет TCP".to_owned(),
            404 => "Неможливо встановити неблокуючий режим для сокет TCP".to_owned(),

            500 => "Проблеми з чергою потоків".to_owned(),
            501 => "Переплутані з'єднання".to_owned(),
            502 => "Неможливо використовувати каталог для кеша прайсів".to_owned(),
            503 => "Неможливо створити каталог для кеша прайсів".to_owned(),

            600 => "Некорректна строка підключення до бази даних".to_owned(),
            601 => "Помилка з'єднання з базою даних".to_owned(),
            602 => "Помилка виконання запиту до бази даних".to_owned(),
            603 => "Не вдалося встановити початкові параметри підключення".to_owned(),

            _ => "Невідома помилка".to_owned(),
        }
    }

    pub fn client_err(&self, err: u32) -> String {
        let err = match err {
            1 => "Помилка 1: Відсутній параметр format".to_owned(),
            2 => "Помилка 2: Формат прайс-листа не підтримується".to_owned(),
            3 => "Помилка 3: Відсутній параметр full".to_owned(),
            4 => "Помилка 4: Тип прайсу не підтримується".to_owned(),
            5 => "Помилка 5: Відсутній параметр companyID".to_owned(),
            6 => "Помилка 6: Невірний формат companyID".to_owned(),
            7 => "Помилка 7: Відсутній параметр targetID".to_owned(),
            8 => "Помилка 8: Невірний формат targetID".to_owned(),
            9 => "Помилка 9: Відсутній параметр lang".to_owned(),
            10 => "Помилка 10: Невірний формат lang".to_owned(),
            11 => "Помилка 11: Відсутній параметр time".to_owned(),
            12 => "Помилка 12: Невірний формат time".to_owned(),
            13 => "Помилка 13: Відсутній параметр userID".to_owned(),
            14 => "Помилка 14: Невірний формат userID".to_owned(),
            15 => "Помилка 15: Відсутній параметр token".to_owned(),
            16 => "Помилка 16: Невірний token".to_owned(),
            17 => "Помилка 17: Доступ заборонено (companyID не знайдено)".to_owned(),
            18 => "Помилка 18: Доступ заборонено (userID не знайдено)".to_owned(),
            19 => "Помилка 19: Доступ заборонено (profilesID не встановлено)".to_owned(),
            20 => "Помилка 20: Неможливо перевірити директорію cache для прайсів".to_owned(),
            21 => "Помилка 21: Неможливо видалити застарілі cache-файли для прайсів".to_owned(),
            22 => "Помилка 22: Неможливо видалити cache-файли для прайсів".to_owned(),
            23 => "Помилка 23: targetID повинен бути > 0".to_owned(),
            24 => "Помилка 24: Сервер прайсів зупинено".to_owned(),
            25 => "Помилка 25: Невідомий targetID".to_owned(),
            26 => "Помилка 26: До targetID не прив'язан склад".to_owned(),
            27 => "Помилка 27: Неможливо отримати ціни".to_owned(),
            28 => "Помилка 28: Неможливо прочитати ціни".to_owned(),
            
            _ => "Невідома помилка".to_owned(),
        };
        format!("<!DOCTYPE HTML><html><head><title>PriceList</title><meta charset=\"utf-8\"/></head><body>{}</body></html>", err)
    }
}