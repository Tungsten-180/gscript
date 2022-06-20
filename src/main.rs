mod gscript_lib;
use gscript_lib::{Param, Script, Scripting, Syntax};
use std::fs::File;
use std::io::Write;
use gscript_lib::{extrude as extrude,shapes as shapes};
use extrude::line;

#[derive(Debug)]
struct Consts<T> {
    z: Option<T>,
    f: Option<T>,
}

#[derive(Debug)]
enum Var<T: Sized> {
    Const(Consts<T>),
    List((Param, Vec<T>)),
}
trait Unwrap<U, T: Sized> {
    fn unwrap_const(&self) -> T;
    fn unwrap_list(&self) -> U;
}
impl Unwrap<(Param, Vec<String>), String> for Var<String> {
    fn unwrap_const(&self) -> String {
        let x = match self {
            Var::Const(some) => some,
            Var::List(_) => panic!(),
        };
        //println!("x::{:?}",&self);
        let s = match &x.z {
            Some(strig) => strig.clone(),
            None => String::new(),
        } + match &x.f {
            Some(strig) => strig,
            None => "",
        };
        //println!("S:{}",s);
        s
    }
    fn unwrap_list(&self) -> (Param, Vec<String>) {
        match self {
            Var::List(some) => some.clone(),
            Var::Const(_) => panic!(),
        }
    }
}

fn prange(start: &str, end: &str, focus: &String) -> String {
    //println!("focus::{:?}", focus);
    let mut out = String::from(
        focus
            .as_str()
            .split_once(start)
            .unwrap()
            .1
            .split_once(end)
            .unwrap()
            .0,
    );
    out
}

// fn com_match(com:Syntax)->&str{}

fn consts(input: &String) -> Var<String> {
    //println!(" consts input string::\t{:?}",input);
    let mut cs: Consts<String> = Consts { z: None, f: None };
    let u = |z: &String| {
        let o = prange("(", ")", z);
        let o = o.as_str().split("const:").nth(1).unwrap();
        if &o.find(",") != &None {
            o.split(";")
                .nth(0)
                .unwrap()
                .split(",")
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        } else {
            vec![o.split(";").nth(0).unwrap().to_string()]
        }
    };
    //println!("u::{:?}",u(input));
    for constant in u(input).into_iter() {
        match constant.get(0..1).unwrap().to_ascii_lowercase().as_str() {
            "" => {
                //println!("const::{:?}", constant.get(0..1).unwrap());
                continue;
            }
            "z" => {
                //println!("const::{:?}", constant.get(0..1).unwrap());
                cs.z = Some(constant)
            }
            "f" => {
                //println!("const::{:?}", constant.get(0..1).unwrap());
                cs.f = Some(constant)
            }
            _ => {}
        };
    }
    Var::Const(cs)
}

fn points(input: &String) -> Vec<String> {
    prange("{", ";}", input)
        .as_str()
        .split(";")
        .map(|x| x.to_string())
        .collect()
}

fn run_settings(command: &String, settings: &mut extrude::Settings) {
    for point in points(command) {
        //println!("{:?}",&point);
        match point
            .split(":")
            .nth(0)
            .unwrap()
            .to_ascii_lowercase()
            .as_str()
        {
            "w" => settings.layer_width = point.split(":").nth(1).unwrap().parse().unwrap(),
            "h" => settings.layer_height = point.split(":").nth(1).unwrap().parse().unwrap(),
            "n" => settings.nozzle = point.split(":").nth(1).unwrap().parse().unwrap(),
            "m" => settings.extrusion_mult = point.split(":").nth(1).unwrap().parse().unwrap(),
            &_ => {
                panic!()
            }
        }
    }
}

fn run_probe(command: &String, output: &mut String) {
    for point in points(&command) {
        let [x, y] = [
            point.split(",").nth(0).unwrap(),
            point.split(",").nth(1).unwrap(),
        ];
        output.push_str(&format!("G1 X{} Y{}\nG30\n", x, y))
    }
}

fn run_travel(command: &String, output: &mut String) {
    for point in points(&command) {
        let [x, y] = [
            point.split(",").nth(0).unwrap(),
            point.split(",").nth(1).unwrap(),
        ];
        output.push_str(&format!("G1 X{} Y{}\n", x, y))
    }
}

