parameter_num=0;
function add_parameter() {
    const row = document.createElement("tr");
    
    const body = document.querySelector("table");
    row.innerHTML = document.getElementById(String(parameter_num)).innerHTML;
    document.querySelector(".addParam").remove();
    parameter_num++;
    row.id=String(parameter_num);
    row.querySelector(".paramNum").textContent=String(parameter_num);

    body.append(row);
}