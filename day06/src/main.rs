// --- Day 6: Universal Orbit Map ---
//
// You've landed at the Universal Orbit Map facility on Mercury. Because navigation in space often involves transferring between orbits, the orbit maps here are useful for finding efficient routes between, for example, you and Santa. You download a map of the local orbits (your puzzle input).
//
// Except for the universal Center of Mass (COM), every object in space is in orbit around exactly one other object. An orbit looks roughly like this:
//
//                   \
//                    \
//                     |
//                     |
// AAA--> o            o <--BBB
//                     |
//                     |
//                    /
//                   /
//
// In this diagram, the object BBB is in orbit around AAA. The path that BBB takes around AAA (drawn with lines) is only partly shown. In the map data, this orbital relationship is written AAA)BBB, which means "BBB is in orbit around AAA".
//
// Before you use your map data to plot a course, you need to make sure it wasn't corrupted during the download. To verify maps, the Universal Orbit Map facility uses orbit count checksums - the total number of direct orbits (like the one shown above) and indirect orbits.
//
// Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain can be any number of objects long: if A orbits B, B orbits C, and C orbits D, then A indirectly orbits D.
//
// For example, suppose you have the following map:
//
// COM)B
// B)C
// C)D
// D)E
// E)F
// B)G
// G)H
// D)I
// E)J
// J)K
// K)L
//
// Visually, the above map of orbits looks like this:
//
//         G - H       J - K - L
//        /           /
// COM - B - C - D - E - F
//                \
//                 I
//
// In this visual representation, when two objects are connected by a line, the one on the right directly orbits the one on the left.
//
// Here, we can count the total number of orbits as follows:
//
//     D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
//     L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.
//     COM orbits nothing.
//
// The total number of direct and indirect orbits in this example is 42.
//
// What is the total number of direct and indirect orbits in your map data?
//
// --- Part Two ---
//
// Now, you just need to figure out how many orbital transfers you (YOU) need to take to get to Santa (SAN).
//
// You start at the object YOU are orbiting; your destination is the object SAN is orbiting. An orbital transfer lets you move from any object to an object orbiting or orbited by that object.
//
// For example, suppose you have the following map:
//
// COM)B
// B)C
// C)D
// D)E
// E)F
// B)G
// G)H
// D)I
// E)J
// J)K
// K)L
// K)YOU
// I)SAN
//
// Visually, the above map of orbits looks like this:
//
//                           YOU
//                          /
//         G - H       J - K - L
//        /           /
// COM - B - C - D - E - F
//                \
//                 I - SAN
//
// In this example, YOU are in orbit around K, and SAN is in orbit around I. To move from K to I, a minimum of 4 orbital transfers are required:
//
//     K to J
//     J to E
//     E to D
//     D to I
//
// Afterward, the map of orbits looks like this:
//
//         G - H       J - K - L
//        /           /
// COM - B - C - D - E - F
//                \
//                 I - SAN
//                  \
//                   YOU
//
// What is the minimum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting? (Between the objects they are orbiting - not between YOU and SAN.)


use std::collections::HashMap;
use std::fs;

fn count_orbits(map: &HashMap<String, Vec<String>>, object: String, depth: i32) -> i32 {
    let objects = map.get(&object);

    if objects.is_some() {
        let sum: i32 = objects.unwrap().iter()
            .map(|o| count_orbits(map, o.to_string(), depth + 1)).sum();
        depth + sum
    } else {
        depth
    }
}

fn path_for(map: &HashMap<String, Vec<String>>, from: String, to: &str) -> Vec<String> {
    match map.get(&from) {
        None => Vec::new(),
        Some(objects) => {
            if objects.contains(&to.to_string()) {
                let mut vec = Vec::new();
                vec.push(from);
                vec
            } else {
                objects.iter()
                    .map(|o| {
                        path_for(map, o.to_string(), to)
                    })
                    .filter(|v| v.len() > 0)
                    .nth(0)
                    .map(|mut v| { v.push(from); v })
                    .unwrap_or(Vec::new())
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    contents.lines().for_each(|l| {
        let mut split = l.split(')');
        let object1 = split.nth(0).unwrap().to_string();
        let object2 = split.nth(0).unwrap().to_string();

        let vec = map.entry(object1).or_insert(Vec::new());
        vec.push(object2);
    });

    let mut i_am_in_orbit_of = "COM".to_string();
    let mut santa_is_in_orbit_of = "COM".to_string();

    map.iter().for_each(|(object, objects)| {
        if objects.contains(&"YOU".to_string()) {
            i_am_in_orbit_of = object.to_string();
        } else if objects.contains(&"SAN".to_string()) {
            santa_is_in_orbit_of = object.to_string();
        }
    });

    let mut path_for_me = path_for(&map, "COM".to_string(), &i_am_in_orbit_of);
    path_for_me.reverse();

    let mut path_for_santa = path_for(&map, "COM".to_string(), &santa_is_in_orbit_of);
    path_for_santa.reverse();

    loop {
        if path_for_me[0] == path_for_santa[0] {
            path_for_me.remove(0);
            path_for_santa.remove(0);

            if path_for_me.is_empty() || path_for_santa.is_empty() {
                break;
            }
        } else {
            break;
        }
    }

    println!("{:?}", path_for_me.len() + 1 + path_for_santa.len() + 1);
}
