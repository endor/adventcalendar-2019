// Sadly, this code doesn't work yet. I had a HashMap implementation that worked for the test run and resulted in a stack
// overflow in the boost mode. I tried to change it to a Vec implementation, but end up with either a stack overflow
// or an index that's too big for the Vec.
//
// --- Day 9: Sensor Boost ---
//
// You've just said goodbye to the rebooted rover and left Mars when you receive a faint distress signal coming from the asteroid belt. It must be the Ceres monitoring station!
//
// In order to lock on to the signal, you'll need to boost your sensors. The Elves send up the latest BOOST program - Basic Operation Of System Test.
//
// While BOOST (your puzzle input) is capable of boosting your sensors, for tenuous safety reasons, it refuses to do so until the computer it runs on passes some checks to demonstrate it is a complete Intcode computer.
//
// Your existing Intcode computer is missing one key feature: it needs support for parameters in relative mode.
//
// Parameters in mode 2, relative mode, behave very similarly to parameters in position mode: the parameter is interpreted as a position. Like position mode, parameters in relative mode can be read from or written to.
//
// The important difference is that relative mode parameters don't count from address 0. Instead, they count from a value called the relative base. The relative base starts at 0.
//
// The address a relative mode parameter refers to is itself plus the current relative base. When the relative base is 0, relative mode parameters and position mode parameters with the same value refer to the same address.
//
// For example, given a relative base of 50, a relative mode parameter of -7 refers to memory address 50 + -7 = 43.
//
// The relative base is modified with the relative base offset instruction:
//
//     Opcode 9 adjusts the relative base by the value of its only parameter. The relative base increases (or decreases, if the value is negative) by the value of the parameter.
//
// For example, if the relative base is 2000, then after the instruction 109,19, the relative base would be 2019. If the next instruction were 204,-34, then the value at address 1985 would be output.
//
// Your Intcode computer will also need a few other capabilities:
//
//     The computer's available memory should be much larger than the initial program. Memory beyond the initial program starts with the value 0 and can be read or written like any other memory. (It is invalid to try to access memory at a negative address, though.)
//     The computer should have support for large program. Some instructions near the beginning of the BOOST program will verify this capability.
//
// Here are some example programs that use these features:
//
//     109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99 takes no input and produces a copy of itself as output.
//     1102,34915192,34915192,7,4,7,99,0 should output a 16-digit number.
//     104,1125899906842624,99 should output the large number in the middle.
//
// The BOOST program will ask for a single input; run it in test mode by providing it the value 1. It will perform a series of checks on each opcode, output any opcodes (and the associated parameter modes) that seem to be functioning incorrectly, and finally output a BOOST keycode.
//
// Once your Intcode computer is fully functional, the BOOST program should report no malfunctioning opcodes when run in test mode; it should only output a single value, the BOOST keycode. What BOOST keycode does it produce?
//
// --- Part Two ---
//
// You now have a complete Intcode computer.
//
// Finally, you can lock on to the Ceres distress signal! You just need to boost your sensors using the BOOST program.
//
// The program runs in sensor boost mode by providing the input instruction the value 2. Once run, it will boost the sensors automatically, but it might take a few seconds to complete the operation on slower hardware. In sensor boost mode, the program will output a single value: the coordinates of the distress signal.
//
// Run the BOOST program in sensor boost mode. What are the coordinates of the distress signal?
//

use std::fs;

fn pad(s: String) -> String {
    if s.len() == 1 {
        format!("0000{}", s)
    } else if s.len() == 2 {
        format!("000{}", s)
    } else if s.len() == 3 {
        format!("00{}", s)
    } else if s.len() == 4 {
        format!("0{}", s)
    } else {
        s
    }
}

fn collect_parameters(s: &str, program: &mut Vec<i64>, position: usize, relative_base: usize) -> Vec<i64> {
    s.chars().rev().enumerate().map(|(i, c)| {
        if c == '0' {   // position mode
            program[program[position + i + 1] as usize]
        } else if c == '2' {
            program[program[position + i + 1] as usize + relative_base]
        } else {        // immediate mode
            program[position + i + 1]
        }
    }).collect()
}

