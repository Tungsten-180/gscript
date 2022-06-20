use std::fs;
pub mod gco;
pub mod shapes;
//pub mod extrude;

#[derive(Debug, Clone, Copy)]
pub enum Syntax<T> {
    Settings(Option<T>),
    Probe(Option<T>),
    Travel(Option<T>),
    Extrude(Option<T>),
    Circle(Option<T>),
    Rectangle(Option<T>),
    Err(),
    Null,
}

#[derive(Debug, Clone)]
pub enum Param {
    Z(Option<f64>),
    F(Option<f64>),
    Null,
}

pub enum Extr {
    PlaneExtr,
    PerimeterExtr,
    InfilExtr,
}

struct Command(Syntax<String>, usize);
struct Point(f64, f64);

#[derive(Debug, Clone)]
pub struct Script {
    pub data: Option<String>,
    pub commands: Option<Vec<String>>,
}

impl Scripting for Script {
    fn init_commands(&mut self) {
        self.commands = Some(get_commands_int(self));
        let mut c = 0;
    }

    fn process(&mut self) {
        self.init_commands();
        self.data = None;
    }
}

pub trait Scripting {
    fn new() -> Script {
        Script {
            data: None,
            commands: None,
        }
    }
    fn from_string(input: String) -> Script {
        Script {
            data: Some(input),
            commands: None,
        }
    }
    fn from_file(input: &str) -> Script {
        Script {
            data: Some(import(input)),
            commands: None,
        }
    }
    fn init_commands(&mut self) {}
    fn process(&mut self) {}
}

pub fn to_syntax(command: &str) -> Syntax<&str> {
    let mut command = command.split_ascii_whitespace();
    match command.nth(0).unwrap() {
        "probe" => {
            if &command.clone().count() == &1usize {
                Syntax::Probe(None)
            } else if &command.clone().count() == &2usize {
                Syntax::Probe::<&str>(Some(command.nth(1).unwrap()))
            } else {
                Syntax::Err()
            }
        }
        "move" | "travel" => Syntax::Travel(None),
        "circle" => Syntax::Circle(None),
        "ce" => Syntax::Circle(Some("E")),
        "ce|" => Syntax::Circle(Some("FE")),
        "rect" | "rectange" => Syntax::Rectangle(None),
        "recte" => Syntax::Rectangle(Some("E")),
        "recte|" => Syntax::Rectangle(Some("FE")),
        "extrude" | "ext" => Syntax::Extrude(None),
        "settings" => Syntax::Settings(None),
        _ => Syntax::Err(),
    }
}

fn import(filepath: &str) -> String {
    match read_file_string(filepath) {
        Ok(data) => data,
        Err(_) => panic!(), //String::from("ERR reading file"),
    }
}

pub fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

fn get_commands_int(input: &mut Script) -> Vec<String> {
    let proc = |x| {
        let mut f = String::from(x);
        match x {
            "ENDOFFILE" => {}
            _ => f.push('}'),
        };
        f
    };
    //println!("Getcommands returns::\n\t{:?}\n\n",input.data.as_ref().unwrap().split("}").nth(1));

    let mut output: Vec<String> = input.data.as_ref().unwrap().split("}").map(proc).collect();
    let mut c = 0;
    for x in output.clone().into_iter() {
        if x.contains("ENDOFFILE") == true {
            output.remove(c);
        }
        c += 1;
    }
    output
}

// fn get_commands_int(input: &mut Script) -> Vec<String> {
// let mut c: usize = 0;
// let mut out = Vec::new();
//
// let mut start: usize = 0;
// let mut end: usize = 0;
// let mut old:char = '4';
// for x in input.data.as_ref().unwrap().as_str().chars() {
//   let k :String = old.to_string()+&x.to_string();
// match k.as_str() {
// "}\n" => start = c + 1,
// "\n}" => {
// end = c;
// out.push(command_int(&start, &end, &input.data.as_ref().unwrap()));
// }
// _ => {}
// }
// c += 1;
//   old = x;
// }
//
// out
// }

fn command_int(start: &usize, end: &usize, focus: &String) -> String {
    String::from(focus.get(start.clone()..end.clone()).unwrap().trim())
}

pub mod extrude {
    use std::f64::consts::PI;
    #[derive(Debug)]
    pub struct Settings {
        pub nozzle: f64,
        pub layer_height: f64,
        pub layer_width: f64,
        pub max_extrusion: Option<f64>,
        pub extrusion_mult: f64,
    }

    pub fn line(set: &Settings, x: &f64, y: &f64) -> String {
        //println!("{:?}",set);
        let l = { x - y }.abs();
        let [w, h, n] = [set.layer_width, set.layer_height, set.nozzle];
        let e = ((h * w * l) / (n * n * PI)) * set.extrusion_mult;
        //println!("e{:?}:w{:?}:h{:?}:n{:?}:l{:?}:x{:?}:y{:?}",e,w,h,n,l,x,y);
        format!("G1 X{} Y{} E{}\n", x, y, e)
    }

    pub fn rectangle_perimeter(r: super::shapes::Rectangle, settings: &Settings) -> String {
        let mut output = String::new();
        for corner in r.corners() {
            let [x, y] = corner;
            output.push_str(&line(settings, &x, &y));
        }
        let [x, y] = *r.corners().get(0).unwrap();
        output.push_str(&line(settings, &x, &y));
        output
    }

    pub fn rectangle_plane_by_perimeters(r: super::shapes::Rectangle, settings: &Settings)->String{
        let mut rect = r.clone();
        let mut output = rectangle_perimeter(r, settings);
        

        rect.c1 = [rect.c1[0]-(&settings.layer_width/2.0),rect.c1[1]-(&settings.layer_width/2.0)];
        rect.c2 = [rect.c2[0]-(&settings.layer_width/2.0),rect.c2[1]-(&settings.layer_width/2.0)];

        while &r.min_difference() > &settings.layer_width {
            output.push_str(&rectangle_perimeter(rect, settings));
            rect.c1 = [rect.c1[0]-(&settings.layer_width/2.0),rect.c1[1]-(&settings.layer_width/2.0)];
            rect.c2 = [rect.c2[0]-(&settings.layer_width/2.0),rect.c2[1]-(&settings.layer_width/2.0)];
        }
        output
        
    }
}
