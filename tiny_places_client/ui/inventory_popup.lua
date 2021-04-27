-- 
-- Player inventory popup
--
-- Author: Hj. Malthaner
-- Date: 2020/04/19
--

local inventoryPopup = {}


local function init(mainUi, itemSet, playerInventory)
  print("Loading inventory popup")
 	inventoryPopup.image = love.graphics.newImage("resources/ui/silver/inventory_ui.png")
 
  inventoryPopup.mainUi = mainUi
  inventoryPopup.itemSet = itemSet
  inventoryPopup.playerInventory = playerInventory
end


local function update(dt)
  if love.keyboard.isDown("escape") then
    inventoryPopup.mainUi.popup = nil
  end
end


local function drawItem(xoff, yoff, itemNo, scale)

  local tile = inventoryPopup.itemSet[itemNo]
  love.graphics.setColor(0, 0.1, 0.2)
  love.graphics.rectangle("fill", xoff, yoff, 32, 32)
  love.graphics.setColor(1, 1, 1)
  love.graphics.draw(tile.image, xoff, yoff, 0, scale, scale) 

end


local function drawItemCentered(xoff, yoff, itemNo, scale)

  local tile = inventoryPopup.itemSet[itemNo]
  love.graphics.setColor(0, 0.1, 0.2)
  love.graphics.rectangle("fill", xoff, yoff, 32, 32)
  love.graphics.setColor(1, 1, 1)
  love.graphics.draw(tile.image, 
                                xoff - tile.image:getWidth()/2 * scale, 
                                yoff - tile.image:getHeight()/2 * scale,
                                0, scale, scale) 

end


local function checkItemPopup(xoff, yoff)
  -- todo
  return 1
end

local function drawBackpack(xoff, yoff)

  -- love.graphics.setColor(0.15, 0.15, 0.15)
  love.graphics.setColor(0.2, 0.2, 0.2)
  for j=0,7 do
    for i=0,16 do
      love.graphics.rectangle("line", xoff+i*32, yoff+j*32, 32, 32)
    end
  end

  for k, item in pairs(inventoryPopup.playerInventory) do 
    
    if item.where == -2 then
      -- backpack items
    elseif item.where >= 0 then
       -- slotted items
       
       -- tile is image for map, tile+1 is image for inventory view
      drawItemCentered(770, 85, item.tile+1, item.scale)
       
    end    
  end


  drawItem(xoff, yoff, 1)

end


local function drawItemPopup(item, pixfont, xoff, yoff)

  local w = 200
  local h = 200
  
  love.graphics.setColor(0.3, 0.3, 0.3, 0.7)
  love.graphics.rectangle("fill", xoff+1, yoff+1, w-2, h-2)

  love.graphics.setColor(0.5, 0.5, 0.5)
  love.graphics.rectangle("line", xoff, yoff, w, h)

  love.graphics.setColor(1, 0.7, 0.2)
  local sw = pixfont.calcStringWidth(item.displayName) * 0.25
  pixfont.drawStringScaled(item.displayName, xoff + 2 + (w - sw)/2, yoff+8, 0.25, 0.25)
  
  local ybase = yoff+42
  local yspace = 22
  
  love.graphics.setColor(0.95, 0.9, 0.85)
  pixfont.drawStringScaled("Energy damage +" .. item.energyDamage .. "%", xoff + 4, ybase + yspace * 0, 0.2, 0.2)

  local ybase = ybase+32

  love.graphics.setColor(0.75, 0.7, 0.65)
  pixfont.drawStringScaled("This is for testing", xoff + 4, ybase + yspace * 0, 0.2, 0.2)
  pixfont.drawStringScaled("more lines and", xoff + 4, ybase + yspace * 1, 0.2, 0.2)
  pixfont.drawStringScaled("numbers.", xoff + 4, ybase + yspace * 2, 0.2, 0.2)
  pixfont.drawStringScaled("1234567890", xoff + 4, ybase + yspace * 3, 0.2, 0.2)
end


local function draw()
  love.graphics.setColor(0, 0, 0)
  love.graphics.rectangle("line", 600, 6, 596+2, 704+2)

  love.graphics.setColor(1, 1, 1)
  love.graphics.draw(inventoryPopup.image, 601, 7, 0, 1, 1) 

  local xoff = 626
  local yoff = 386
  drawBackpack(xoff, yoff)
  
  -- if mouse is over an item, draw a popup
  local item = checkItemPopup(inventoryPopup.mainUi.mx - xoff, inventoryPopup.mainUi.my - yoff)
  if item then
    drawItemPopup(inventoryPopup.playerInventory[1], inventoryPopup.mainUi.pixfont, 780, 60)
  end
  
end


local function mousePressed(button, mx, my)
end


local function mouseReleased(button, mx, my)
end


local function mouseDragged(button, mx, my)
end


inventoryPopup.init = init
inventoryPopup.update = update
inventoryPopup.draw = draw
inventoryPopup.mousePressed = mousePressed
inventoryPopup.mouseReleased = mouseReleased
inventoryPopup.mouseDragged = mouseDragged


return inventoryPopup
