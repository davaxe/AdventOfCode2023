//! Module for solving part 2. Important to note is that this code will not generally
//! work for other inputs than the one given in the problem description. This is because
//! the code relies on specific properties of the input, e.g. the fact that `cn` is the
//! only module that can send high pulses to `rx`, and that `cn` is (backwards) connected
//! to `ch`, `gh`, `sv`, and `th`.

use std::collections::VecDeque;

use crate::parser::{self, Module, ModuleType, PulseType, State};

#[derive(Debug)]
struct Pulse<'a> {
    from: &'a str,
    to: &'a str,
    pulse_type: PulseType,
}

pub fn task(input: &str) -> Option<String> {
    let (_, mut modules) = parser::modules(input).ok()?;

    // Pre-process all modules by adding initial memory to conjunctions, i.e.
    // memory for all connections to conjunctions will be initialized to low.
    let all_conjunctions: Vec<_> = modules
        .iter()
        .filter_map(|(_, m)| match m.module_type {
            ModuleType::Conjunction(_) => Some(m.identifier),
            _ => None,
        })
        .collect();

    // All connections to conjunctions (from, to), where to is a conjunction
    let connections_to_conjunctions: Vec<(&str, &str)> = modules
        .iter()
        .flat_map(|(_, m)| {
            m.connections
                .iter()
                .filter(|&&c| all_conjunctions.contains(&c))
                .map(|&c| (m.identifier, c))
                .collect::<Vec<_>>()
        })
        .collect();

    // Initialize memory
    connections_to_conjunctions
        .into_iter()
        .for_each(|(from, to)| {
            let module = modules.get_mut(to).unwrap();
            // Always a conjunction by design, need to extract memory
            if let ModuleType::Conjunction(memory) = &mut module.module_type {
                memory.insert(from, PulseType::Low);
            }
        });

    // Queue of pulses to be processed (FIFO)
    let mut pulses: VecDeque<Pulse> = VecDeque::new();
    let mut cycle_lengths = Vec::new();

    // Press HQ button 1000 times, and count number of high and low pulses
    for press in 1.. {
        // Initial pulse
        pulses.push_back(Pulse {
            from: "HQ",
            to: "broadcaster",
            pulse_type: PulseType::Low,
        });

        while let Some(pulse) = pulses.pop_front() {
            // Get module that pulse is sent to
            let module = match modules.get_mut(pulse.to) {
                Some(m) => m,
                None => {
                    // Apparently there are some destination modules that are not defined
                    // in the input, e.g. "rx", ignore these.
                    continue;
                }
            };

            // `cn` is the only module that can send high pulses to `rx`, and
            // `cn` is (backwards) connected to `ch`, `gh`, `sv`, and `th`. Since,
            // `cn` is a conjunction, it will only send a low pulse to `rx` when
            // all of its connections are high. Therefore, if we find cycle lengths
            // for `ch`, `gh`, `sv`, and `th`, we can calculate the least common
            // multiple of these cycle lengths, and that will be the number of
            // presses required to send a low pulse to `rx`.
            match pulse {
                Pulse {
                    from: "ch",
                    to: "cn",
                    pulse_type: PulseType::High,
                } => {
                    if !cycle_lengths.iter().any(|(name, _)| *name == "ch") {
                        cycle_lengths.push(("ch", press));
                    }
                }
                Pulse {
                    from: "gh",
                    to: "cn",
                    pulse_type: PulseType::High,
                } => {
                    if !cycle_lengths.iter().any(|(name, _)| *name == "gh") {
                        cycle_lengths.push(("gh", press));
                    }
                }
                Pulse {
                    from: "sv",
                    to: "cn",
                    pulse_type: PulseType::High,
                } => {
                    if !cycle_lengths.iter().any(|(name, _)| *name == "sv") {
                        cycle_lengths.push(("sv", press));
                    }
                }
                Pulse {
                    from: "th",
                    to: "cn",
                    pulse_type: PulseType::High,
                } => {
                    if !cycle_lengths.iter().any(|(name, _)| *name == "th") {
                        cycle_lengths.push(("th", press));
                    }
                }
                _ => {}
            }

            // Module to send pulse to
            match module {
                Module {
                    module_type: ModuleType::Broadcast,
                    connections,
                    ..
                } => {
                    // Broadcast pulse to all connections
                    pulses.extend(connections.iter().map(|&c| Pulse {
                        from: pulse.to,
                        to: c,
                        pulse_type: pulse.pulse_type,
                    }));
                }
                Module {
                    module_type: ModuleType::Conjunction(memory),
                    connections,
                    ..
                } => {
                    // Store pulse in memory
                    memory
                        .entry(pulse.from)
                        .and_modify(|e| *e = pulse.pulse_type)
                        .or_insert(pulse.pulse_type);

                    // If all connections are high, send low pulse to all connections
                    // otherwise send high pulse to all connections
                    let high = memory.values().all(|&v| v == PulseType::High);
                    let pulse_type = if high {
                        PulseType::Low
                    } else {
                        PulseType::High
                    };

                    // Send pulse to all connections
                    pulses.extend(connections.iter().map(|&c| Pulse {
                        from: pulse.to,
                        to: c,
                        pulse_type,
                    }));
                }
                // Flip-flop module only reacts to low pulses
                Module {
                    module_type: ModuleType::FlipFlop(state),
                    connections,
                    ..
                } if pulse.pulse_type == PulseType::Low => {
                    // Off -> On + send high pulse to all connections
                    // On -> Off + send low pulse to all connections
                    let pulse_type = match state {
                        State::On => PulseType::Low,
                        State::Off => PulseType::High,
                    };

                    // Toggle state
                    *state = match state {
                        State::On => State::Off,
                        State::Off => State::On,
                    };

                    // Send pulse to all connections
                    pulses.extend(connections.iter().map(|&c| Pulse {
                        from: pulse.to,
                        to: c,
                        pulse_type,
                    }));
                }
                _ => {}
            }
        }

        // Check if we have found all cycle lengths
        if cycle_lengths.len() == 4 {
            break;
        }
    }

    let cycle_lengths: Vec<usize> = cycle_lengths
        .iter()
        .map(|(_, cycle_length)| *cycle_length)
        .collect();

    // Find least common multiple of all cycle lengths, i.e. press when all modules
    // send high pulses at the same time => rx receives low pulse
    let lcm = lcm(&cycle_lengths);

    Some(lcm.to_string())
}

fn lcm(numbers: &[usize]) -> usize {
    if numbers.len() == 1 {
        return numbers[0];
    }
    let a = numbers[0];
    let b = lcm(&numbers[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "No test case for part 2"]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "");
    }
}
