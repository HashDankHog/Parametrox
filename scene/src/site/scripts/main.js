var c = document.getElementById("viewport_canvas");
var ctx = c.getContext("2d");
ctx.moveTo(0, 0);
ctx.lineTo(c.getBoundingClientRect().width, c.getBoundingClientRect().height);
ctx.stroke();