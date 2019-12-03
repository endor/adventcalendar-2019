// --- Day 3: Crossed Wires ---
//
// The gravity assist was successful, and you're well on your way to the Venus refuelling station. During the rush back on Earth, the fuel management system wasn't completely installed, so that's next on the priority list.
//
// Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a central port and extend outward on a grid. You trace the path each wire takes as it leaves the central port, one wire per line of text (your puzzle input).
//
// The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need to find the intersection point closest to the central port. Because the wires are on a grid, use the Manhattan distance for this measurement. While the wires do technically cross right at the central port where they both start, this point does not count, nor does a wire count as crossing with itself.
//
// For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o), it goes right 8, up 5, left 5, and finally down 3:
//
// ...........
// ...........
// ...........
// ....+----+.
// ....|....|.
// ....|....|.
// ....|....|.
// .........|.
// .o-------+.
// ...........
//
// Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4, and left 4:
//
// ...........
// .+-----+...
// .|.....|...
// .|..+--X-+.
// .|..|..|.|.
// .|.-X--+.|.
// .|..|....|.
// .|.......|.
// .o-------+.
// ...........
//
// These wires cross at two locations (marked X), but the lower-left one is closer to the central port: its distance is 3 + 3 = 6.
//
// Here are a few more examples:
//
//     R75,D30,R83,U83,L12,D49,R71,U7,L72
//     U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
//     R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
//
// What is the Manhattan distance from the central port to the closest intersection?
//
// --- Part Two ---
//
// It turns out that this circuit is very timing-sensitive; you actually need to minimize the signal delay.
//
// To do this, calculate the number of steps each wire takes to reach each intersection; choose the intersection where the sum of both wires' steps is lowest. If a wire visits a position on the grid multiple times, use the steps value from the first time it visits that position when calculating the total value of a specific intersection.
//
// The number of steps a wire takes is the total number of grid squares the wire has entered to get to that location, including the intersection being considered. Again consider the example from above:
//
// ...........
// .+-----+...
// .|.....|...
// .|..+--X-+.
// .|..|..|.|.
// .|.-X--+.|.
// .|..|....|.
// .|.......|.
// .o-------+.
// ...........
//
// In the above example, the intersection closest to the central port is reached after 8+5+5+2 = 20 steps by the first wire and 7+6+4+3 = 20 steps by the second wire for a total of 20+20 = 40 steps.
//
// However, the top-right intersection is better: the first wire takes only 8+5+2 = 15 and the second wire takes only 7+6+2 = 15, a total of 15+15 = 30 steps.
//
// Here are the best steps for the extra examples from above:
//
//     R75,D30,R83,U83,L12,D49,R71,U7,L72
//     U62,R66,U55,R34,D71,R55,D58,R83 = 610 steps
//     R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = 410 steps
//
// What is the fewest combined steps the wires must take to reach an intersection?

use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: i32,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn to_map_string(self) -> String {
        format!("{}:{}", self.x, self.y)
    }
}

fn parse_input(s: &str) -> Vec<Instruction> {
    s.split(',').map(|step| {
        let mut step = step.to_string();
        let direction = step.remove(0);
        let steps = step.parse::<i32>().expect("Should be a number");

        match direction {
            'R' => Instruction { direction: Direction::Right, steps: steps},
            'U' => Instruction { direction: Direction::Up, steps: steps},
            'L' => Instruction { direction: Direction::Left, steps: steps},
            'D' => Instruction { direction: Direction::Down, steps: steps},
            _   => panic!("Invalid direction"),
        }
    }).collect()
}

fn map_to_points(line: &Vec<Instruction>) -> HashMap<String, i32> {
    let mut total_steps = 0;
    let mut current_point = Point { x: 0, y: 0 };
    let mut map = HashMap::new();

    line.iter().for_each(|instruction| {
        for _ in 1..=instruction.steps {
            total_steps += 1;

            match instruction.direction {
                Direction::Right => {
                    current_point.x += 1;
                },
                Direction::Left => {
                    current_point.x -= 1;
                },
                Direction::Up => {
                    current_point.y += 1;
                },
                Direction::Down => {
                    current_point.y -= 1;
                },
            }

            if !map.contains_key(&current_point.to_map_string()) {
                map.insert(current_point.to_map_string(), total_steps);
            }
        }
    });

    map
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<Vec<Instruction>> = contents.lines().map(|l| parse_input(l)).collect();
    let points: Vec<HashMap<String, i32>> = lines.iter().map(|l| map_to_points(l)).collect();

    let line1 = &points[0];
    let line2 = &points[1];

    let mut steps: Vec<i32> = Vec::new();

    for (point, point_steps) in line1.iter() {
        if line2.contains_key(point) {
            steps.push(point_steps + line2.get(point).unwrap());
        }
    }

    println!("{:?}", steps.iter().min().unwrap());
}
