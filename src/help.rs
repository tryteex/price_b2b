pub struct Help {}

impl Help {
    pub fn show() {
        let desc = "Brain B2B — це високошвидкісна програма FastCGI для WEB-додатків, яка генерує прайс-листи для клієнтів.";
        let ver = format!("brain_b2b версія: {}", env!("CARGO_PKG_VERSION"));
        let help = "
    Використання: brain_b2b [start|check|stop|help]
    
    Дії:
        start       : запуск додатка
        check       : перевірити irc сокет, і якщо він вільний, запустити додаток,
                    : якщо зайнятий, то не створює запис в error.log
        stop        : зупинка додатка з усіма робочими потоками
        help        : показати цю довідку
    ";
        println!("");
        println!("{}", desc);
        println!("{}", ver);
        println!("{}", help);
    }
}