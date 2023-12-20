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

    connections_to_conjunctions
        .into_iter()
        .for_each(|(from, to)| {
            let module = modules.get_mut(to).unwrap();
            // Always a conjunction by design, need to extract memory
            if let ModuleType::Conjunction(memory) = &mut module.module_type {
                memory.insert(from, PulseType::Low);
            }
        });

    let mut highs = 0;
    let mut lows = 0;

    // Queue of pulses to be processed (FIFO)
    let mut pulses: VecDeque<Pulse> = VecDeque::new();

    // Press HQ button 1000 times, and count number of high and low pulses
    for _ in 0..1000 {
        // Initial pulse
        pulses.push_back(Pulse {
            from: "HQ",
            to: "broadcaster",
            pulse_type: PulseType::Low,
        });

        // Handle all pulses resulting from pressing HQ button
        while let Some(pulse) = pulses.pop_front() {
            match pulse.pulse_type {
                PulseType::High => highs += 1,
                PulseType::Low => lows += 1,
            }

            // Get module that pulse is sent to
            let module = match modules.get_mut(pulse.to) {
                Some(m) => m,
                None => {
                    // Apparently there are some destination modules that are not defined
                    // in the input, e.g. "rx", ignore these.
                    continue;
                }
            };

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
    }

    Some((highs * lows).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "32000000");
    }

    #[test]
    fn test_task2() {
        let input = include_str!("../example2.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "11687500");
    }
}
