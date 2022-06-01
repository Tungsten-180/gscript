use std::fs;

#[derive(Debug, Clone, Copy)]
pub enum Syntax {
    Probe,
    Travel,
    Err(),
    Null,
}

#[derive(Debug, Clone)]
pub enum Param {
    Z(Option<f64>),
    F(Option<f64>),
    Null,
}

#[derive(Debug, Clone)]
pub struct Script {
    pub data: Option<String>,
    pub commands: Option<Vec<(Syntax, usize)>>,
    pub parameters: Option<Vec<[Param; 2]>>,
    pub points: Option<Vec<Vec<[f64; 2]>>>,
}

impl Scripting for Script {
    fn init_commands(&mut self) {
        self.commands = Some(get_commands_int(self));
    }
    fn init_params(&mut self) {
        self.parameters = Some(Vec::new());
        for x in self.commands.as_ref().unwrap() {
            let mut temp = self.parameters.clone().unwrap();
            temp.push(get_params_int(&x.1, &self.data.as_ref().unwrap()));
            self.parameters = Some(temp);
        }
    }
    fn init_points(&mut self) {
        self.points = Some(Vec::new());
        for x in self.commands.as_ref().unwrap() {
            let mut k = self.points.as_ref().unwrap().clone();
            k.push(get_points_int(&x.1, &self.data.as_ref().unwrap()));
            self.points = Some(k);
        }
    }

    fn process(&mut self) {
        self.init_commands();
        self.init_params();
        self.init_points();
    }
    fn removepoints(&mut self) -> Vec<[f64; 2]> {
        let mut k = self.points.as_ref().unwrap().clone();
        let out = k.remove(0);
        self.points = Some(k);
        out
    }
    fn removeparameters(&mut self) -> [Param; 2] {
        let mut k = self.parameters.as_ref().unwrap().clone();
        let out = &k.remove(0);
        self.parameters = Some(k);
        out.clone()
    }
}

pub trait Scripting {
    fn new() -> Script {
        Script {
            data: None,
            commands: None,
            parameters: None,
            points: None,
        }
    }
    fn from_string(input: String) -> Script {
        Script {
            data: Some(input),
            commands: None,
            parameters: None,
            points: None,
        }
    }
    fn from_file(input: &str) -> Script {
        Script {
            data: Some(import(input)),
            commands: None,
            parameters: None,
            points: None,
        }
    }
    fn init_commands(&mut self) {}
    fn init_params(&mut self) {}
    fn init_points(&mut self) {}
    fn process(&mut self) {}
    fn removepoints(&mut self) -> Vec<[f64; 2]> {
        vec![[0.0, 0.0]]
    }
    fn removeparameters(&mut self) -> [Param; 2] {
        [Param::Null, Param::Null]
    }
}

pub fn to_syntax(command: &str) -> Syntax {
    match command {
        "probe" => Syntax::Probe,
        "move" => Syntax::Travel,
        _ => Syntax::Err(),
    }
}

fn import(filepath: &str) -> String {
    match read_file_string(filepath) {
        Ok(data) => data,
        Err(_) => String::new(),
    }
}

pub fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

fn get_commands_int(input: &mut Script) -> Vec<(Syntax, usize)> {
    let mut c: usize = 0;
    let mut out = Vec::new();

    let mut start: usize = 0;
    let mut end: usize = 0;

    for x in input.data.as_ref().unwrap().as_str().chars() {
        match x {
            '}' => start = c + 1,
            '(' => {
                end = c;
                out.push(command_int(&start, &end, &input.data.as_ref().unwrap()));
            }
            _ => {}
        }
        c += 1;
    }

    out
}

fn command_int(start: &usize, end: &usize, focus: &String) -> (Syntax, usize) {
    (
        to_syntax(match focus.get(start.clone()..end.clone()) {
            Some(res) => res.trim(),
            None => "",
        }),
        *end,
    )
}

pub fn get_params_int(start: &usize, focus: &String) -> [Param; 2] {
    //Vec<Param> {
    let mut index = start.clone();
    let mut outcount = 0;
    //let mut out: Vec<Param> = Vec::new();
    let mut out: [Param; 2] = [Param::Z(None), Param::F(None)];
    //loop returns [char;7]
    loop {
        let x = focus.as_bytes()[index];
        if x == b')' {
            break;
        } else if x.to_ascii_lowercase() == b'z' {
            let mut temp = ['0'; 7];
            let mut delta: usize = 1;
            loop {
                let x = focus.as_bytes()[index + delta] as char;
                if x == ')' || x.is_ascii_whitespace() || delta > 7 {
                    //println!("{:?}",delta);
                    temp[delta - 1] = '.';
                    break;
                } else if x.is_ascii_digit() || x.is_ascii_punctuation() {
                    temp[delta - 1] = x;
                    delta += 1;
                }
            }
            println!("{:?}", temp);
            outcount += 1;
            out[0] = Param::Z(Some(
                //out.push(Param::Z(Some(
                temp.iter().collect::<String>().parse::<f64>().unwrap(),
            )); //);
        } else if x.to_ascii_lowercase() == b'f' {
            let mut temp = ['0'; 7];
            let mut delta: usize = 1;
            loop {
                let x = focus.as_bytes()[index + delta] as char;
                if delta > 5 || x == ')' || x.is_ascii_whitespace() {
                    //println!("{:?}",delta);
                    temp[delta - 1] = '.';
                    break;
                } else if x.is_ascii_digit() || x.is_ascii_punctuation() {
                    temp[delta - 1] = x;
                    delta += 1;
                }
            }
            println!("{:?}", temp);
            outcount -= 2;
            out[1] = Param::F(Some(
                //out.push(Param::F(Some(
                temp.iter().collect::<String>().parse::<f64>().unwrap(),
            )); //);
        };
        index += 1;
    }
    // match outcount{
    //     -1=>{},
    //     -2=>out[0] = {let tempint = out.pop().unwrap();out.push(Param::Z(None));out.push(tempint)},
    //     1=>out[1] = out.push(Param::F(None)),
    //     0=>out = {out.push(Param::Z(None));out.push(Param::F(None))}
    //     _=>{},
    // }
    out
}

fn get_points_int(start: &usize, focus: &String) -> Vec<[f64; 2]> {
    let mut index: usize = 0;
    let mut index2: usize = 0;
    loop {
        //let x = focus.chars().nth(start+index);
        if { |x| focus.chars().nth(start + x) }(index).unwrap() == '{' {
            break;
        } else {
            index += 1
        }
    }
    loop {
        if { |x| focus.chars().nth(start + index + x) }(index2).unwrap() == '}' {
            break;
        } else {
            index2 += 1
        }
    }

    let mut out: Vec<[f64; 2]> = Vec::new();
    let it = focus
        .as_str()
        .split_at(start + index + 1)
        .1
        .split_at(index2 - 1)
        .0
        .split(';');
    for x in it {
        let mut array = [0f64, 0.0];
        array[0] = x
            .trim()
            .split(',')
            .nth(0)
            .unwrap()
            .parse()
            .expect("parse float error(Remove semicolon from last point)");
        array[1] = x
            .trim()
            .split(',')
            .nth(1)
            .unwrap()
            .parse()
            .expect("parse float error(Remove semicolon from last point)");
        out.push(array);
    }
    out
}
