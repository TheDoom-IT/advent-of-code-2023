use std::cmp::min;
use std::fs;

struct Map {
    mappings: Vec<Mapping>,
}
struct Mapping {
    destination_start: u64,
    source_start: u64,
    source_end: u64,
}

#[derive(Clone)]
struct Range {
    start: u64,
    end: u64,
}

pub fn main() {
    let content = fs::read_to_string("./src/day5/input.txt").expect("Could not read file");

    // parse input
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();

    let mut are_seeds_handled = false;
    let mut is_inside_map = false;
    let mut current_map = Map {
        mappings: Vec::new(),
    };
    for line in content.lines() {
        //handle seeds
        if !are_seeds_handled {
            let seeds_line_split: Vec<&str> = line.split(':').collect();
            if seeds_line_split.len() != 2 {
                panic!("Invalid line: {}", line);
            }
            let seeds_str = seeds_line_split[1].trim();
            seeds = get_numbers_from_text(seeds_str);
            are_seeds_handled = true;
            continue;
        }

        if line.len() == 0 {
            if is_inside_map {
                //save mapping here
                maps.push(current_map);
                current_map = Map {
                    mappings: Vec::new(),
                };
                is_inside_map = false;
            }
            continue;
        }

        if is_inside_map {
            // add element to mapping
            let map_numbers = get_numbers_from_text(line);
            if map_numbers.len() != 3 {
                panic!("Invalid line: {}", line);
            }
            let mapping = Mapping {
                destination_start: map_numbers[0],
                source_start: map_numbers[1],
                source_end: map_numbers[1] + map_numbers[2] - 1,
            };
            current_map.mappings.push(mapping);
            continue;
        }

        // check for a new mapping
        if line.contains("map:") {
            is_inside_map = true;
            continue;
        } else {
            panic!("Invalid line: {}", line)
        }
    }
    if is_inside_map {
        maps.push(current_map);
    }

    // sort mappings - required for step 2
    maps.iter_mut().for_each(|map| {
        map.mappings
            .sort_by(|a, b| a.source_start.cmp(&b.source_start))
    });

    let locations: Vec<u64> = seeds
        .iter()
        .map(|seed| map_seed_to_location(*seed, &maps))
        .collect();

    let min_location = locations.iter().min().unwrap();

    println!("Task 1: {}", min_location); //51580674

    //task 2
    let mut seeds_range: Vec<Range> = Vec::new();
    for chunk in seeds.chunks(2) {
        if chunk.len() != 2 {
            panic!("Invalid chunk: {:?}", chunk);
        }
        seeds_range.push(Range {
            start: chunk[0],
            end: chunk[0] + chunk[1] - 1,
        });
    }

    let result: Vec<Range> = map_seed_ranges_to_locations(&seeds_range, &maps);
    let min_seed = result.iter().map(|v| v.start).min().unwrap();
    println!("Task 2: {}", min_seed); //99751240
}

fn convert_to_destination(mut source: Range, mapping: &Mapping) -> Range {
    let range = source.end - source.start;
    let bias = source.start - mapping.source_start;
    source.start = mapping.destination_start + bias;
    source.end = mapping.destination_start + range + bias;
    return source;
}

fn map_seed_ranges_to_locations(seeds: &Vec<Range>, maps: &Vec<Map>) -> Vec<Range> {
    let mut current_seeds: Vec<Range> = Vec::from_iter(seeds.clone());

    for map in maps.iter() {
        let mut next_seeds: Vec<Range> = Vec::new();
        for seed in current_seeds.iter() {
            let mut curr_seed = seed.clone();
            for (index, mapping) in map.mappings.iter().enumerate() {
                if curr_seed.start > curr_seed.end {
                    break;
                }

                if curr_seed.start < mapping.source_start {
                    let end = min(curr_seed.end, mapping.source_start - 1);
                    next_seeds.push(Range {
                        start: curr_seed.start,
                        end,
                    });
                    curr_seed.start = mapping.source_start;
                    if curr_seed.start > curr_seed.end {
                        break;
                    }
                }
                if curr_seed.start >= mapping.source_start && curr_seed.start <= mapping.source_end
                {
                    let end = min(mapping.source_end, curr_seed.end);
                    // convert to destination
                    next_seeds.push(convert_to_destination(
                        Range {
                            start: curr_seed.start,
                            end,
                        },
                        mapping,
                    ));
                    curr_seed.start = end + 1;
                }

                if index == map.mappings.len() - 1 && curr_seed.start <= curr_seed.end {
                    //failed to find mapping
                    next_seeds.push(Range {
                        start: curr_seed.start,
                        end: curr_seed.end,
                    });
                }
            }
        }
        current_seeds = next_seeds;
    }

    return current_seeds;
}

fn map_seed_to_location(seed: u64, maps: &Vec<Map>) -> u64 {
    let mut curr_value = seed;
    for map in maps {
        for mapping in map.mappings.iter() {
            if curr_value >= mapping.source_start && curr_value <= mapping.source_end {
                curr_value = mapping.destination_start + (curr_value - mapping.source_start);
                break;
            }
        }
    }
    return curr_value;
}

fn get_numbers_from_text(text: &str) -> Vec<u64> {
    text.split(' ')
        .map(|n| n.trim().parse::<u64>())
        .filter(|n| n.is_ok())
        .map(|n| n.unwrap())
        .collect()
}
