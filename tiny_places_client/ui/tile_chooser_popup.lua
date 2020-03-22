-- 
-- Map editor tile chooser
--
-- Author: Hj. Malthaner
-- Date: 2020/03/09
--

local tileChooser = {}
local indexMap = {}
local uiOrderedSet = {}


local function init(mainUi, tileset)
  print("Loading tileset popup")
  
  uiOrderedSet = {}
  
  tileChooser.mainUi = mainUi
  tileChooser.tileset = tileset
  tileChooser.page = 0
  
  -- build table with tiles in ui order
  for index=0, 3000 do
    local tile = tileChooser.tileset[index]
    if tile then
      uiOrderedSet[tile.uiOrder] = tile
    end
  end  
end


local function update(dt)
end


local function drawPage(startIndex)
  
  local count = 0
  local endIndex = startIndex
  
  for index=startIndex, 3000 do
    local tile = uiOrderedSet[index]
    if tile and tile.image then
  
      local x = (1200-1024)/2 + (count % 16) * 64
      local y = 60 + math.floor(count/16) * 96
      local scale = 0.25
      
      love.graphics.setColor(0.6, 0.6, 0.6)
      love.graphics.rectangle("line", x, y, 64, 96)
      
      love.graphics.setColor(1, 1, 1)
      love.graphics.draw(tile.image, x - tile.footX*scale + 32, y - tile.footY*scale + 80, 0, scale, scale)

      indexMap[count] = tile.id
	    endIndex = index	
      count = count + 1
      
      if(count >= 16*6) then
        break
      end
    end
  end

  -- love.graphics.setColor(1, 1, 1)
  love.graphics.setColor(1.0*0.9, 0.8*0.9, 0.4*0.9)
  love.graphics.print("< Prev Page", 100, 641, 0, 1.25, 1)
  love.graphics.print("Next Page >", 1006, 641, 0, 1.25, 1)

  return endIndex
end


local function findStartIndexForPage(page)
  -- a page can show 16 * 6 tiles
  
  local start = page * 16 * 6
  local count = 0
  
  for index=0, 3000 do
    local tile = uiOrderedSet[index]
    if tile and tile.image then
      count = count + 1
      if count >= start then
        return index
      end
    end
  end
  
  return 0
end


local function draw()
  love.graphics.setColor(0.3, 0.3, 0.3)
  love.graphics.rectangle("fill", (1200-1024)/2, 60, 1024, 720-120)

  local startIndex = findStartIndexForPage(tileChooser.page)
  drawPage(startIndex)

  -- print("page=" .. tileChooser.page .. " startIndex=" .. startIndex)
end


local function mousePressed(button, mx, my)

  -- inside the tile selection?
  if my < 638 then

    -- yes, select and close this popup
    tileChooser.mainUi.popup = nil
    
    local index = math.floor((mx - 100) / 64)
    index = index + math.floor((my - 60) / 96) * 16
    
    local id = indexMap[index]
    
    print("Index=" .. index)
    print("ID=" .. id)
    
    tileChooser.mainUi.ui.tile = id

    if tileChooser.mainUi.ui.previewMob then
      tileChooser.mainUi.ui.previewMob.tile = id
      tileChooser.mainUi.ui.previewMob.displayTile = id
		end
		  
  else
    -- no -> navigation buttons
    
    if mx < 600 then
      tileChooser.page = tileChooser.page - 1
    else
      tileChooser.page = tileChooser.page + 1
    end
    
    if tileChooser.page < 0 then 
      tileChooser.page = 0 
    end
        
    print("page=" .. tileChooser.page)
  end
  
end


local function mouseReleased(button, mx, my)
end


local function mouseDragged(button, mx, my)
end


tileChooser.init = init
tileChooser.update = update
tileChooser.draw = draw
tileChooser.mousePressed = mousePressed
tileChooser.mouseReleased = mouseReleased
tileChooser.mouseDragged = mouseDragged


return tileChooser
