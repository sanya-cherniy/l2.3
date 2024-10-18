use clap::{Arg, ArgMatches, Command};
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Инициализируем аргументы командной строки
    let matches = args_init();

    // Получаем путь к файлу
    let path = matches.get_one::<String>("input").unwrap();
    // Открываем файл и считываем его в память
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    // Преобразовываем файл в массив строк
    let mut str_vec: Vec<String> = buffered.lines().map(|x| x.unwrap()).collect();

    // Вызываем функцию сортировки массива строк
    sort(&mut str_vec, &matches);

    let mut buffer = String::new(); // переменная-буфер для флага -u

    if matches.get_flag("reverse") {
        // Если указан флаг -r - выводим отсортированнные строки в обратном порядке
        for line in str_vec.iter().rev() {
            if matches.get_flag("unique") && *line == buffer {
                continue;
            }
            buffer = line.clone();
            println!("{}", line);
        }
    } else {
        // Выводим отсортированнные строки
        for line in str_vec {
            if matches.get_flag("unique") && line == buffer {
                // Если флаг -u включен и строка соответствуюет предыдущей, пропускаем ее
                continue;
            }
            buffer = line.clone();
            println!("{}", line);
        }
    }
}

// Функция для сортировки массива строк
fn sort(vec: &mut Vec<String>, matches: &ArgMatches) {
    vec.sort_by(|a, b| {
        // Получаем срезы обеих строк
        let s1 = slice_for_key(a, matches);
        let s2 = slice_for_key(b, matches);
        //Если включен флаг -n
        if matches.get_flag("numeric") {
            //извлекаем из обеих строк наибольшее число от начала строки и сравниваем между собой
            if let (Ok(value1), Ok(value2)) = (find_largest_f64(&s1), find_largest_f64(&s2)) {
                if (value1 - value2) > 1e-10 {
                    return std::cmp::Ordering::Greater;
                } else if (value2 - value1) > 1e-10 {
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Equal;
                }
            } else if let (Ok(_), Err(_)) = (find_largest_f64(&s1), find_largest_f64(&s2)) {
                // Если первая строка начинается с числа а вторая нет - первая всегда считается "большей"
                return std::cmp::Ordering::Greater;
            } else if let (Err(_), Ok(_)) = (find_largest_f64(&s1), find_largest_f64(&s2)) {
                // Если вторая строка начинается с числа а первая нет - вторая всегда считается "большей"
                return std::cmp::Ordering::Less;
            }
        }
        // Если не включен флаг -n либо включен но обе строки не начинаются с чисел, сравниваем как обычные строки
        let cmp = s1.to_lowercase().cmp(&s2.to_lowercase()); // сравниваем строки в нижнем регистре
        if cmp == std::cmp::Ordering::Equal {
            // если отличий нет, проходим по каждому символу обеих строк и сравниваем их регистры, строка в которой первым будет найдет символ верхнего регистра считается "большей"
            if s1.chars().next().unwrap().is_lowercase()
                && s2.chars().next().unwrap().is_uppercase()
            {
                std::cmp::Ordering::Less
            } else if s1.chars().next().unwrap().is_uppercase()
                && s2.chars().next().unwrap().is_lowercase()
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        } else {
            cmp
        }
    });
}

// Функция для обработки аргумента указанного при флаге "-k", возвращает массив в котором: элемент 0 - первая колонка из диапазона, элемент 1 - номер символа из первой колонки диапазона, элемент 2 - последняя колонка из диапазона, элемент 3 - последний символ из  последней колонки диапазона, в случае если последний символ не указан или последняя колонка не укзана - конец диапазона последний символ последней колонки в таком случае соответствующим значениям будет присвоен ноль

fn validate_input(val: &str) -> Result<Vec<usize>, &str> {
    let mut res = vec![];

    // Если указанная строка парсится в число - сортировку проводим только по одной колонке, добавляем ее номер в массив остальные значения заполняем нулями и возвращаем в качестве результата
    if let Ok(value) = val.parse::<usize>() {
        if value == 0 {
            return Err("field number is zero: invalid field specification)");
        }
        res.push(value);
        res.push(0);
        res.push(0); //
        res.push(0); // заполняем остальные значения нулями
        return Ok(res);
    }

    // Если указанную строку можно разделить по ","
    if let Some((start, end)) = val.split_once(',') {
        // Если начало и конец разбиения пустые строки - указан только символ ',' , некорректный случай
        if start == "" || end == "" {
            return Err("invalid number nearby ',': invalid count at start");
        }

        // Проверка подстроки идущей перед запятой

        // Если указанную строку можно разделить по "."
        if let Some((start, end)) = start.split_once('.') {
            if start == "" || end == "" {
                return Err("invalid number nearby '.': invalid count");
            } else {
                // Если обе части парсятся в числа - указана первая колонка диапазона и первый символ
                if let Ok(start) = start.parse::<usize>() {
                    if let Ok(end) = end.parse::<usize>() {
                        if start == 0 || end == 0 {
                            return Err("field number is zero: invalid field specification");
                        }
                        res.push(start);
                        res.push(end);
                    } else {
                        return Err("invalid number after ',': invalid count at start");
                    }
                } else {
                    return Err("invalid number before '.': invalid count");
                }
            }
        } else {
            if let Ok(value) = start.parse::<usize>() {
                if value == 0 {
                    return Err("field number is zero: invalid field specification)");
                }
                res.push(value);
                res.push(0);
            } else {
                return Err("invalid number after ',': invalid count at start");
            }
        }
        if let Some((start, end)) = end.split_once('.') {
            if start == "" || end == "" {
                return Err("invalid number nearby '.': invalid count");
            } else {
                if let Ok(start) = start.parse::<usize>() {
                    if let Ok(end) = end.parse::<usize>() {
                        if start == 0 || end == 0 {
                            return Err("field number is zero: invalid field specification)");
                        }
                        res.push(start);
                        res.push(end);
                    } else {
                        return Err("invalid number after '.': invalid count");
                    }
                } else {
                    return Err("invalid number after '.': invalid count");
                }
            }
        } else {
            if let Ok(value) = end.parse::<usize>() {
                if value == 0 {
                    return Err("field number is zero: invalid field specification)");
                }
                res.push(value);
                res.push(0);
            } else {
                return Err("invalid number after ',': invalid count at start");
            }
        }
        return Ok(res);
    }
    // Если указанная строка разбивается по разделителю: '.' - указана первая колонка и первый символ диапазона
    if let Some((start, end)) = val.split_once('.') {
        if start == "" || end == "" {
            return Err("invalid number nearby '.': invalid count");
        } else {
            if let Ok(start) = start.parse::<usize>() {
                if let Ok(end) = end.parse::<usize>() {
                    if start != 0 && end != 0 {
                        res.push(end);
                        res.push(start);
                        res.push(0);
                        res.push(0);
                        return Ok(res);
                    } else {
                        return Err("field number is zero: invalid field specification)");
                    }
                    // res.push(0);
                } else {
                    return Err("invalid number after ',': invalid count at start");
                }
            } else {
                return Err("invalid number before ',': invalid count at start");
            }
        }
    }

    Err("invalid number")
}

