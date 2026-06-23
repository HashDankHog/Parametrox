const { invoke } = window.parent.__TAURI__.core;
var segments = [];
var parameter_num=0;
function add_parameter() {
    const row = document.createElement("tr");
    
    const body = document.querySelector("table");
    row.innerHTML = document.getElementById(String(parameter_num)).innerHTML;
    const value = document.createElement("P");
    value.textContent = "0";
    value.className = "value";

    document.querySelector(".addParam").replaceWith(value);
    parameter_num++;
    row.id=String(parameter_num);
    row.querySelector(".paramNum").textContent=String(parameter_num);

    body.append(row);
}


async function update() {
    var expressions = [];
    const inputFields = document.getElementsByClassName("expression");
    for (let i = 0; i < inputFields.length; i+= 1) {
        if (inputFields[i].value != "") {
            
            expressions.push(inputFields[i].value);
        } else {
            expressions.push("0")
        }
    }
    var values = Array.from(await invoke("update_parameter", {expressions: expressions}));
    var parameter_values = document.getElementsByClassName("value");
    for (let i = 0; i < values.length; i+= 1) {
        parameter_values[i].innerHTML = String(values[i]);
    }
}
function add_segment(){
    var seg_pos = document.getElementsByClassName("parameter").length;
    segments.push(seg_pos);
    invoke("add_segment", {position: seg_pos});
    add_parameter();
    add_parameter();
}

document.getElementById("update").addEventListener("click", update);
document.getElementById("Add Segment").addEventListener("click", add_segment);

window.plot = function plot(){
    var expressions = [];
    const inputFields = document.getElementsByClassName("expression");
    for (let i = 0; i < inputFields.length; i+= 1) {
        if (inputFields[i].value != "") {
            
            expressions.push(inputFields[i].value);
        } else {
            expressions.push("0")
        }
    }
    for (let segment = 0; segment < segments.length; segment += 1){
        invoke("plot", {segment: segment, expressions: expressions, color: [255,255,255]});
    }
}

