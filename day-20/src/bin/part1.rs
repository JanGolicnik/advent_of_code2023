use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Signal {
    High,
    Low,
}

#[derive(Debug)]
enum Module<'a> {
    Broadcaster,
    FlipFlop(Signal),
    Conjunction(HashMap<&'a str, Signal>),
}

fn send_signal(
    modules: &mut HashMap<&str, (Module, Vec<&str>)>,
    low_counter: &mut u32,
    high_counter: &mut u32,
) {
    let mut pending_signals: VecDeque<(&str, Signal, &str)> =
        vec![("broadcaster", Signal::Low, "button")].into();
    while !pending_signals.is_empty() {
        let (target_module, signal, sender) = pending_signals.pop_front().unwrap();
        match signal {
            Signal::High => *high_counter += 1,
            Signal::Low => *low_counter += 1,
        }

        if !modules.contains_key(target_module) {
            continue;
        }

        let prev_pending_signals_size = pending_signals.len();

        let (module, destinations) = modules.get_mut(target_module).unwrap();
        match module {
            Module::Broadcaster => {
                for dest in destinations {
                    pending_signals.push_back((dest, signal, target_module))
                }
            }
            Module::FlipFlop(val) => {
                if let Signal::Low = signal {
                    match val {
                        Signal::High => *val = Signal::Low,
                        Signal::Low => *val = Signal::High,
                    }
                    for dest in destinations {
                        pending_signals.push_back((dest, *val, target_module))
                    }
                }
            }
            Module::Conjunction(received) => {
                *received.get_mut(sender).unwrap() = signal;
                let mut sent_signal: Signal = Signal::Low;
                for el in received.iter() {
                    if *(el.1) == Signal::Low {
                        sent_signal = Signal::High;
                        break;
                    }
                }
                for dest in destinations {
                    pending_signals.push_back((dest, sent_signal, target_module))
                }
            }
        }

        // println!("Sent from {target_module} to: ");
        // for item in pending_signals.iter().skip(prev_pending_signals_size) {
        //     print!("{:?}", item);
        // }
        // println!();
    }
}

fn part1(input: &str) -> String {
    let mut modules: HashMap<&str, (Module, Vec<&str>)> = HashMap::new();
    for line in input.lines() {
        let arrow_index = line.find(" -> ").unwrap();
        let destinations: Vec<&str> = line[arrow_index + 4..]
            .split(',')
            .map(|e| e.trim())
            .collect();
        let first_char = line.chars().next().unwrap();
        match first_char {
            '%' => {
                modules.insert(
                    &line[1..arrow_index],
                    (Module::FlipFlop(Signal::Low), destinations),
                );
            }
            '&' => {
                modules.insert(
                    &line[1..arrow_index],
                    (Module::Conjunction(HashMap::new()), destinations),
                );
            }
            _ => {
                modules.insert(&line[..arrow_index], (Module::Broadcaster, destinations));
            }
        }
    }

    let mut to_add: HashMap<&str, Vec<&str>> = HashMap::new();
    for (module_name, (_, destinations)) in modules.iter() {
        for dest in destinations {
            if dest != module_name {
                if !modules.contains_key(*dest) {
                    continue;
                }
                if let Module::Conjunction(_) = modules[*dest].0 {
                    if to_add.contains_key(*dest) {
                        to_add.get_mut(*dest).unwrap().push(module_name);
                    } else {
                        to_add.insert(*dest, vec![*module_name]);
                    }
                }
            }
        }
    }

    for (module, to_add) in to_add {
        if let Module::Conjunction(ref mut module) = modules.get_mut(module).unwrap().0 {
            for to_add in to_add {
                module.insert(to_add, Signal::Low);
            }
        }
    }

    let mut low_counter = 0;
    let mut high_counter = 0;
    for _ in 0..1000 {
        send_signal(&mut modules, &mut low_counter, &mut high_counter);
    }

    (low_counter * high_counter).to_string()
}
