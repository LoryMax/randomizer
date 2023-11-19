use rand::RngCore;
use rand::rngs::OsRng;
use std::collections::HashSet;
use std::io;
use clap::Parser;

#[derive(Debug, clap::Parser)]
#[clap(name = "Randomizer", version = "1.0", author = "LoryMax")]
struct Cli {
    /// Нижняя граница диапазона
    #[clap(short, long, default_value = "1", name = "min")]
    lower_bound: usize,

    /// Верхняя граница диапазона
    #[clap(short, long, default_value = "100", name = "max")]
    upper_bound: usize,

    /// Количество случайных чисел для генерации
    #[clap(short, long, default_value = "1", name = "num")]
    num_of_numbers: usize,

    /// Список чисел для игнорирования, разделенных пробелами
    #[clap(short, long, default_value = "", name = "ignore")]
    ignore_list: String,

    /// Turn debugging information on
    #[clap(short, long, default_value = "0")]
    debug: u8,
}

fn main() {    
    loop {
        if let Some((lower_bound, upper_bound, num_of_numbers, ignore_list)) = input_data() {
            process_numbers_generation(lower_bound, upper_bound, num_of_numbers, &ignore_list);

            if !return_to_begin() {
                println!("\nДо свидания!\n");
                break;
            }
        }
    }
}

///Передача вводных данных
fn input_data() -> Option<(usize, usize, usize, HashSet<usize>)> {
        let args = Cli::parse();
        loop {
            let num_of_numbers: usize;
            let mut lower_bound: usize;
            let mut upper_bound: usize;

            match args.debug {
                0 => { 
                    num_of_numbers = read_usize_input("\nВведите количество чисел для генерации:");
                },
                1 => {
                    num_of_numbers = args.num_of_numbers;
                },
                _ => {
                    println!("Debug mode is unknown. Use only 0 or 1.");
                    std::process::exit(0);
                },
            }   

            // Ввод нижней и верхней границы диапазона, пока не будут введены корректные значения
            loop {
                match args.debug {
                    0 => {
                        lower_bound= read_usize_input("Введите нижнюю границу диапазона:");
                        upper_bound = read_usize_input("Введите верхнюю границу диапазона:");
                    },
                    1 => {                    
                        lower_bound = args.lower_bound;
                        upper_bound = args.upper_bound;
                    },
                    _ => {
                        println!("Debug mode is unknown. Use only 0 or 1.");
                        std::process::exit(0);
                    },
                }

                if upper_bound>lower_bound {
                    // Проверка на возможность сгенерировать достаточное количество уникальных чисел            
                    let available_numbers = (lower_bound..=upper_bound).collect::<HashSet<_>>();
                    if available_numbers.len()<num_of_numbers {
                        println!("\nОшибка. Недостаточно уникальных чисел для генерации.");
                        println!("Пожалуйста, уменьшите количество чисел для генерации или уменьшите список чисел, которые нужно игнорировать.\n");
                        continue;
                    }
                } else {
                    println!("\nОшибка. Верхняя граница должна быть больше нижней. Повторите ввод обоих границ.\n");
                }

            let ignore_list = read_ignore_list("\nВведите список чисел, которые нужно игнорировать, через пробел:");
            // Проверка на возможность сгенерировать достаточное количество уникальных чисел, учитывая список исключения      
            let available_numbers = (lower_bound..=upper_bound)
                .filter(|num| !ignore_list.contains(num))
                .collect::<HashSet<_>>();
            if available_numbers.len()<num_of_numbers {
                println!("\nОшибка. Недостаточно уникальных чисел для генерации.");
                println!("Пожалуйста, уменьшите количество чисел для генерации или уменьшите список чисел, которые нужно игнорировать.\n");
                continue;
                }
                return Some((lower_bound, upper_bound, num_of_numbers, ignore_list));
            }
        };        
}

