
# gscript.Beta
<h3>A custom script interpreter for writing gcode more easily but providing more control than in a slicer.</h4>
<br>

Currently Supports the following commands:
<code>
Circle(){}
Rectangle(){}
Probe(){}
Travel(){}
</code>
Requires:
<code>Settings(){}</code>

Also supports extrusion for Rectangles and Circles through:
<code>recte(){}</code>
and 
<code>ce(){}</code>

___




| Syntax | Explanation |
|--|--|
|<code>Settings(){<br>  n:0.4;<br>w:0.4;<br>h:0.2;<br>m:1.0;<br>}|Mandatory settings command- can be changed at any point by another settings command<br>Nozzle diameter in mm(Code is unitless but default gcode is mm)<br> Line Width<br>      Layer Height<br>           Extrusion Multiplier- 1 is 100% 0 is 0%(Code assumes perfect cylinder from nozzle squishes into perfect rectangle so play with this)|
|<code>Travel (const:Z.2,f400;){<br>  0,0;    <br> 1,1;       <br>  20,3.5;<br>  50,50;<br>}|<expl>Supported Constants are Z(the Z axis) and F(speed).<br> Must be separated by comma if two, end with a ";"<br> <br> Points. Line must end with ";" and be in x,y format.</expl><br><br><br>-|
|<code>Probe(){<br>0,0;<br>1,1;<br>  20,3.5;<br>  50,50;  <br>}|Points. Line must end with <code>;</code> and be in <code>x,y</code> format.
|<code>Rectangle(){<br>0,0:70,20;<br>}</code>|Constants supported, not mandatory--- ONLY RECTANGULAR **TRAVEL**<br>Format is <code>x<sub>1</sub>,y<sub>1</sub>:x<sub>2</sub>,y<sub>2</sub>;</code>      where Corner 1 and 2 are opposite and given in <code>x,y</code> form.  
|<code>recte(){<br>  0,0:70,20;<br>}|Rectangular Extrusion- will EXTRUDE outline of rectangle|Circle(){<br>  o:10,5 <br>r:25;<br>}|Circle outline travel-- DOES NOT EXTRUDE<br>Center point for circle<br> Radius of circle
|<code>ce(){<br> 10,5:25; <br>}|Circle Extrusion- extrudes outline of circle<br>Circles and Circle Extrusion support both <code>o:center;<br> r:radius;</code> syntax and <code>center:radius;</code> syntax
|<code>ENDOFFILE|Must have ENDOFFILE Declaration.|