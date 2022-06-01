#[derive(Debug)]
pub struct Point {
    x: Data,
    y: Data,
    z: Data,
}
#[derive(Debug)]
pub enum Feild {
    Arc(Point, Point, Point),
    Coord(Point),
    Null,
}
#[derive(Debug)]
pub enum Data {
    Celcius(u8),
    Mili(usize),
    Sec(usize),
    Speed(u8),
    Disp(f64),
    Message(String),
    Null,
}
#[derive(Debug)]
pub enum C {
    G1,  //move
    G2,  //cw arc
    G3,  //ccw arc
    G4,  //pause
    G21, //units mm
    G28, //home axis
    G30, //probe
    G90, //abs coords
    G91, //rel coords

    M104, //extruder temp
    M105, //read current temp
    M106, //fan speed
    M109, //wait for extr temp
    M114, //disp curr pos
    M82,  //abs E codes
    M83,  //rel E codes
    M84,  //disable steppers
    M117, //write message to lcd

    Invalid,
    Null,
}

pub fn gcode_to_str(input: C) -> String {
    String::from(match input {
        C::G1 => "G1",     //move
        C::G2 => "G2",     //cw arc
        C::G3 => "G3",     //ccw arc
        C::G4 => "G4",     //pause
        C::G21 => "G21",   //units mm
        C::G28 => "G28",   //home axis
        C::G30 => "G30",   //probe command
        C::G90 => "G90",   //abs coords
        C::G91 => "G91",   //rel coords
        C::M104 => "M104", //ext temp
        C::M105 => "M105", //read current temp
        C::M106 => "M106", //fan speed
        C::M109 => "M109", //wait for extr temp
        C::M114 => "M114", //disp curr pos
        C::M82 => "M82",   //abs E codes
        C::M83 => "M83",   //rel E codes
        C::M84 => "M84",   //disable steppers
        C::M117 => "M117", //write message to lcd
        _ => "Err",
    })
}

pub fn into_gcode_internal(input: &str) -> C {
    match input {
        "G1" => C::G1,   //move
        "G2" => C::G2,   //cw arc
        "G3" => C::G3,   //ccw arc
        "G4" => C::G4,   //pause
        "G21" => C::G21, //units mm
        "G28" => C::G28, //home axis
        "G30" => C::G30, //probe command
        "G90" => C::G90, //abs coords
        "G91" => C::G91, //rel coords

        "M104" => C::M104, //ext temp
        "M105" => C::M105, //read current temp
        "M106" => C::M106, //fan speed
        "M109" => C::M109, //wait for extr temp
        "M114" => C::M114, //disp curr pos
        "M82" => C::M82,   //abs E codes
        "M83" => C::M83,   //rel E codes
        "M84" => C::M84,   //disable steppers
        "M117" => C::M117, //write message to lcd
        arg => C::Invalid,
    }
}

pub trait Code {
    fn into_gcode(&self) -> C {
        C::Null
    }
}