fn position_result(s: &str, program: &mut Vec<i64>, position_r: usize, relative_base: usize) -> usize {
    if s.starts_with("2") {
        program[position_r] as usize + relative_base
    } else {
        program[position_r] as usize
    }
}

fn run(position: usize, program: &mut Vec<i64>, inputs: &mut Vec<i64>, relative_base: usize) {
    println!("run {} {}", position, program.len());
    match program[position].to_string() {
        x if x.ends_with("3") => {
            let input = inputs.pop().expect("No more inputs available");
            let pos = position_result(&x, program, position + 1, relative_base);
            program[pos] = input;
            run(position + 2, program, inputs, relative_base);
        },
        x if x.ends_with("5") => {
            let mut s = pad(x);
            let _ = s.drain(s.len()-2..);

            let parameters = collect_parameters(&s, program, position, relative_base);

            if parameters[0] != 0 {
                run(parameters[1] as usize, program, inputs, relative_base);
            } else {
                run(position + 3, program, inputs, relative_base);
            }
        },
        x if x.ends_with("6") => {
            let mut s = pad(x);
            let _ = s.drain(s.len()-2..);

            let parameters = collect_parameters(&s, program, position, relative_base);

            if parameters[0] == 0 {
                run(parameters[1] as usize, program, inputs, relative_base);
            } else {
                run(position + 3, program, inputs, relative_base);
            }
        },
        x if x.ends_with("7") => {
            let mut s = pad(x);
            let _ = s.drain(s.len()-2..);

            let parameters = collect_parameters(&s, program, position, relative_base);
            let pos = position_result(&s, program, position + 3, relative_base);

            if parameters[0] < parameters[1] {
                program[pos] = 1;
            } else {
                program[pos] = 0;
            }

            run(position + 4, program, inputs, relative_base);
        },
        x if x.ends_with("8") => {
            let mut s = pad(x);
            let _ = s.drain(s.len()-2..);

            let parameters = collect_parameters(&s, program, position, relative_base);
            let pos = position_result(&s, program, position + 3, relative_base);

            if parameters[0] == parameters[1] {
                program[pos] = 1;
            } else {
                program[pos] = 0;
            }

            run(position + 4, program, inputs, relative_base);
        },
        x if x.ends_with("99") => {
            return;
        },
        x if x.ends_with("9") => {
            let mut s = pad(x);
            let _ = s.drain(s.len()-2..);
            let parameters = collect_parameters(&s, program, position, relative_base);
            run(position + 2, program, inputs, relative_base + parameters[0] as usize);
        },
        x if x.ends_with("4") => {
            let s = x.to_string();

            let val = if s.starts_with("0") || s == "4".to_string() {
                program[program[position + 1] as usize]
            } else if s.starts_with("2") {
                program[program[position + 1] as usize + relative_base]
            } else {
                program[position + 1]
            };

            println!("OUTPUT {:?}", val);

            run(position + 2, program, inputs, relative_base);
        },
        x if x.ends_with("1") || x.ends_with("2") => {
            let mut s = pad(x);

            let op: String = s.drain(s.len()-2..).collect();
            let parameters = collect_parameters(&s, program, position, relative_base);

            let a = parameters[0];
            let b = parameters[1];
            let pos = position_result(&s, program, position + 3, relative_base);

            if op == "01" {     // addition
                program[pos] = a + b;
            } else {            // multiplication
                program[pos] = a * b;
            }

            run(position + 4, program, inputs, relative_base);
        },
        _ => {
            panic!("This should not happen");
        },
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut program: Vec<i64> = contents.split(',').map(|n| n.parse::<i64>().unwrap_or(0)).collect();
    program.resize(32768, 0);

    let mut inputs: Vec<i64> = [1].to_vec();

    run(0, &mut program, &mut inputs, 0);
}
