//TODO: FIX
//this doesnt export properly, I basically just need to rewrite this w3school mess of a program

var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
export function dragElement(elmnt) {
    const body = document.querySelector('body');
    if (document.getElementById(elmnt.className + "Header")) {
        // if present, the header is where you move the DIV from:
        const header = document.getElementById(elmnt.className + "Header");
        header.addEventListener("onmousedown", (evt) => {dragMouseDown(evt, elmnt)});
    } else {
        // otherwise, move the DIV from anywhere inside the DIV:
        elmnt.onmousedown = dragMouseDown;
    }
}

function dragMouseDown(evt, elmnt) {
    // get the mouse cursor position at startup:
    pos3 = evt.clientX;
    pos4 = evt.clientY;
    document.onmouseup = closeDragElement;
    // call a function whenever the cursor moves:
    document.onmousemove = elementDrag(evt, elmnt);
}

function elementDrag(evt, elmnt) {
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
export function closeWindow() {
    const a = document.getElementById("window");
    a.style.top = "-450px";
}