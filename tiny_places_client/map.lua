-- 
-- Tiny Places map data
--
-- Author: Hj. Malthaner
-- Date: 2020/03/09
--

local moveFactory = require("actions/move")
local flash = require("actions/flash")

local tileset = require("tileset")
local clientSocket = require("net/client_socket")
local sounds = require("sounds")

local map = {}

-- layer tilesets
local patchSet = {}
local mobSet = {}
local creatureSet = {}
local playerSet = {}
local projectileSet = {}
local cloudSet = {}


local function clear()
  map.mobs = {}
  map.patches = {}
  map.clouds = {}
  map.filename = nil
end


local function orient(mob, dx, dy)

    -- calculate facing
    local r = math.atan2(dy*2, dx)
    
    -- round to a segment
    r = r + math.pi + math.pi/8

    -- calculate tile offsets from 0 to 7    
    r = 4 + math.floor((r * 8)  / (math.pi * 2))
    if r >= 8 then r = r - 8 end
    
    -- print("dx=" .. dx .. " dy=" .. dy .. " r="..r)

    -- set the tile to show
    mob.displayTile = mob.tile + r
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
      return mob, i
    end
  end
  
  return nil, nil
end


local function addObject(id, layer, tile, x, y, scale, color, ctype, speed)
  print("Adding object with id " .. id ..  ", tile " .. tile .. " and type '" .. ctype .. "' to layer " .. layer)

  -- tile should be constant, displayTile can change during animations
  local mob = {id = id, tile = tile, displayTile = tile,
               x = x, y = y, scale = scale, 
               color = unmarshallColor(color), 
               type = ctype,
               speed = speed, zOff = 0, zSpeed = 0,
               orient = orient}

  local ltab = getLayerTable(layer)
  table.insert(ltab, mob)
  
  return mob
end


local function updateObject(id, layer, tile, x, y, scale, color)
  print("Updating object " .. tile .. " with id " .. id)

  local mob = findMob(id, layer)

  if mob then
    mob.id=id
	  mob.tile=tile
	  mob.displayTile=tile
    mob.x=x
    mob.y=y
    mob.scale=scale
    mob.color=unmarshallColor(color)
  end
end


local function deleteObject(id, layer)
  print("Removing object with id " .. id .. " from layer " .. layer)
  
  local mob, i = findMob(id, layer)

  if mob then
    local ltab = getLayerTable(layer)
    table.remove(ltab, i)
  end
end


local function addMove(id, layer, x, y, speed, pattern)
  print("Adding move for object with id " .. id .. " to " .. x .. ", " .. y)

  local actions = map.actions
  
  -- check if there is alread an ongoing move
  -- if yes, remove it from the table
  for k, v in pairs(actions) do
    if v.mob and v.mob.id == id then
      print("Old move found in table, clearing old move ...")
      table.remove(actions, k)
    end
  end

  local mob = findMob(id, layer)
  mob.speed = speed
  
  local move = moveFactory.newMove(map, mob, x, y, pattern)
  
  table.insert(actions, move)  
end


local function addProjectile(source, id, layer, ptype, sx, sy, dx, dy, speed)
  print("Adding projectile with type " .. ptype .. " fired at " .. dx .. ", " .. dy)

  local shooter, i = findMob(source, layer)
  
  
  -- there shouldbe a ptype -> tile calculation here, once
  -- there is more than one projectile type
  local tile = 1;
  
  -- make projectile appear in front of the shooter
  local nx = dx-sx
  local ny = dy-sy
  local len = math.sqrt(nx*nx + ny*ny)
  
  nx = nx / len
  ny = ny / len

  local distance = 12
  sx = sx + nx * distance * 2
  sy = sy + ny * distance
  
  shooter:orient(nx, ny)

  addObject(id, layer, tile, sx, sy, 1, "1 1 1 1", "projectile", speed)
  addMove(id, layer, dx, dy, speed, "glide")
end


local function load(backdrop, filename)
  map.image = love.graphics.newImage("resources/map/" .. backdrop .. ".png")
  map.filename = filename
end


