-- 
-- Player inventory popup
--
-- Author: Hj. Malthaner
-- Date: 2020/04/19
--

local playerInventory = {}


local function init(mainUi, itemSet)
  print("Loading inventory popup")
 	playerInventory.image = love.graphics.newImage("resources/ui/inventory_ui.png")
 
  playerInventory.mainUi = mainUi
  playerInventory.itemSet = itemSet
end


local function update(dt)
  if love.keyboard.isDown("escape") then
    playerInventory.mainUi.popup = nil
  end
end


local function drawItem(xoff, yoff, itemNo)

  local tile = playerInventory.itemSet[itemNo]
  love.graphics.setColor(0, 0.1, 0.2)
  love.graphics.rectangle("fill", xoff, yoff, 32, 32)
  love.graphics.setColor(1, 1, 1)
  love.graphics.draw(tile.image, xoff, yoff) 

end


local function checkItemPopup(xoff, yoff)
  -- todo
  return 1
end


local function drawBackpack(xoff, yoff)

  drawItem(xoff, yoff, 1)

  -- love.graphics.setColor(0.15, 0.15, 0.15)
  love.graphics.setColor(0.2, 0.2, 0.2)
  for j=0,7 do
    for i=0,16 do
      love.graphics.rectangle("line", xoff+i*32, yoff+j*32, 32, 32)
    end
  end

end


local function drawItemPopup(pixfont, xoff, yoff)

  local w = 160
  local h = 200
  
  love.graphics.setColor(0, 0, 0, 0.5)
  love.graphics.rectangle("fill", xoff+1, yoff+1, w-2, h-2)

  love.graphics.setColor(0.5, 0.5, 0.5)
  love.graphics.rectangle("line", xoff, yoff, w, h)
  -- love.graphics.print("73 Coins", xoff+rx+1, yoff+ry+16, 0, 1, 1)
  
  love.graphics.setColor(1, 0.7, 0.2)
  local sw = pixfont.calcStringWidth("73 Coins") * 0.25
  pixfont.drawStringScaled("73 Coins", xoff + 2 + (w - sw)/2, yoff+8, 0.25)
  
  local ybase = yoff+42
  local yspace = 22
  
  love.graphics.setColor(0.9, 0.9, 0.9)
  pixfont.drawStringScaled("Standard currency", xoff + 4, ybase + yspace * 0, 0.2)

  local ybase = ybase+32

  love.graphics.setColor(0.7, 0.7, 0.7)
  pixfont.drawStringScaled("This is for testing", xoff + 4, ybase + yspace * 0, 0.2)
  pixfont.drawStringScaled("more lines and", xoff + 4, ybase + yspace * 1, 0.2)
  pixfont.drawStringScaled("numbers.", xoff + 4, ybase + yspace * 2, 0.2)
  pixfont.drawStringScaled("1234567890", xoff + 4, ybase + yspace * 3, 0.2)
end


local function draw()
  love.graphics.setColor(0, 0, 0)
  love.graphics.rectangle("line", 600, 6, 596+2, 704+2)

  love.graphics.setColor(1, 1, 1)
  love.graphics.draw(playerInventory.image, 601, 7, 0, 1, 1) 

  local xoff = 626
  local yoff = 386
  drawBackpack(xoff, yoff)
  
  -- if mouse is over an item, draw a popup
  local item = checkItemPopup(playerInventory.mainUi.mx - xoff, playerInventory.mainUi.my - yoff)
  if item then
    drawItemPopup(playerInventory.mainUi.pixfont, xoff+16, yoff+16)
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
