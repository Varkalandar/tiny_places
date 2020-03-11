-- 
-- Tiny Places map data
--
-- Author: Hj. Malthaner
-- Date: 2020/03/09
--

local tileset = require("tileset")
local clientSocket = require("net/client_socket")

local map = {}


local function clear()
  map.mobs = {}
end

local function addObject(id, tile, x, y, scale)
  print("Adding object " .. tile .. " with id " .. id)

  table.insert(map.mobs, {id=id, tile=tile, x=x, y=y, scale=scale})
end


local function updateObject(id, tile, x, y, scale)
  print("Updating object " .. tile .. " with id " .. id)

  for i, mob in pairs(map.mobs) do
	if mob.id == id then
	  mob.id=id
	  mob.tile=tile
	  mob.x=x
	  mob.y=y
	  mob.scale=scale
	  
	  break
	end
  end
end


local function init()  
  print("Initializing map")
  
  tileset.init()
  
  map.image = love.graphics.newImage("resources/map_floor.png")
  map.mobs = {}
  map.tileset = tileset
  
  -- host and port should come from a better place than this  
  clientSocket.connect("127.0.0.1", 9194)

  -- login should be here
  clientSocket.send("HELO")
  
  -- load the starting map
  clientSocket.send("LOAD")

  map.clientSocket = clientSocket
end


local function selectObject(x, y, catch)
  local distance = catch * catch
  
  for index, mob in pairs(map.mobs) do
		local xd = x - mob.x
		local yd = y - mob.y
		local d = xd*xd + yd*yd
		
		mob.selected = false
		
		if d < distance then
			distance = d
			best = mob
			best.selected = true
		end
  end
  
  return best
end


local function drawFloor(roomNumber)
  love.graphics.setColor(1.0, 1.0, 1.0)
  love.graphics.draw(map.image)  
end


local function drawObjects(roomNumber)
  love.graphics.setColor(1.0, 1.0, 1.0)

  for index, mob in pairs(map.mobs) do
	if mob.selected then
	  love.graphics.ellipse("line", mob.x, mob.y, 30, 15)
	end
	
	local tile = tileset.get(mob.tile)
	local scale = mob.scale
	
    love.graphics.draw(tile.image, 
                       mob.x - tile.footX * scale, mob.y - tile.footY*scale, 0, scale, scale)
  end
end


map.init = init
map.drawFloor = drawFloor
map.drawObjects = drawObjects
map.clear = clear
map.addObject = addObject
map.updateObject = updateObject
map.selectObject = selectObject


return map;
