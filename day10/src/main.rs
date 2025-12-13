use get_input::get_input;

fn main() {
    let input = get_input(10)
        .unwrap()
        .lines()
        .map(|l| Machine::from_str(l))
        .collect::<Vec<_>>();

    part1(&input);
    part2_2(&input);
}

fn part1(input: &[Machine]) {
    let mut sum_of_sequences = 0;
    for machine in input {
        let shortest_sequence = machine.compute_shortest_light_sequence();
        sum_of_sequences += shortest_sequence.len();
        println!("Shortest sequence for {:?} is {:?}", machine.required_lights, shortest_sequence);
    }
    println!("Part 1: Sum of shortest paths {}", sum_of_sequences);
}

fn part2(input: &[Machine]) {
    let mut sum_of_sequences = 0;
    for machine in input {
        let shortest_sequence = machine.compute_shortest_joltage_sequence();
        sum_of_sequences += shortest_sequence.len();
        println!("Shortest sequence for {:?} is {:?}", machine.joltages, shortest_sequence.len());
    }
    println!("Part 2: Sum of shortest paths {}", sum_of_sequences);
}

fn part2_2(input: &[Machine]) {
    for m in input {
        compute_shortest_joltage_sequence(m);
    }
}

#[derive(Debug)]
struct Machine {
    required_lights: Vec<bool>,
    buttons: Vec<Button>,
    joltages: Vec<u16>,
}

impl Machine {
    fn from_str(s: &str) -> Self {
        let light_end = s.find(']').unwrap();
        let buttons_end = s.find('{').unwrap();

        let lighting = &s[1..light_end];
        let buttons = &s[(light_end + 2)..(buttons_end - 1)];
        let joltages = &s[(buttons_end + 1)..(s.len() - 1)];

        let required_lights = lighting
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => panic!("Invalid character"),
            })
            .collect();

        let buttons = buttons.split(' ').map(|v| Button::from_str(v)).collect();
        let joltages = joltages.split(',').map(|v| v.parse().unwrap()).collect();
        Self {
            required_lights,
            buttons,
            joltages,
        }
    }

    fn compute_shortest_light_sequence(&self) -> Vec<Button> {
        // Lets try the naive way
        let mut step_states = vec![StepState::new(self.required_lights.len())];

        loop {
            let mut new_step_states = Vec::new();
            for step_state in step_states.drain(..) {
                for i in 0..self.buttons.len() {
                    let mut step_state = step_state.clone();
                    step_state.light_state = self.apply_button(i, step_state.light_state);
                    step_state.buttons_pressed.push(self.buttons[i].clone());
                    if step_state.light_state == self.required_lights {
                        return step_state.buttons_pressed;
                    }
                    new_step_states.push(step_state);
                }
            }
            step_states = new_step_states;
        }
    }

    fn compute_shortest_joltage_sequence(&self) -> Vec<Button> {
        let mut step_states = vec![StepStateJolt::new(self.required_lights.len())];

        loop {
            let mut new_step_states = Vec::new();
            for step_state in step_states.drain(..) {
                for i in 0..self.buttons.len() {
                    let mut step_state = step_state.clone();
                    step_state.jolt_state = self.apply_button_jolt(i, step_state.jolt_state);
                    step_state.buttons_pressed.push(self.buttons[i].clone());
                    if step_state.jolt_state == self.joltages {
                        return step_state.buttons_pressed;
                    }

                    // Since we cannot reverse an increment, if we've blown over any joltages we can reject that subtree
                    if !any_joltage_over(&step_state.jolt_state, &self.joltages) {
                        new_step_states.push(step_state);
                    } else {
                        // println!("OVERJOLTAGE ON {:?} PRUNING", step_state.jolt_state);
                    }
                }
            }
            step_states = new_step_states;
        }
    }

    fn apply_button(&self, button_index: usize, mut light_state: Vec<bool>) -> Vec<bool> {
        let button = &self.buttons[button_index];
        // print!("Applying button {:?} to {:?} -> ", button.switch_set, light_state);
        for index in &button.switch_set {
            let i = *index as usize;
            light_state[i] = !light_state[i];
        }
        // println!("{:?}", light_state);
        light_state
    }

    fn apply_button_jolt(&self, button_index: usize, mut jolt_state: Vec<u16>) -> Vec<u16> {
        let button = &self.buttons[button_index];
        // print!("Applying button {:?} to {:?} -> ", button.switch_set, jolt_state);
        for index in &button.switch_set {
            let i = *index as usize;
            jolt_state[i] += 1;
        }
        // println!("{:?}", jolt_state);
        jolt_state
    }

    fn buttons_sorted_by_size(&self) -> Vec<Vec<Button>> {
        let mut sorted_buttons = self.buttons.clone();
        sorted_buttons.sort_by(|l, r| r.switch_set.len().cmp(&l.switch_set.len()));
        sorted_buttons.chunk_by(|l, r| l.switch_set.len() == r.switch_set.len()).map(|c| c.to_vec()).collect()
    }

    fn buttons_sorted_by_size2(&self) -> Vec<Button> {
        let mut sorted_buttons = self.buttons.clone();
        sorted_buttons.sort_by(|l, r| r.switch_set.len().cmp(&l.switch_set.len()));
        sorted_buttons
    }
}