// Функция получения наибольшего числа от начала строки, если строка начинается не с цифры - возвращает Err
fn find_largest_f64(s: &str) -> Result<f64, ()> {
    let mut largest_slice = 0.0; // здесь храним результат
    for end in 0..s.len() {
        let slice = &s[0..=end]; // берем срез строки от начала и на каждой итерции цикла увеличиваем длину на единицу
        if let Ok(value) = slice.trim().parse::<f64>() {
            // если полученный срез парсится в f64, записываем его в результат
            largest_slice = value;
        } else {
            // если срез не парсится в f64
            if end == 0 {
                // в случае если был рассмотрен только первый символ возвращаем Err
                return Err(());
            }
            break; // выходим из цикла, результатом работы будет число полученное на предыдущей итерации
        }
    }

    Ok(largest_slice)
}

// Функция для получения среза строки в определенном промежутке если указан флаг -k, если флаг не указан возвращаем
fn slice_for_key(input: &String, matches: &ArgMatches) -> String {
    let mut s1 = String::new();

    let input = input.split_whitespace().collect::<Vec<&str>>();

    if matches.contains_id("columns") {
        // Получаем значения колонок
        let columns = validate_input(matches.get_one::<String>("columns").unwrap()).unwrap();
        let mut first_line_index = columns[0]; // Номер первой строки
        let mut first_char_index = columns[1]; // Номер первого символа
        let mut second_line_index = columns[2]; // Номер второй строки
        let mut second_char_index = columns[3]; // Номер второго символа

        // Для обработки строк уменьшаем значения на единицу, т.к. индексы начинаются с нуля а пользователь указывает начиная с единицы
        if first_line_index > 0 {
            first_line_index -= 1;
        }
        if first_char_index > 0 {
            first_char_index -= 1;
        }
        if second_line_index > 0 {
            second_line_index -= 1;
        }
        if second_char_index > 0 {
            second_char_index -= 1;
        }

        // Проверяем что индексы не выходят за длину строки
        if first_line_index < input.len() && second_line_index < input.len() {
            let range = &input[first_line_index..=second_line_index]; // создаем срез

            for (i, line) in range.iter().enumerate() {
                if i == 0 {
                    // Для первой строки добавляем срез от start до конца строки
                    s1.push_str(&line[first_char_index..]);
                } else if i == range.len() - 1 {
                    // Для последней строк добавляем всю строку если ее символ выходит за границу или не указан
                    if second_char_index == 0 || second_char_index > line.len() {
                        s1.push_str(line);
                    } else {
                        // Для последней строки добавляем срез от начала до указанного символа
                        s1.push_str(&line[..=second_char_index]);
                    }
                } else {
                    // Для промежуточных строк добавляем всю строку
                    s1.push_str(line);
                }
            }
        } else {
            s1 = input.join("");
        }
    } else {
        s1 = input.join("");
    }
    s1
}

// Функция для инициализации аргументов командной строки с использованием библиотеки clap
fn args_init() -> ArgMatches {
    let matches = Command::new("My sort")
        .version("1.0")
        .about("cut analog")
        .arg(
            Arg::new("columns")
                .short('k')
                .help("sort via a key; KEYDEF gives location and type")
                .value_parser(clap::builder::NonEmptyStringValueParser::new()), // после флага должно быть указано значение, являющееся непустой строкой
        )
        .arg(
            Arg::new("numeric")
                .short('n')
                .help("compare according to string numerical value")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("reverse")
                .short('r')
                .help("reverse the result of comparisons")
                .action(clap::ArgAction::SetTrue), // .action(clap::ArgAction::SetTrue), // .required(false),
        )
        .arg(
            Arg::new("unique")
                .short('u')
                .action(clap::ArgAction::SetTrue)
                .help("with -c, check for strict ordering; without -c, output only the first of an equal run"), // .action(clap::ArgAction::SetTrue), // .required(false),
        )
                .arg(
            Arg::new("input").help("Input file to use").required(true), // .index(1),
        )
        .get_matches();
    return matches;
}
