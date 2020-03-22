-- 
-- Tiny Places map data
--
-- Author: Hj. Malthaner
-- Date: 2020/03/09
--

local moveFactory = require("actions/move")
local tileset = require("tileset")
local clientSocket = require("net/client_socket")

local map = {}

-- layer tilesets
local patchSet = {}
local mobSet = {}
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


local function unmarshallColor(color)
  if color then
    -- components are space separated
    local iter = color:gmatch("[^ ]+")

    local r = tonumber(iter())
    local g = tonumber(iter())
    local b = tonumber(iter())
    local a = tonumber(iter())

    return {r=r, b=b, g=g, a=a}
  else
    -- backwards compatibility - old objects had no color data
    return {r=1, g=1, b=0, a=1}
  end
end


local function findMob(id, layer)
  local ltab = getLayerTable(layer)

  for i, mob in pairs(ltab) do
    if mob.id == id then
      return mob
    end
  end
  
  return nil
end


local function addObject(id, layer, tile, x, y, scale, color)
  print("Adding object " .. tile .. " with id " .. id .. " to layer " .. layer)

  local ltab = getLayerTable(layer)
  local mob = {id=id, tile=tile, x=x, y=y, scale=scale, color=unmarshallColor(color)}

  table.insert(ltab, mob)
  
  return mob
end


local function updateObject(id, layer, tile, x, y, scale, color)
  print("Updating object " .. tile .. " with id " .. id)

  local mob = findMob(id, layer)

  if mob then
    mob.id=id
	  mob.tile=tile
    mob.x=x
    mob.y=y
    mob.scale=scale
    mob.color=unmarshallColor(color)
  end
end


local function deleteObject(id, layer)
  print("Removing object with id " .. id)
  
  local mob = findMob(id, layer)

  if mob then
    local ltab = getLayerTable(layer)
    table.remove(ltab, i)
  end
end


local function addMove(id, layer, x, y)
  print("Adding move for object with id " .. id .. " to " .. x .. ", " .. y)

  local actions = map.actions
  
  -- check if there is alread an ongoing move
  -- if yes, remove it from the table
  for k, v in pairs(actions) do
    if v.mob.id == id then
      print("Old move found in table, clearing old move ...")
      table.remove(actions, k)
    end
  end


  local mob = findMob(id, layer)
  local move = moveFactory.newMove(mob, x, y)
  
  table.insert(actions, move)  
end


local function init()  
  print("Initializing map")
  
  patchSet = tileset.readSet("resources/grounds/", "map_objects.tica")
  mobSet = tileset.readSet("resources/objects/", "map_objects.tica")
  cloudSet = tileset.readSet("resources/clouds/", "map_objects.tica")
  
  map.image = love.graphics.newImage("resources/map_floor.png")
  map.mobs = {}
  map.patches = {}
  map.clouds = {}
  
  map.mobSet = mobSet
  map.patchSet = patchSet
  map.cloudSet = cloudSet

  map.actions = {}

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


local function updateActions(dt)
	local actions = map.actions
	
	for k, v in pairs(actions) do
		v:update(dt)
	end

 	for k, v in pairs(actions) do
    if v.done then
      -- print("Removing stale action: " .. v)
		  table.remove(actions, k)
    end
	end

end


local function update(dt)
  
  updateActions(dt)
  
  table.sort(map.mobs, sortByDepth)
  table.sort(map.patches, sortByDepth)
  table.sort(map.clouds, sortByDepth)
end


local function drawTileTable(objects, set)
  for index, mob in ipairs(objects) do
    if mob.selected then
      love.graphics.ellipse("line", mob.x, mob.y, 30, 15)
    end
	
	  local color = mob.color	  
    love.graphics.setColor(color.r, color.g, color.b, color.a)	
	
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
  love.graphics.setColor(1.0, 1.0, 1.0, 0.5)
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
map.addMove = addMove

return map;