#[derive(Debug, Clone)]
struct Button {
    switch_set: Vec<u8>,
}

impl Button {
    fn from_str(s: &str) -> Self {
        // expecting (##,##,...)
        let content = &s[1..s.len() - 1];
        Self {
            switch_set: content.split(',').map(|v| v.parse().unwrap()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct StepState {
    buttons_pressed: Vec<Button>,
    light_state: Vec<bool>,
}

impl StepState {
    fn new(light_count: usize) -> Self {
        Self {
            buttons_pressed: Vec::new(),
            light_state: vec![false; light_count],
        }
    }
}

#[derive(Debug, Clone)]
struct StepStateJolt {
    buttons_pressed: Vec<Button>,
    jolt_state: Vec<u16>,
}

impl StepStateJolt {
    fn new(light_count: usize) -> Self {
        Self {
            buttons_pressed: Vec::new(),
            jolt_state: vec![0; light_count],
        }
    }
}

fn any_joltage_over(joltages: &[u16], target: &[u16]) -> bool {
    for (j, t) in joltages.iter().zip(target.iter()) {
        if j > t {
            return true;
        }
    }
    false
}

fn compute_shortest_joltage_sequence(m: &Machine) {
    let mut jolt_state = vec![0; m.joltages.len()];
    // Start with the widest button
    let mut sorted_buttons = m.buttons_sorted_by_size2();
    let b = sorted_buttons.pop().unwrap();
    // apply the button as much as it can
    print!("Applying button {:?} maximum times for target {:?} -> ", b, m.joltages);
    
    let step_states = vec![StepStateJolt::new(m.joltages.len())];
    while let Some(b) = sorted_buttons.pop() {
        for 
        jolt_state = apply_button_maximally(jolt_state, &m.joltages, &b);
        println!("{:?}", jolt_state);
    }
}

fn apply_button_maximally(mut jolt_state: Vec<u16>, target_jolt_state: &Vec<u16>, button: &Button) -> Vec<u16> {
    while !jolt_state_has_any_match(&jolt_state, target_jolt_state) {
        jolt_state = apply_button(button, jolt_state);
    }
    jolt_state
}

fn apply_button(button: &Button, mut jolt_state: Vec<u16>) -> Vec<u16> {
    for index in &button.switch_set {
        jolt_state[*index as usize] += 1;
    }
    jolt_state
}

fn jolt_state_has_any_match(jolt_state: &Vec<u16>, target_jolt_state: &Vec<u16>) -> bool {
    jolt_state.iter().zip(target_jolt_state.iter()).any(|(l, r)| l == r)
}

// Thoughts on part 2:

// Start with the button with the most coverage, apply it until we hit one target


// ex 1. target the lowest joltage (3)
// We can apply button (0,2) or (0,1) 3 times lets pick (0,2) (we want to apply the widest button first)
// we get (3,0,3,0)
// now target the next lowest joltage (4)
// we can apply button (2), (2,3), (0,2) 1 times but since we exhausted 0 already, we cannot use (0,2) we pick (2,3)
// we get (3,0,4,1)
// Next lowest joltage is 5
// We can apply button (1,3), (0,1) 5 times, but we we must exclude buttons that contain 0 or 2 so we use (1,3)
// we get (3,5,4,6)
// Now since we are lucky enough to have the button (3) available we push it once
// we get (3,5,4,7)

// With this train of thought we can pigeonhole ourselves to not having a button available

// We can think of this like factors
// How do we decompose the final product into the lowest number of factors possible
// There is no analytical answer since there are multiple lowest answers

// The lowest width button sort of determines the last buttons we should push
// so in our example 1 the lowest width buttons are (3) and (2). The highest joltage associated is luckily
// (3) -> 7 but if it wasn't we would take the max between the narrowest buttons

// It feels like the make the lowest amount of change problem but our denominations are interrelated
// its like if a penny was mandatorily glued to a nickel in some cases

// And we have ambiguity between the coinage hierarchy
// So we have multiple starting points

// So how about this
// We start by applying the largest button as many times as possible
// Then we pop that button off
// Then we try the next largest button as many times as possible
// Then we pop that button off

// It like a tree but the sub trees are explored in order from largest button to smallest
// And we start with n trees where n is the number of buttons with the max width
