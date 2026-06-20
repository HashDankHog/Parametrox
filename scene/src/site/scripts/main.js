/* 
It turns out that I just needed to move the draw_rect function to its own section
beforehand each draw_pixel call was taking roughly a millesecond
but since I was doing so many it added up super quickly
now it is taking roughly 400ms to run update_canvas and 1ms to run draw_rect which is a huge improvement
*/
const { invoke } = window.__TAURI__.core;

var c = document.getElementById("viewport_canvas");
var ctx = c.getContext("2d");
ctx.moveTo(0, 0);
ctx.lineTo(c.getBoundingClientRect().width, c.getBoundingClientRect().height);
ctx.stroke();

const img = ctx.getImageData(0, 0, c.width, c.height);
async function update_canvas(){
    const test = await invoke("update_canvas");
    const data = new Uint8ClampedArray(test);
    for (let i = 0; i < data.length; i += 1) {
        img.data[i] = data[i];
    }
    ctx.putImageData(img,0,0);  
}


invoke("create_canvas", {width: 1000, height: 1000});






function resizeIframe(obj) {
    obj.style.height = obj.contentWindow.document.documentElement.scrollHeight + 'px';
}
function populateRibbon(obj) {
    const ribbon = document.querySelector(".ribbon");
    for (const section of obj.section) {
        const sectionDiv = document.createElement("div");

        sectionDiv.className = "section";
        sectionDiv.id = section.name;

        const headerDiv = document.createElement("div");

        headerDiv.className = "header";
        headerDiv.id = section.name+"Header";

        const sectionHeader = document.createElement("h2");
        sectionHeader.textContent = section.name;
        headerDiv.append(sectionHeader);
        sectionDiv.append(headerDiv);

        const bottomDiv = document.createElement("div");
        bottomDiv.className = "bottom";
        bottomDiv.id = "bottom";

        for (const item of section.item) {
            const itemDiv = document.createElement("div");

            itemDiv.className = "item";
            itemDiv.id = item.name;

            
            const itemImage = document.createElement("img");
            itemImage.src = item.src;
            itemImage.textContent = item.name;
            
            itemDiv.append(itemImage);

            const actionDiv = document.createElement("div");

            actionDiv.className = "action";
            actionDiv.id = item.name;
            for (const action of item.action) {
                const actionButton = document.createElement("button");

                actionButton.className = "actionButton";
                actionButton.id = action;
                
                actionButton.textContent = action;
                actionDiv.append(actionButton);
                
            }
            itemDiv.append(actionDiv);
            bottomDiv.append(itemDiv);
        }
        sectionDiv.append(bottomDiv);
        ribbon.appendChild(sectionDiv);
    }
}

const myRequest = new Request("json/designRibbon.json");

fetch(myRequest)
  .then((response) => response.json())
  .then((data) => {
        populateRibbon(data);
  })
  .catch(console.error);

//code for draggable window, actually buns af and needs to be rewritten
//entirely
//TODO: fix

// Make the DIV element draggable:
dragElement(document.getElementById("window"));
var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
var elmnt=document.getElementById("windowHeader");
var win = document.getElementById("window");
elmnt.style.top="0px";
elmnt.style.left="0px";
win.style.top="0px";
win.style.left="0px";
function dragElement(elmnt) {
    const body = document.querySelector('body');
    if (document.getElementById(elmnt.className + "Header")) {
        // if present, the header is where you move the DIV from:
        const header = document.getElementById(elmnt.className + "Header");
        header.addEventListener("onmousedown", dragMouseDown);
    } else {
        // otherwise, move the DIV from anywhere inside the DIV:
        elmnt.onmousedown = dragMouseDown;
    }
}
function dragMouseDown(evt) {
    // get the mouse cursor position at startup:
    pos3 = evt.clientX;
    pos4 = evt.clientY;
    document.onmouseup = closeDragElement;
    // call a function whenever the cursor moves:
    document.onmousemove = elementDrag;
}

function elementDrag(evt) {
    // calculate the new cursor position:
    pos1 = pos3 - evt.clientX;
    pos2 = pos4 - evt.clientY;
    pos3 = evt.clientX;
    pos4 = evt.clientY;
    // set the element's new position:
    elmnt.style.top = (elmnt.offsetTop - pos2) + "px";
    elmnt.style.left = (elmnt.offsetLeft - pos1) + "px";
    win.style.top = (parseInt(win.style.top)-pos2)+"px";
    win.style.left = (parseInt(win.style.left)-pos1)+"px";
}

function closeDragElement() {
    // stop moving when mouse button is released:
    document.onmouseup = null;
    document.onmousemove = null;
}
function closeWindow() {
    const a = document.getElementById("window");
    a.style.top = "-450px";
}
closeWindow();
update_canvas();
let i = 100;
async function animate() {
    invoke("clear_canvas");
    invoke("draw_rect", {coord: [i,i], size: [100, 100], color: [255,255,255]});
    update_canvas();
    i +=1;
    requestAnimationFrame(animate);
}
requestAnimationFrame(animate);

document.getElementById("windowClose").addEventListener("click", closeWindow);


ctx.fillStyle = "rgb(200 0 0)";
ctx.fillRect(0, 0, 500, 500);