fn run_circle(
    command: &String,
    output: &mut String,
    settings: &mut extrude::Settings,
    val: Option<&str>,
) {
    let mut c = shapes::Circle {
        o: [0.0; 2],
        r: 0.0,
    };
    for point in points(&command) {
        if point.contains("o:") {
            c.o = [
                point
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .nth(0)
                    .unwrap()
                    .parse::<f64>()
                    .unwrap(),
                point
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .nth(1)
                    .unwrap()
                    .parse::<f64>()
                    .unwrap(),
            ];
        } else if point.contains("r:") {
            c.r = point.split(":").nth(1).unwrap().parse::<f64>().unwrap();
        } else if point.matches(":").collect::<Vec<&str>>().len() == 1 {
            c.o = [
                point
                    .split(":")
                    .nth(0)
                    .unwrap()
                    .split(",")
                    .nth(0)
                    .unwrap()
                    .parse::<f64>()
                    .unwrap(),
                point
                    .split(":")
                    .nth(0)
                    .unwrap()
                    .split(",")
                    .nth(1)
                    .unwrap()
                    .parse::<f64>()
                    .unwrap(),
            ];
            c.r = point.split(":").nth(1).unwrap().parse::<f64>().unwrap();
        }
    }
    if val == None {
        for step in c.steps() {
            let [x, y] = step;
            output.push_str(&format!("G1 X{} Y{}\n", x, y))
        }
    } else if val == Some("E") {
        for step in c.steps() {
            let [x, y] = step;
            output.push_str(&line(&settings, &x, &y));
        }
    }
}

fn run_rectangle(
    command: &String,
    output: &mut String,
    settings: &mut extrude::Settings,
    val: Option<&str>,
) {
    let mut R = shapes::Rectangle {
        c1: [0.0; 2],
        c2: [0.0; 2],
    };
    for point in points(&command) {
        let mut point = point.split(":");
        //println!("{:?}",&point.clone().collect::<Vec<&str>>());
        R.c1 = [
            *&point
                .clone()
                .nth(0)
                .unwrap()
                .split(",")
                .nth(0)
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            *&point
                .clone()
                .nth(0)
                .unwrap()
                .split(",")
                .nth(1)
                .unwrap()
                .parse::<f64>()
                .unwrap(),
        ];
        //println!("{:?}",&point.clone().collect::<Vec<&str>>());
        R.c2 = [
            *&point
                .clone()
                .nth(1)
                .unwrap()
                .split(",")
                .nth(0)
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            *&point
                .clone()
                .nth(1)
                .unwrap()
                .split(",")
                .nth(1)
                .unwrap()
                .parse::<f64>()
                .unwrap(),
        ];
        if val == None {
            for corner in R.corners() {
                let [x, y] = corner;
                output.push_str(&format!("G1 X{} Y{}\n", x, y));
            }
            let [x, y] = *R.corners().get(0).unwrap();
            output.push_str(&format!("G1 X{} Y{}\n", x, y));
        }
        if val == Some("E") {
            for corner in R.corners() {
                let [x, y] = corner;
                output.push_str(&line(&settings, &x, &y));
            }
            let [x, y] = *R.corners().get(0).unwrap();
            output.push_str(&line(&settings, &x, &y));
        }
        if val == Some("FE"){
             
        }
    }
}

fn main() {
    let mut input = Script::from_file("in.gscript");

    //println!("{:?}",input);

    input.process();

    let mut output: String = String::from("G28;home\nG90;abs axis\nG83;rel e\n");

    let mut settings = extrude::Settings {
        nozzle: 0.0,
        layer_width: 0.0,
        layer_height: 0.0,
        extrusion_mult: 0.0,
        max_extrusion: None,
    };

    for command in input.commands.unwrap() {
        let command: String = command.split_ascii_whitespace().collect();
        //cleans whitespace ^

        if command.contains("const:") {
            let x: String = consts(&command).unwrap_const();
            output.push_str(&format!("G1 {}\n", x));
            //   ^prints constant gcode for this command(ie. initial layer height)
        }
        println!("Command:{:?}", &command);
        let ccom = prange("", "(", &command);

        let com = gscript_lib::to_syntax(&ccom);
        // gets syntax ^

        match com {
            Syntax::Settings(_) => {
                run_settings(&command, &mut settings);
            }
            Syntax::Probe(_) => {
                run_probe(&command, &mut output);
            }
            Syntax::Travel(_) => {
                run_travel(&command, &mut output);
            }
            Syntax::Circle(val) => {
                run_circle(&command, &mut output, &mut settings, val);
            }
            Syntax::Rectangle(val) => run_rectangle(&command, &mut output, &mut settings, val),

            Syntax::Err() => {}
            Syntax::Null => {}
            Syntax::Extrude(_) => {}
        }
    }
    output.push_str("G28");
    let mut outfile = File::create("out.gcode").unwrap();

    write!(outfile, "{}", output);
}