local function init()  
  print("Initializing map")
  
  patchSet = tileset.readSet("resources/grounds/", "map_objects.tica")
  mobSet = tileset.readSet("resources/objects/", "map_objects.tica")
  creatureSet = tileset.readSet("resources/creatures/", "creatures.tica")
  playerSet = tileset.readSet("resources/players/", "players.tica")
  projectileSet = tileset.readSet("resources/projectiles/", "projectiles.tica")
  cloudSet = tileset.readSet("resources/clouds/", "map_objects.tica")
  
  load("map_wasteland", nil)

  map.bumpmap = love.graphics.newImage("resources/map/map_bumps.png")
  map.mobs = {}
  map.patches = {}
  map.clouds = {}
  
  map.mobSet = mobSet
  map.patchSet = patchSet
  map.cloudSet = cloudSet

  map.actions = {}
  
  sounds.init()
  map.sounds = sounds
  
  map.clientSocket = clientSocket

  -- host and port should come from a better place than this  
  map.clientSocket.connect("127.0.0.1", 9194)

  -- login should be here
  map.clientSocket.send("HELO")
  
  -- load the starting map
  -- map.clientSocket.send("LOAD,green_and_pond")
  map.clientSocket.send("LOAD,wasteland_and_pond")

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
      
      -- todo: cleanup
      if v.mob and v.mob.type == "projectile" then
        if math.random() < 0.7 then
          map.sounds.fireballHit1:stop()
          map.sounds.fireballHit1:setPitch(0.9 + math.random() * 0.2)
          map.sounds.fireballHit1:play()
        else
          map.sounds.fireballHit2:stop()
          map.sounds.fireballHit2:setPitch(0.9 + math.random() * 0.2)
          map.sounds.fireballHit2:play()
        end
        
        local flash = flash.new(v.mob.x, v.mob.y - v.mob.zOff, cloudSet[18].image)
        table.insert(actions, flash)
      end
      
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
	
    local tile
    
    -- special cases
    if mob.type == "player" then
      tile = playerSet[mob.displayTile]
    elseif mob.type == "creature" then
      tile = creatureSet[mob.displayTile]
    elseif mob.type == "projectile" then
      tile = projectileSet[mob.displayTile]

      -- testing
      local scale = mob.scale
      local mode, alphamode = love.graphics.getBlendMode()
      love.graphics.setColor(1.0, 0.9, 0.5, 0.3)
      love.graphics.setBlendMode("add", "alphamultiply")
      love.graphics.draw(tile.image, 
                         mob.x - tile.footX * scale, 
                         mob.y - tile.footY * scale - mob.zOff, 
                         0, 
                         scale, scale)

      love.graphics.setColor(1.0, 0.8, 0.4, 0.5)
      scale = 1.5
      love.graphics.draw(cloudSet[18].image,
                         mob.x - 98 * scale,
                         mob.y - 49 * scale, 
                         0, scale, scale)


      love.graphics.setBlendMode(mode, alphamode)

      -- testing end
      
    else
      tile = set[mob.displayTile]
    end
    
    local scale = mob.scale
	
    if mob.type ~= "projectile" then
      if tile.image then
        love.graphics.draw(tile.image, 
                           mob.x - tile.footX * scale, 
                           mob.y - tile.footY * scale - mob.zOff, 
                           0, 
                           scale, scale)
      else
        print("Error in map.drawTileTable(): tile #" .. mob.displayTile .. " has no image")
      end
    end
  end
end


local function drawFloor()
  love.graphics.setColor(1.0, 1.0, 1.0, 1.0)
  love.graphics.draw(map.image, 0, 24)

  drawTileTable(map.patches, patchSet)

  love.graphics.setColor(1.0, 1.0, 1.0, 1.0)
  local mode, alphamode = love.graphics.getBlendMode()
  love.graphics.setBlendMode("multiply", "premultiplied")
  love.graphics.draw(map.bumpmap, 0, 24)
  love.graphics.setBlendMode(mode, alphamode)
end


local function drawObjects()
  drawTileTable(map.mobs, mobSet)
  
  -- there are drawable actions
	for k, v in pairs(map.actions) do
		if v.draw then v:draw() end
	end
  
end


local function drawClouds()
  drawTileTable(map.clouds, cloudSet)
end


map.init = init
map.update = update
map.getLayerTileset = getLayerTileset

map.drawFloor = drawFloor
map.drawObjects = drawObjects
map.drawClouds = drawClouds

map.clear = clear
map.load = load
map.addObject = addObject
map.updateObject = updateObject
map.deleteObject = deleteObject
map.selectObject = selectObject
map.addMove = addMove
map.addProjectile = addProjectile

return map;
