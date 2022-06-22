# gscript - DO NOT USE!!
A custom script interpreter for writing gcode more easily but providing more control than in a slicer.

Currently Supports the following commands:

Circle(){}
Rectangle(){}
Probe(){}
Travel(){}

Requires a:
Settings(){}

Also supports extrusion for Rectangles and Circles through:
recte(){}

and 

ce(){}

_________________________________________
Example                                   Explanations

Settings(){                                Manadatory settings command- can be changed at any point by another settings command
  n:0.4;                                   Nozzle diameter in mm(Code is unitless but default gcode is mm)
  w:.4;                                    Line Width
  h:0.2;                                   Layer Height
  m:1;                                     Extrusion Multiplier- 1 is 100% 0 is 0%(Code assumes perfect cyinder from nozzle squishes into perfect rectangle so play with this)
}
Travel (const:Z.2,f400;){                 Supported Constants are Z(the Z axis) and F(speed). Must be seperated by comma if two, end with a ";"
  0,0;                                    Points. Line must end with ";" and be in x,y format.
  1,1;                                    |
  20,3.5;                                 |
  50,50;                                  |
}
Probe(){
  0,0;                                    Points. Line must end with ";" and be in x,y format.
  1,1;                                    |
  20,3.5;                                 |
  50,50;                                  |
}
Rectangle(){                              Constants supported, not mandatory---- ONLY RECTANGULAR TRAVEL
  0,0:70,20;                              format is corner1:corner2;      where corner1 and 2 are opposite and given in x,y form.  
}
recte(){                                     Rectangular Extrusion- will EXTRUDE outline of rectangle
  0,0:70,20;
}
Circle(){                                 Circle outline travel-- DOES NOT EXTRUDE
  o:10,5                                  Center point for circle
  r:25;                                   Radius of circle
}
ce(){                                     Circle Extrusion- extrudes outline of circle
 10,5:25;                                 Circles and Circle Extrusion support both "o:point \n r:value" syntax and "point:radius;" syntax
}

ENDOFFILE                                 Must have ENDOFFILE Declaration.
