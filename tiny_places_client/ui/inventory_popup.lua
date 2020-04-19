-- 
-- Player inventory popup
--
-- Author: Hj. Malthaner
-- Date: 2020/04/19
--

local playerInventory = {}


local function init(mainUi, tileset)
  print("Loading inventory popup")
 	playerInventory.image = love.graphics.newImage("resources/ui/inventory_ui.png")
 
  playerInventory.mainUi = mainUi
end


local function update(dt)
end


local function draw()
  love.graphics.setColor(0, 0, 0)
  love.graphics.rectangle("line", 600, 6, 596+2, 704+2)

  love.graphics.setColor(1, 1, 1)
  love.graphics.draw(playerInventory.image, 601, 7, 0, 1, 1) 

  love.graphics.setColor(0.15, 0.15, 0.15)
  for j=0,7 do
    for i=0,16 do
      love.graphics.rectangle("line", 626+i*32, 388+j*32, 32, 32)
    end
  end
end


local function mousePressed(button, mx, my)
end


local function mouseReleased(button, mx, my)
end


local function mouseDragged(button, mx, my)
end


playerInventory.init = init
playerInventory.update = update
playerInventory.draw = draw
playerInventory.mousePressed = mousePressed
playerInventory.mouseReleased = mouseReleased
playerInventory.mouseDragged = mouseDragged


return playerInventory