/// Функция генерации чисел
fn process_numbers_generation(lower_bound: usize, upper_bound: usize, num_of_numbers: usize, ignore_list: &HashSet<usize>) {
         let unnormal_fraction_range:f64 = 70.0;
         let selected_fraction: f64 = (num_of_numbers as f64 / (upper_bound - lower_bound + 1) as f64).round();

         if selected_fraction >= unnormal_fraction_range {
            println!("\n>>> Предупреждение! <<<");
            println!("Слишком маленький диапазон чисел. Доля выбранных чисел {} %.", selected_fraction);
            println!("Попробуйте увеличить верхнюю границу диапазона или уменьшить количество чисел для генерации.\n");
            }

            let random_numbers = generate_unique_numbers(lower_bound, upper_bound, num_of_numbers, &ignore_list);
            
            if num_of_numbers == 1 {
                println!("\nСгенерированное число: {}", random_numbers[0]);

                } else if your_choice("\nТребуется ли упорядочить числа? (y/n)") {
                    println!("\nСгенерированные упорядоченные числа:");
                    let mut sorted_numbers = random_numbers.clone();
                    sorted_numbers.sort();
                    for num in &sorted_numbers {
                        println!("{}", num);
                    }
                } else {
                    println!("\nСгенерированные неупорядоченные числа:");
                    for num in &random_numbers {
                        println!("{}", num);
                    }
                };
}

/// Функция генерации уникальных чисел в заданном диапазоне
fn generate_unique_numbers(
    lower_bound: usize, 
    upper_bound: usize, 
    num_of_numbers: usize, 
    ignore_list: &HashSet<usize>
) -> Vec<usize> {
    let mut csprng = OsRng::default();
    let mut unique_numbers: Vec<usize> = Vec::new();
    let mut numbers_set: HashSet<usize> = HashSet::new();

    while unique_numbers.len() < num_of_numbers {
        let mut buffer = [0; 8];
        csprng.fill_bytes(&mut buffer);
        let random_number = lower_bound + usize::from_le_bytes(buffer) % (upper_bound - lower_bound + 1);

        if !ignore_list.contains(&random_number) && numbers_set.insert(random_number) {
            unique_numbers.push(random_number);
        }
    }

    unique_numbers
}

/// Функция чтения целочисленного ввода
fn read_usize_input(message: &str) -> usize {
    loop {
        println!("{}", message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
        match input.trim().parse() {
            Ok(num) if num > 0 && num <= 1000000 => return num,
            _ => println!("Ошибка. Требуется ввести целое число до 1 до 1 млн."),
        }
    }
}

/// Функция чтения списка чисел для игнорирования
fn read_ignore_list (message: &str) -> HashSet<usize> {
    println!("{}", message);
    let mut ignore_list: HashSet<usize> = HashSet::new();
    let mut inpute = String::new();
    io::stdin().read_line(&mut inpute).expect("Ошибка чтения ввода");
    for num in inpute.trim().split_whitespace() {
        if let Ok(num) = num.parse::<usize>() {
            ignore_list.insert(num);
        } else {
            println!("Ошибка. Некорректное число в списке игнорирования, оно будет проигнорировано.");
        }
    }

    ignore_list
}

/// Функция для запроса повторной генерации чисел
fn return_to_begin () -> bool {
    loop {
        println!("\nХотите сгенерировать еще числа? (y/n)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
        if let Some(choice) = input.trim().chars().next() {
            match choice {
                'y' => return true,
                'n' => return false,
                _ => println!("Ошибка. Введите 'y' или 'n'.")
            }
        } else {
            println!("Ошибка. Введите 'y' или 'n'.");
        }
    };
}

/// Функция для упорядочивания чисел
fn your_choice (message: &str) -> bool {
    loop {
        println!("{}", message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
    
        if let Some(choice) = input.trim().chars().next() {
            match choice.to_ascii_lowercase() {
                'y' => return true,
                'n' => return false,
                _ => println!("\nОшибка. Введите 'y' или 'n'."),
            }
        } else {
            println!("\nОшибка. Введите 'y' или 'n'.");
        };
    };
}