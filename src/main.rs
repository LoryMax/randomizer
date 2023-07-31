use rand::RngCore;
use rand::rngs::OsRng;
use std::collections::HashSet;
use std::io;

fn main() { 
    
    loop {
           process_numbers_generation();
        
        if !return_to_begin() {
            println!("\nДо свидания!\n");
            break;
        }
    }
}

// Функция генерации чисел
fn process_numbers_generation () {

    loop {
         let num_of_numbers: usize = read_usize_input("\nВведите количество чисел для генерации:");
         // Вводим верхнюю границу диапазона, пока не будет введено корректное значение            
         let (lower_bound, upper_bound) = loop {
            let lower_bound: usize = read_usize_input("Введите нижнюю границу диапазона:");
            let upper_bound: usize = read_usize_input("Введите верхнюю границу диапазона:");
    
            if upper_bound>lower_bound {
                // Проверка на возможность сгенерировать достаточное количество уникальных чисел            
                let available_numbers = (lower_bound..=upper_bound).collect::<HashSet<_>>();
                if available_numbers.len()<num_of_numbers {
                    println!("\nОшибка. Недостаточно уникальных чисел для генерации.");
                    println!("Пожалуйста, уменьшите количество чисел для генерации или уменьшите список чисел, которые нужно игнорировать.\n");
                    continue;
                }
                break (lower_bound, upper_bound);
            } else {
                println!("\nОшибка. Верхняя граница должна быть больше нижней. Повторите ввод обоих границ.\n");
            }
         };

         let ignore_list = read_ignore_list("Введите список чисел, которые нужно игнорировать, через пробел:");
         // Проверка на возможность сгенерировать достаточное количество уникальных чисел, учитывая список исключения      
         let available_numbers = (lower_bound..=upper_bound)
            .filter(|num| !ignore_list.contains(num))
            .collect::<HashSet<_>>();
         if available_numbers.len()<num_of_numbers {
            println!("\nОшибка. Недостаточно уникальных чисел для генерации.");
            println!("Пожалуйста, уменьшите количество чисел для генерации или уменьшите список чисел, которые нужно игнорировать.\n");
            continue;
         }

         let unnormal_fraction_range:f64 = 0.7;
         let selected_fraction: f64 = (num_of_numbers as f64 / (upper_bound - lower_bound + 1) as f64).round();

         if selected_fraction >= unnormal_fraction_range {
            println!("\n>>> Предупреждение! <<<");
            println!("Слишком маленький диапазон чисел. Доля выбранных чисел {} %.", selected_fraction*100.0);
            println!("Попробуйте увеличить верхнюю границу диапазона или уменьшить количество чисел для генерации.\n");
            }

            let random_numbers = generate_unique_numbers(lower_bound, upper_bound, num_of_numbers, &ignore_list);
            
            if num_of_numbers == 1 {
            println!("\nСгенерированное число: {}", random_numbers[0]);

         } else if order_choice() {
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

            break;
     }
}

// Функция генерации уникальных чисел в заданном диапазоне
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

// Функция чтения целочисленного ввода
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

// Функция чтения списка чисел для игнорирования
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

// Функция для запроса повторной генерации чисел
fn return_to_begin () -> bool {
    loop {
        println!("");
        println!("Хотите сгенерировать еще числа? (y/n)");

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

// Функция для упорядочивания чисел
fn order_choice () -> bool {
    loop {
        println!("\nТребуется ли упорядочить числа? (y/n)");
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