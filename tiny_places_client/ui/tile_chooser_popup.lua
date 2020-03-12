-- 
-- Map editor tile chooser
--
-- Author: Hj. Malthaner
-- Date: 2020/03/09
--

local tileChooser = {}
local indexMap = {}

local function init(mainUi, tileset)
  print("Loading tileset popup")
  tileChooser.mainUi = mainUi
  tileChooser.tileset = tileset
end


local function update(dt)
end


local function draw()
  love.graphics.setColor(0.3, 0.3, 0.3)
  love.graphics.rectangle("fill", 100, 60, 1200-200, 720-120)


  local count = 0
  
  for index=0, 2000 do
      local tile = tileChooser.tileset[index]
      if tile and tile.image then
	  
        local x = 100 + (count % 12) * 64
        local y = 60 + math.floor(count/12) * 96
        local scale = 0.25
        
        love.graphics.setColor(0.6, 0.6, 0.6)
        love.graphics.rectangle("line", x, y, 64, 96)
        
        love.graphics.setColor(1, 1, 1)
        love.graphics.draw(tile.image, x - tile.footX*scale + 32, y - tile.footY*scale + 80, 0, scale, scale)

		indexMap[count] = tile.id
		
		count = count +1
      end
  end
end


local function mousePressed(button, mx, my)
  -- close this popup
  tileChooser.mainUi.popup = nil
  
  local index = math.floor((mx - 100) / 64)
  index = index + math.floor((my - 60) / 96) * 12
  
  local id = indexMap[index]
  
  print("Index=" .. index)
  print("ID=" .. id)
  
  tileChooser.mainUi.ui.tile = id
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
