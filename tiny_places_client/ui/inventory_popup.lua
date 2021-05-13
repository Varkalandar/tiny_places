-- 
-- Player inventory popup
--
-- Author: Hj. Malthaner
-- Date: 2020/04/19
--

local inventoryPopup = {}

-- display coordinates of the equipment slots
local slots =
{
 {x=770, y=85, w=48, h=48},
 {x=1024, y=85, w=48, h=48},
 {x=743, y=252, w=48, h=48},
 {x=1053, y=252, w=48, h=48},
 {x=898, y=188, w=48, h=64}
}


local function init(mainUi, itemSet, playerInventory)
  print("Loading inventory popup")
 	inventoryPopup.image = love.graphics.newImage("resources/ui/silver/inventory_ui.png")
 	inventoryPopup.headerGradient = love.graphics.newImage("resources/ui/header_gradient.png")

  inventoryPopup.mainUi = mainUi
  inventoryPopup.itemSet = itemSet
  inventoryPopup.playerInventory = playerInventory
end


local function update(dt)
  if love.keyboard.isDown("escape") then
    inventoryPopup.mainUi.popup = nil
  end
end


local function drawItem(x, y, itemNo, scale)

  love.graphics.setColor(1, 1, 1)
  local tile = inventoryPopup.itemSet[itemNo]
  love.graphics.draw(tile.image, 
                     x, 
                     y,
                     0, scale, scale) 

end


local function drawItemCentered(x, y, itemNo, scale)

  love.graphics.setColor(1, 1, 1)
  local tile = inventoryPopup.itemSet[itemNo]
  love.graphics.draw(tile.image, 
                     x - tile.image:getWidth()/2 * scale, 
                     y - tile.image:getHeight()/2 * scale,
                     0, scale, scale) 

end


local function getItemUnderMouse()

  local mx = inventoryPopup.mainUi.mx
  local my = inventoryPopup.mainUi.my

  for k, item in pairs(inventoryPopup.playerInventory) do 

    -- print("mx=" .. mx .. " my=" .. my .. " displayX=" .. item.displayX .. " displayY=" .. item.displayY)
  
    if item.displayX <= mx and
       item.displayY <= my and
       item.displayX + item.displayW > mx and
       item.displayY + item.displayH > my then
       
       return item
    end
  end
  
  -- nothing found
  return nil
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

    local tile = item.tile+1
	  local image = inventoryPopup.itemSet[tile].image
    item.displayW = 32 * math.floor((image:getWidth()*item.scale+31) / 32)
    item.displayH = 32 * math.floor((image:getHeight()*item.scale+31) / 32)
  
    if item.where == -2 then
      -- backpack items
    
      item.displayX = xoff + item.x*32
      item.displayY = yoff + item.y*32
      
	    drawItemCentered(item.displayX + item.displayW/2, 
                       item.displayY + item.displayH/2, 
				  				     tile, 
					  			     item.scale)

    elseif item.where >= 0 then
      -- slotted items
       
      local slotx = slots[item.where].x 
      local sloty = slots[item.where].y 
       
      -- tile is image for map, tile+1 is image for inventory view
      drawItemCentered(slotx, sloty, item.tile+1, item.scale)
      
      item.displayX = slotx - item.displayW / 2 
      item.displayY = sloty - item.displayH / 2
       
    end    
  end

end


local function drawItemPopup(item, pixfont, xoff, yoff)

  local w = 240
  local h = 120
  
  love.graphics.setColor(0.2, 0.15, 0.1, 0.7)
  love.graphics.rectangle("fill", xoff+1, yoff+1, w-2, h-2)

  love.graphics.setColor(0.8, 0.4, 0.2, 0.7)
  love.graphics.draw(inventoryPopup.headerGradient, xoff, yoff, 0, w, 0.66)
  
  love.graphics.setColor(0.5, 0.4, 0.3)
  love.graphics.rectangle("line", xoff, yoff, w, h)

  love.graphics.setColor(1, 0.7, 0.2)
  local sw = pixfont.calcStringWidth(item.displayName) * 0.25
  pixfont.drawStringScaled(item.displayName, xoff + 2 + (w - sw)/2, yoff+6, 0.25, 0.25)
  
  local ybase = yoff+48
  local yspace = 24
  
  love.graphics.setColor(0.95, 0.9, 0.85)
  
  if item.energyDamage > 0 then
    pixfont.drawStringScaled("Energy damage: +" .. item.energyDamage .. "%", xoff + 8, ybase + yspace * 0, 0.22, 0.22)
    ybase = ybase + yspace
  end
  
  pixfont.drawStringScaled("Value: " .. item.value .. " credits", xoff + 8, ybase + yspace * 0, 0.22, 0.22)
  ybase = ybase + yspace
end


