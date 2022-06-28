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

fn command_int(start: &usize, end: &usize, focus: &String) -> String {
    String::from(focus.get(start.clone()..end.clone()).unwrap().trim())
}

pub mod extrude {
    use std::arch::x86_64::_SIDD_LEAST_SIGNIFICANT;
    use std::f64::consts::PI;

    use crate::gscript_lib::extrude;
    use crate::shapes::Shape;

    use super::Extr;

    #[derive(Debug)]
    pub struct Settings {
        pub filament_diameter: f64,
        pub layer_height: f64,
        pub layer_width: f64,
        pub max_extrusion: Option<f64>,
        pub extrusion_mult: f64,
    }

    pub fn extrude(set: &Settings, nextpoint: &[f64; 2], lastpoint: &Option<[f64; 2]>) -> String {
        //println!("{:?}",set);
        let [x, y] = nextpoint;
        if lastpoint != &None {
            let [w, h, f] = [set.layer_width, set.layer_height, set.filament_diameter];
            let l = line_len(nextpoint, lastpoint);
            let line = h * w * l;
            let f = (f / 2.0).powi(2);
            let circle = PI * f;

            let e = (line / circle) * set.extrusion_mult;

            format!("G1 X{:.3} Y{:.3} E{:.5}\n", x, y, e)
        } else {
            format!("G1 X{:.3} Y{:.3}\n", x, y)
        }
    }

    pub fn line_len(nextpoint: &[f64; 2], lastpoint: &Option<[f64; 2]>) -> f64 {
        let r = super::shapes::Rectangle::from(*nextpoint, lastpoint.unwrap());
        r.sideln(1).hypot(r.sideln(0))
    }

    pub fn circle_perimeter(circle: &super::shapes::Circle, settings: &Settings) -> String {
        let mut output = String::new();
        let mut laststep: Option<[f64; 2]> = None;
        for step in circle.steps() {
            // let [x, y] = step;
            output.push_str(&extrude(&settings, &step, &laststep));
            laststep = Some(step);
        }
        output.push_str(&extrude(&settings, &circle.steps().remove(0), &laststep));
        output
    }

    pub fn circle_plane_by_perimeter(
        circle: &super::shapes::Circle,
        settings: &Settings,
    ) -> String {
        let mut output = String::new();
        let mut circ = *circle;
        while circ.r > settings.layer_width {
            output.push_str(&circle_perimeter(&circ, settings));
            circ.r -= settings.layer_width;
        }

        output
    }

    pub fn rectangle_perimeter(r: &super::shapes::Rectangle, settings: &Settings) -> String {
        let mut output = String::new();
        let mut lastcorner: Option<[f64; 2]> = None;
        for corner in r.corners() {
            output.push_str(&extrude(&settings, &corner, &lastcorner));
            lastcorner = Some(corner);
        }
        output.push_str(&extrude(&settings, &r.corners()[0], &lastcorner));
        output
    }

    pub fn rectangle_plane_by_perimeters(
        r: super::shapes::Rectangle,
        settings: &Settings,
    ) -> String {
        let mut rect = r;
        let mut output = rectangle_perimeter(&r, &settings);

        println!("rect:{:?}", &rect.min_difference());
        while &rect.min_difference() > &settings.layer_width {
            let [rx, ry] = &rect.c1;
            let [rxx, ryy] = &rect.c2;

            output.push_str(&rectangle_perimeter(&rect, &settings));
            rect.c1 = [rx - settings.layer_width, ry - settings.layer_width];
            rect.c2 = [rxx + settings.layer_width, ryy + settings.layer_width];
        }

        output
    }

    pub fn rectangle_plane_by_diagonal(
        r: &super::shapes::Rectangle,
        settings: &Settings,
    ) -> String {
        let mut output: String = String::from(rectangle_perimeter(r, settings));
        let mut rect = *r;
        let mut opposite_start = true;

        let mut count = 0;
        //long side first short second

        // while count < 200{

        if r.height() != r.width() {
            match rect.height() > rect.width() {
                true => rect.c1[1] = rect.c2[1] + rect.width(),
                false => rect.c2[0] = rect.c1[0] + rect.height(),
            }
        }
        output.push_str(&extrude(settings, &rect.c2, &None));

        while rect.diagonal() > settings.layer_width {
            println!("1::{:?}", rect);
            if opposite_start == false {
                output.push_str(&extrude(settings, &rect.c2, &Some(rect.c1)));
            } else {
                output.push_str(&extrude(settings, &rect.c1, &Some(rect.c2)));
            }

            rect.c1[1] = rect.c1[1] - settings.layer_width;
            rect.c2[0] = rect.c2[0] + settings.layer_width;

            if opposite_start == false {
                output.push_str(&extrude(settings, &rect.c2, &None));
            } else {
                output.push_str(&extrude(settings, &rect.c1, &None));
            }

            match opposite_start {
                true => opposite_start = false,
                false => opposite_start = true,
            }
            count += 1;
        }
        rect = *r;
        if r.height() != r.width() {
            // rect.c1[0] = rect.c1[0]+settings.layer_width;
            // rect.c2[1] = rect.c2[1]-settings.layer_width;

            let mut condition: (usize, f64) = (22, 0.0);
            match r.height() > r.width() {
                false => {
                    // rect.c2[0] = rect.c2[0];
                    rect.c1[0] = rect.c2[0] + rect.height();
                    condition = (0usize, r.c1[0]);
                }
                true => {
                    // rect.c2[1] = rect.c2[1] + rect.height();
                    rect.c1[1] = rect.c2[1] + rect.width();
                    condition = (1usize, r.c1[1])
                }
            }

            // while count < 20 {
            // println!("{:?}",rect);
            while rect.c1[condition.0] < condition.1 {
                println!("2::{:?}", rect);
                if opposite_start == true {
                    output.push_str(&extrude(settings, &rect.c2, &Some(rect.c1)));
                } else {
                    output.push_str(&extrude(settings, &rect.c1, &Some(rect.c2)));
                }

                rect.c1[condition.0] += settings.layer_width;
                rect.c2[condition.0] += settings.layer_width;

                if opposite_start == true {
                    output.push_str(&extrude(settings, &rect.c2, &None));
                } else {
                    output.push_str(&extrude(settings, &rect.c1, &None));
                }

                match opposite_start {
                    true => opposite_start = false,
                    false => opposite_start = true,
                }
                // count += 1;
            }
        }

        rect = *r;
        if r.height() != r.width() {
            match rect.height() > rect.width() {
                true => rect.c2[1] = rect.c1[1] - rect.width(),
                false => rect.c1[0] = rect.c2[0] - rect.height(),
            }
        }

        rect.c1[0] = rect.c1[0] - settings.layer_width;
        rect.c2[1] = rect.c2[1] + settings.layer_width;

        // while count < 20{
        while rect.diagonal() > settings.layer_width {
            println!("3::{:?}", rect);
            if opposite_start == true {
                output.push_str(&extrude(settings, &rect.c2, &Some(rect.c1)));
            } else {
                output.push_str(&extrude(settings, &rect.c1, &Some(rect.c2)));
            }
            rect.c1[0] = rect.c1[0] - settings.layer_width;
            rect.c2[1] = rect.c2[1] + settings.layer_width;
            if opposite_start == true {
                output.push_str(&extrude(settings, &rect.c2, &None));
            } else {
                output.push_str(&extrude(settings, &rect.c1, &None));
            }

            match opposite_start {
                true => opposite_start = false,
                false => opposite_start = true,
            }
            // count += 1;
        }

        output
    }
}
