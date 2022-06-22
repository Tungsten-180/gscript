
# gscript
# WARNING!! CURRENTLY EXTREMELY ALPHA- DO NOT USE ON 3D PRINTER
<h3>A custom script interpreter for writing gcode more easily but providing more control than in a slicer.</h4>
<br>

## Currently Supports the following commands:
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


## Examples with Explanation

| Syntax | Explanation |
|--|--|
|<code>Settings(){</code><br><code>n:0.4;</code><br><code>w:0.4;</code><br><code>h:0.2;</code><br><code>m:1.0;</code><br><code>}|Mandatory settings command- can be changed at any point by another settings command<br>Nozzle diameter in mm(Code is unitless but default gcode is mm)<br> Line Width<br>Layer Height<br>Extrusion Multiplier-1 is 100% 0 is 0%(Code assumes perfect cylinder from nozzle squishes into perfect rectangle so play with this)|
|<code>Travel (const:Z.2,f400;){</code><br><code>0,0;</code><br><code> 1,1; </code><br><code>20,3.5;</code><br><code>50,50;</code><br><code>}|<expl>Supported Constants are Z(the Z axis) and F(speed).<br> Must be separated by comma if two, end with a ";"<br> <br> Points. Line must end with ";" and be in x,y format.</expl><br><br><br>-|
|<code>Probe(){</code><br><code>0,0;</code><br><code>1,1;</code><br><code>20,3.5;</code><br><code>50,50;</code><br><code>}|Points. Line must end with <code>;</code> and be in <code>x,y</code> format.
|<code>Rectangle(){</code><br><code>0,0:70,20;</code><br><code>}</code>|Constants supported, not mandatory--- ONLY RECTANGULAR **TRAVEL**<br>Format is <code>x<sub>1</sub>,y<sub>1</sub>:x<sub>2</sub>,y<sub>2</sub>;</code>where Corner 1 and 2 are opposite and given in <code>x,y</code> form.
|<code>recte(){</code><br><code>0,0:70,20;</code><br><code>}|Rectangular Extrusion- will EXTRUDE outline of rectangle|Circle(){</code><br><code>o:10,5 </code><br><code>r:25;</code><br><code>}|Circle outline travel-- DOES NOT EXTRUDE<br>Center point for circle<br> Radius of circle
|<code>ce(){</code><br><code> 10,5:25; </code><br><code>}|Circle Extrusion- extrudes outline of circle<br>Circles and Circle Extrusion support both <code>o:center;</code><br><code> r:radius;</code> syntax and <code>center:radius;</code> syntax
|<code>ENDOFFILE|Must have ENDOFFILE Declaration.|
