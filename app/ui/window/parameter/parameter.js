const { invoke } = window.parent.__TAURI__.core;

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
        expressions.push(inputFields[i].value);
    }
    var values = Array.from(await invoke("update_parameter", {expressions: expressions}));
    var parameter_values = document.getElementsByClassName("value");
    for (let i = 0; i < values.length; i+= 1) {
        parameter_values[i].innerHTML = String(values[i]);
    }
}

document.getElementById("update").addEventListener("click", update);