local function drawCorePopup(item, pixfont, xoff, yoff)

  local w = 240
  local h = 160
  
  -- select colors depending on type
  if item.itype == "func" then
    love.graphics.setColor(0.1, 0.15, 0.2, 0.7)
  else
    love.graphics.setColor(0.2, 0.15, 0.1, 0.7)
  end
  
  love.graphics.rectangle("fill", xoff+1, yoff+1, w-2, h-2)

  if item.itype == "func" then
    love.graphics.setColor(0.2, 0.4, 0.8, 0.7)
  else
    love.graphics.setColor(0.8, 0.4, 0.2, 0.7)
  end
  
  love.graphics.draw(inventoryPopup.headerGradient, xoff, yoff, 0, w, 0.66)
  
  if item.itype == "func" then
    love.graphics.setColor(0.3, 0.4, 0.5)
  else
    love.graphics.setColor(0.5, 0.4, 0.3)
  end
  
  love.graphics.rectangle("line", xoff, yoff, w, h)

  love.graphics.setColor(1, 0.7, 0.2)
  local sw = pixfont.calcStringWidth(item.displayName) * 0.25
  pixfont.drawStringScaled(item.displayName, xoff + 2 + (w - sw)/2, yoff+6, 0.25, 0.25)
  
  local ybase = yoff+48
  local yspace = 24
  
  love.graphics.setColor(0.85, 0.9, 0.95)
  
  local lines = pixfont.drawBoxStringScaled(item.description, xoff + 8, ybase + yspace * 0, w-8, h, yspace, 0.22, 0.22)
  ybase = ybase + yspace * lines + 6

  love.graphics.setColor(0.95, 0.9, 0.85)

  pixfont.drawStringScaled("Value: " .. item.value .. " credits", xoff + 8, ybase + yspace * 0, 0.22, 0.22)
  ybase = ybase + yspace
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
  local item = getItemUnderMouse()
  if item then
    local mainUi = inventoryPopup.mainUi
    
    if item.iclass == "core" then
      drawCorePopup(item, mainUi.pixfont, mainUi.mx, mainUi.my)
    else
      drawItemPopup(item, mainUi.pixfont, mainUi.mx, mainUi.my)
    end
    
  end
  
  if inventoryPopup.draggedItem then
    drawItem(inventoryPopup.mainUi.mx, inventoryPopup.mainUi.my,
             inventoryPopup.draggedItem.tile + 1,
             inventoryPopup.draggedItem.scale)
  end
  
  
  -- debug, show slot recatangles
  -- for i, slot in ipairs(slots) do
  --   love.graphics.rectangle("line",
  --                           slot.x - slot.w, slot.y - slot.h,
  --                          slot.w*2, slot.h*2)
  -- end
  
  
end


local function sendItemUpdate(item)

  local map = inventoryPopup.mainUi.gameUi.map
  
  -- for k, v in pairs(item) do print(k .. "=" .. v) end
  
  map.clientSocket.send("UPDI," .. 
                        item.itemId .. "," .. 
                        item.where .. ","..
                        item.x .. "," ..
                        item.y)

end


local function dropItem(mx, my)

  -- find out what the player clicked
  -- first check the backpack area
  
  local xoff = 626
  local yoff = 386

  local i = math.floor((mx - xoff) / 32)
  local j = math.floor((my - yoff) / 32)

  -- inside backpack area?
  if i >= 0 and j >= 0 and i <= 16 and j<= 8 then

    local item = inventoryPopup.draggedItem
    inventoryPopup.draggedItem = nil

    item.x = i
    item.y = j
    item.where = -2   -- -2 is magic for "in inventory"
    
    table.insert(inventoryPopup.playerInventory, item)
    sendItemUpdate(item)
  else
    -- test item slots
    for i, slot in ipairs(slots) do
      if slot.x - slot.w <= mx and slot.y - slot.h <= my and 
         slot.x + slot.w >= mx and slot.y + slot.h >= my then

        local item = inventoryPopup.draggedItem
        inventoryPopup.draggedItem = nil
         
        item.x = 0
        item.y = 0
        item.where = i
        
        table.insert(inventoryPopup.playerInventory, item)
        sendItemUpdate(item)
      end
    end
  end
  
  
end


local function mousePressed(button, mx, my)
end


local function mouseReleased(button, mx, my)

  local draggedItem = inventoryPopup.draggedItem
  local item = getItemUnderMouse()

  if draggedItem then
    dropItem(mx, my)

  else
    inventoryPopup.draggedItem = item
    local index = nil
    for k, item in pairs(inventoryPopup.playerInventory) do 
      if inventoryPopup.draggedItem == item then index = k end
    end
    
    table.remove(inventoryPopup.playerInventory, index)
  
    -- for k, v in pairs(inventoryPopup.draggedItem) do print(k .. "=" .. v) end
  
  end

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
