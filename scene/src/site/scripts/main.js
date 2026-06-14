var c = document.getElementById("viewport_canvas");
var ctx = c.getContext("2d");
ctx.moveTo(0, 0);
ctx.lineTo(c.getBoundingClientRect().width, c.getBoundingClientRect().height);
ctx.stroke();
function populateRibbon(obj) {
    const ribbon = document.querySelector(".ribbon");
    for (const section of obj.section) {
        const sectionDiv = document.createElement("div");

        sectionDiv.className = "section";
        sectionDiv.id = section.name;

        
        const sectionHeader = document.createElement("h2");
        sectionHeader.textContent = section.name;
    
        sectionDiv.append(sectionHeader);

        
        for (const item of section.item) {
            const itemDiv = document.createElement("div");

            itemDiv.className = "item";
            itemDiv.id = item.name;

            
            const itemImage = document.createElement("img");
            itemImage.src = item.src;
            itemImage.textContent = item;
            
            itemDiv.append(itemImage);

            const actionDiv = document.createElement("div");

            actionDiv.className = "action";
            actionDiv.id = item.name;
            for (const action of item.action) {
                const actionButton = document.createElement("button");

                actionButton.className = "action";
                actionButton.id = action;
                
                actionButton.textContent = action;
                actionDiv.append(actionButton);
                
            }
            itemDiv.append(actionDiv);
            sectionDiv.append(itemDiv);
        }
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