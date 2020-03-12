-- 
-- Tiny Places map data
--
-- Author: Hj. Malthaner
-- Date: 2020/03/09
--

local tileset = require("tileset")
local clientSocket = require("net/client_socket")

local map = {}

-- layer tilesets
local mobSet = {}
local patchSet = {}
local cloudSet = {}


local function clear()
  map.mobs = {}
  map.patches = {}
  map.clouds = {}
end


local function getLayerTable(layer)
  if layer == 1 then
    return map.patches
  elseif layer == 3 then
    return map.mobs
  elseif layer == 5 then
    return map.clouds
  else
    print("getLayerTable(): Error - no such layer: " .. layer)
    return nil
  end
end


local function getLayerTileset(layer)
  if layer == 1 then
    return patchSet
  elseif layer == 3 then
    return mobSet
  elseif layer == 5 then
    return cloudSet
  else
    print("getLayerTable(): Error - no such layer: " .. layer)
    return nil
  end
end

local function addObject(id, layer, tile, x, y, scale)
  print("Adding object " .. tile .. " with id " .. id)

  local ltab = getLayerTable(layer)

  table.insert(ltab, {id=id, tile=tile, x=x, y=y, scale=scale})
end


local function updateObject(id, layer, tile, x, y, scale)
  print("Updating object " .. tile .. " with id " .. id)

  local ltab = getLayerTable(layer)

  for i, mob in pairs(ltab) do
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

local function deleteObject(id, layer)
  print("Removing object with id " .. id)
  
  local ltab = getLayerTable(layer)

  for i, mob in pairs(ltab) do
	  if mob.id == id then
	    table.remove(ltab, i)
	    break
	  end
  end
end


local function init()  
  print("Initializing map")
  
  mobSet = tileset.readSet("resources/objects/map_objects.tica")
  
  map.image = love.graphics.newImage("resources/map_floor.png")
  map.mobs = {}
  map.patches = {}
  map.clouds = {}
  
  map.mobSet = mobSet
  map.patchSet = patchSet
  map.cloudSet = cloudSet

  map.clientSocket = clientSocket

  -- host and port should come from a better place than this  
  map.clientSocket.connect("127.0.0.1", 9194)

  -- login should be here
  map.clientSocket.send("HELO")
  
  -- load the starting map
  map.clientSocket.send("LOAD")

end


local function selectObject(layer, x, y, catch)

  local ltab = getLayerTable(layer)
  local distance = catch * catch
  local best = nil
  
  for index, mob in pairs(ltab) do
		local xd = x - mob.x
		local yd = y - mob.y
		local d = xd*xd + yd*yd
		
		mob.selected = false
		
		if d < distance then
			distance = d
			best = mob
		end
  end
  
  if best then
	  best.selected = true
	else
	  print("map.selectObject(): Could not find an object in layer " .. layer .. " near "..x..", "..y)
	end
	
  return best
end


local function sortByDepth(mob1, mob2)
  return mob1.y < mob2.y
end


local function update(dt)
  table.sort(map.mobs, sortByDepth)
  table.sort(map.patches, sortByDepth)
  table.sort(map.clouds, sortByDepth)
end


local function drawTileTable(objects, set)
  for index, mob in ipairs(objects) do
    if mob.selected then
      love.graphics.ellipse("line", mob.x, mob.y, 30, 15)
    end
	
    local tile = set[mob.tile]
    local scale = mob.scale
	
    love.graphics.draw(tile.image, 
                       mob.x - tile.footX * scale, mob.y - tile.footY*scale, 0, scale, scale)
  end
end


local function drawFloor()
  love.graphics.setColor(1.0, 1.0, 1.0)
  love.graphics.draw(map.image)  
  drawTileTable(map.patches, patchSet)
end


local function drawObjects()
  love.graphics.setColor(1.0, 1.0, 1.0)
  drawTileTable(map.mobs, mobSet)
end


local function drawClouds()
  love.graphics.setColor(1.0, 1.0, 1.0)
  drawTileTable(map.clouds, cloudSet)
end


map.init = init
map.update = update
map.getLayerTileset = getLayerTileset

map.drawFloor = drawFloor
map.drawObjects = drawObjects
map.drawClouds = drawClouds

map.clear = clear
map.addObject = addObject
map.updateObject = updateObject
map.deleteObject = deleteObject
map.selectObject = selectObject

return map;
