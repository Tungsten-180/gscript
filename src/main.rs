mod gco;
use gco::C;
mod gscript_lib;
use gscript_lib::{Param, Script, Scripting, Syntax};
use std::fs::File;
use std::io::Write;

impl Keys for Syntax {
    fn to_gco(&self) -> C {
        match self {
            Syntax::Probe => C::G30,
            Syntax::Travel => C::G1,
            other => C::Invalid,
        }
    }
}

impl gco::Code for str {
    fn into_gcode(&self) -> gco::C {
        gco::into_gcode_internal(self)
    }
}

trait Keys {
    fn to_gco(&self) -> C {
        C::Null
    }
}
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Line(C, Option<[Param; 2]>, Option<Vec<[f64; 2]>>);
///////////////////////////////////////////////////////////////////////////////

fn script_lines(input: &mut Script) -> Vec<Line> {
    let mut file_out: Vec<Line> = Vec::new();

    for x in input.commands.clone().as_mut().unwrap() {
        let mut temp = Line(
            x.0.to_gco(),
            Some(input.removeparameters().clone()),
            Some({
                let k = input.removepoints();
                k.to_vec()
            }),
        );
        file_out.push(temp);
    }
    file_out
}

fn lines_to_string(lines: Vec<Line>) -> String {
    let mut out = String::from("G28\n");

    for l in lines {
        let mut temp = String::new();
        let g = gco::gcode_to_str(l.0);
        temp.push_str(&g);
        let mut fo = String::from("");
        
        //pushes params to string if there
        match l.1 {
            Some(val) => {
                for u in val {
                    match u {
                        Param::Z(Some(z)) => {
                            temp.push_str(" Z");
                            temp.push_str(&z.to_string());
                        }
                        Param::F(Some(f)) => {
                            temp.push_str(" F");
                            temp.push_str(&f.to_string());
                            fo.push_str(format!("F{}",&f.to_string()).as_str());
                        }
                        Param::Null | Param::Z(None) | Param::F(None) => {}
                    }
                }
            }
            None => {}
        }//end of param push
        temp.push('\n');
        out.push_str(&temp);
        for o in l.2.unwrap(){
          let [x,y] = o;
          out.push_str(format!("{} X{} Y{}",g,x,y).as_str());
          if fo.as_str() != ""{out.push_str(format!("F{}\n",fo).as_str());}
          else{out.push('\n');}
        }
    }
    out
}

fn probe_clean(input:String)->String{
  let mut out = String::new();
  for x in input.as_str().split('\n'){
    println!("{:?}",&x);
    if x.len() > 3{
      println!("{}",&x[0..3]);
    if &x[0..3] == "G30" {out.push_str(&format!("G1 {}\nG30\n",&x[4..]));}
    else{out.push_str(&format!("{}\n",&x))}
    }else{out.push_str(&format!("{}\n",&x))}
  }
  out
}

fn main() {
    let mut script: Script = Script::from_file("in.gscript");

    script.process();

    let lines = script_lines(&mut script);

    println!("Output::\n{:?}", lines);

    let mut output = File::create("out.gcode").unwrap();

    write!(output,"{}",probe_clean(lines_to_string(lines))).unwrap();

  }
