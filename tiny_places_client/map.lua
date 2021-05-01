-- 
-- Tiny Places map data
--
-- Author: Hj. Malthaner
-- Date: 2020/03/09
--

local moves = require("actions/move")
local flashes = require("actions/flash")
local animations = require("actions/animation")
local spells = require("actions/spell")

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
local animationSet = {}


local function clear()
  map.mobs = {}
  map.patches = {}
  map.clouds = {}
  map.filename = nil
  map.name = "unnamed"
end


local function orient(mob, dx, dy)
  local faces = mob.faces
  
  -- calculate facing
  local r = math.atan2(dy*2, dx)
  
  -- round to a segment
  r = r + math.pi + math.pi/8

  -- calculate tile offsets from 0 to faces-1    
  r = faces/2 + math.floor((r * faces)  / (math.pi * 2) - 0.5)
  if r >= faces then r = r - faces end
  
  -- print("dx=" .. dx .. " dy=" .. dy .. " r="..r .. " faces=" .. faces)
  
  -- error, usually caused by a move of length 0
  -- in this case r is nan and IEEE (nan ~= nan) is true 
  if(r ~= r) then 
    print(debug.traceback()) 
  else
    -- set the tile to show
    mob.displayTile = mob.tile + r
  end
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


local function addObject(id, layer, tile, x, y, scale, color, ctype, speed, faces)
  
  print("Adding object with id " .. id ..  ", tile " .. tile .. " and type '" .. ctype .. "' to layer " .. layer)
  
  -- tile should be constant, displayTile can change during animations
  local mob = {id = id, tile = tile, displayTile = tile,
               x = x, y = y, scale = scale, 
               color = unmarshallColor(color), 
               type = ctype,
               speed = speed, zOff = 0, zSpeed = 0,
               faces = faces,
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


local function removeObject(id, layer)
  print("Removing object with id " .. id .. " from layer " .. layer)
  
  local mob, i = findMob(id, layer)

  if mob then
    local ltab = getLayerTable(layer)
    table.remove(ltab, i)
  end
end


local function addMove(id, layer, x, y, speed, pattern)
  -- print("Adding move for object with id " .. id .. " to " .. x .. ", " .. y)

  local actions = map.actions
  
  -- check if there is alread an ongoing move
  -- if yes, remove it from the table
  for k, v in pairs(actions) do
  
    -- hack - only move type actions have a pattern (and must have one)
    if v.mob and v.mob.id == id and v.pattern then
      print("Removing old move for mob id=" .. id)
      table.remove(actions, k)
    end
  end

  local mob = findMob(id, layer)
  mob.speed = speed
  
  -- debug desyncs
  -- if sx and sy then print("Move sync dx=" .. (sx - mob.x) .. " dy=" .. (sy - mob.y)) end
  
  local move = moves.new(map, mob, x, y, pattern, speed)
  
  table.insert(actions, move)

  return move
end


local function fireProjectile(source, id, layer, ptype, castTime, dx, dy, speed)
  -- print("Adding projectile with type " .. ptype .. " fired at " .. dx .. ", " .. dy)

  local shooter, i = findMob(source, layer)
  local nx = dx - shooter.x
  local ny = dy - shooter.y

  shooter:orient(nx, ny)

  local spell = spells.new(map, shooter, id, layer, ptype, castTime, dx, dy, speed, animationSet)
  
  -- some spells have a buildup time, the projectile will be fired later
  table.insert(map.actions, spell)

end


local function load(name, backdrop, filename)
  print("loading map '" .. name .. "' with backdrop '" .. backdrop .. "'")
  
  map.clear()
  map.name = name
  map.image = love.graphics.newImage("resources/map/" .. backdrop .. ".png")
  map.filename = filename
end


local function playAnimation(id, layer, x, y)
  
  if id == 1 then
    local function scalef(t) return 1, 1 end
    
    -- x, y, tileset, scalef, start, end, time, r, g, b, a
    local animation = animations.new(x, y, animationSet, scalef, 1, 20, 0.02, 1, 1, 1, 1, "add")
    table.insert(map.actions, animation)
    
    -- animation sound
    map.sounds.randplay(map.sounds.vortexBang, 1, 0.1)
    
  else
    local function scalef(t) local s = math.sin(t*math.pi) * 0.3 return s, s end
  
    -- x, y, tileset, scalex, scaley, start, end, time, r, g, b, a
    local animation = animations.new(x, y, animationSet, scalef, 40, 56, 0.12, 1, 1, 1, 1, "subtract")
    table.insert(map.actions, animation)
    
    -- shadow "flash"
    local flash = flashes.new(x, y+10, cloudSet[21].image, 0.18, 
                              0.7, 0.7, 0.7, "subtract", true, 0.2)
    table.insert(map.actions, flash)

    
    -- animation sound
    map.sounds.randplay(map.sounds.noisedChirp, 1, 0.2)
  
  end
end

local function init()  
  print("Initializing map")
  
  map.playerInventory = { }
  
  patchSet = tileset.readSet("resources/grounds/", "map_objects.tica")
  mobSet = tileset.readSet("resources/objects/", "map_objects.tica")
  creatureSet = tileset.readSet("resources/creatures/", "creatures.tica")
  playerSet = tileset.readSet("resources/players/", "players.tica")
  projectileSet = tileset.readSet("resources/projectiles/", "projectiles.tica")
  cloudSet = tileset.readSet("resources/clouds/", "map_objects.tica")
  animationSet = tileset.readSet("resources/animations/", "animations.tica")
  local itemSet = tileset.readSet("resources/items/", "items.tica")
  
  load("Wasteland", "map_wasteland", "")

  map.bumpmap = love.graphics.newImage("resources/map/map_bumps.png")
  map.mobs = {}
  map.patches = {}
  map.clouds = {}
  
  map.mobSet = mobSet
  map.patchSet = patchSet
  map.cloudSet = cloudSet
  map.itemSet = itemSet
  
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
  -- map.clientSocket.send("LOAD,wasteland_and_pond")
  -- map.clientSocket.send("LOAD,desert")
  map.clientSocket.send("LOAD,lobby")
  -- map.clientSocket.send("LOAD,dark_technoland")
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
      
      local mob = v.mob
      
      -- todo: cleanup
      if mob and mob.type == "projectile" and (mob.ptype == "fireball" or mob.ptype == "dust_vortex") then
      
        if mob.ptype == "fireball" then
          map.sounds.randplay2(map.sounds.fireballHit1, map.sounds.fireballHit2, 0.7, 1, 0.1)
        else
          map.sounds.randplay2(map.sounds.vortexBang1, map.sounds.vortexBang2, 0.5, 1.0, 0.2)
        end
        
        -- make flash appear a bit in front of target
        local flash

        if mob.ptype == "fireball" then
          flash = flashes.new(mob.x, mob.y+10, cloudSet[21].image, 1, 
                              1, 0.7, 0.4, "add", false, 1)
        else
          flash = flashes.new(mob.x, mob.y+10, cloudSet[21].image, 1, 
                              1, 0.9, 0.4, "add", false, 1)
        end
        
        table.insert(actions, flash)
      end
      
      if mob and mob.type == "projectile" and (mob.ptype == "debris" or mob.ptype == "dirt_shot") then
        if math.random() < 0.2 or mob.ptype == "dirt_shot" then
          map.sounds.randplay2(map.sounds.debrisHit1, map.sounds.debrisHit2, 0.5, 2.0, 1.0)
        end
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


local function drawPlayer(mob, tile, scale)
  if mob.tile == 20 then
  
  
    -- print("displayTile=" .. mob.displayTile)
  
    -- spectre testing
    local mode, alphamode = love.graphics.getBlendMode()
    love.graphics.setBlendMode("add", "alphamultiply")
    
    love.graphics.setColor(0.6, 0.6, 0.7, 1)
    love.graphics.draw(tile.image, 
                       mob.x - tile.footX * scale, 
                       mob.y - tile.footY * scale - mob.zOff, 
                       0, 
                       scale, scale)
    
    -- ground shine
    love.graphics.setColor(0.5, 0.5, 0.6, 0.2)
    -- scale = 0.9
    love.graphics.draw(cloudSet[21].image,
                       mob.x - 171 * scale,
                       mob.y - 67 * scale, 
                       0, scale, scale)

    love.graphics.setBlendMode(mode, alphamode)
    
     -- spectre testing end
  else
    love.graphics.draw(tile.image, 
                       mob.x - tile.footX * scale, 
                       mob.y - tile.footY * scale - mob.zOff, 
                       0, 
                       scale, scale)
  end
end


local function drawCreature(mob, tile, scale)
  
  if mob.tile == 9 then
    -- vortex testing
    local time = love.timer.getTime() * 60
    
    -- large dust disk
    local scale = 0.4
                       
    love.graphics.setColor(0.3, 0.28, 0.26, 0.9)
    local dust = animationSet[23 + math.floor((time * 0.3 + mob.id) % 16)]
    love.graphics.draw(dust.image, 
                       mob.x - dust.image:getWidth()/2 * scale, 
                       mob.y - dust.image:getHeight()/2 * scale - mob.zOff - 4, 
                       0, 
                       scale, scale)
    -- small dust disk
    local scale = 0.18
                       
    love.graphics.setColor(0.40, 0.38, 0.36, 0.5)
    local dust = animationSet[23 + math.floor((time * 1.0 + mob.id) % 16)]
    love.graphics.draw(dust.image, 
                       mob.x - dust.image:getWidth()/2 * scale, 
                       mob.y - dust.image:getHeight()/2 * scale - mob.zOff - 4, 
                       0, 
                       scale, scale)
                       
    -- vortex itself
    local vtime = time + (mob.x + mob.y) * 0.01
    local tix = mob.tile + math.floor(vtime % 8)
    tile = creatureSet[tix]
    
    local scale = 0.3
	  local color = mob.color	  
    love.graphics.setColor(color.r, color.g, color.b, color.a)
    
    love.graphics.draw(tile.image, 
                       mob.x - tile.footX * scale, 
                       mob.y - tile.footY * scale - mob.zOff, 
                       0, 
                       scale, scale)
                       
    -- vortex testing end
  else
    love.graphics.draw(tile.image, 
                       mob.x - tile.footX * scale, 
                       mob.y - tile.footY * scale - mob.zOff, 
                       0, 
                       scale, scale)
  end
end


local function drawProjectile(mob, tile, scale)

  local color = mob.color
  local mode, alphamode = love.graphics.getBlendMode()
  love.graphics.setBlendMode("add", "alphamultiply")

  if mob.ptype == "fireball" then
    love.graphics.setColor(color.r, color.g, color.b, color.a)
  elseif mob.ptype == "dust_vortex" then
    love.graphics.setColor(1.0, 0.9, 0.4, 0.5)
  else
    -- love.graphics.setColor(1.0, 1.0, 1.0, 0.3)
    love.graphics.setColor(color.r, color.g, color.b, color.a)
  end
  
  -- the projectile
  love.graphics.draw(tile.image, 
                     mob.x - tile.footX * scale, 
                     mob.y - tile.footY * scale - mob.zOff, 
                     0, 
                     scale, scale)

  local ptype = mob.ptype
  if ptype == "fireball" then 
    -- ground shine
    love.graphics.setColor(1.0, 0.7, 0.4, 0.5)
    scale = 0.9
    love.graphics.draw(cloudSet[21].image,
                       mob.x - 171 * scale,
                       mob.y - 67 * scale, 
                       0, scale, scale)
  elseif ptype == "dust_vortex" then
    -- ground shine
    love.graphics.setColor(1.0, 0.85, 0.5, 0.4)
    scale = 0.4
    love.graphics.draw(cloudSet[21].image,
                       mob.x - 171 * scale,
                       mob.y - 67 * scale, 
                       0, scale, scale)
  
  end
  
  love.graphics.setBlendMode(mode, alphamode)
end


local function drawTileTable(objects, set)

  for index, mob in ipairs(objects) do
  
    if mob.selected then
      love.graphics.setColor(1, 1, 1, 1)
      love.graphics.ellipse("line", mob.x, mob.y, 30, 15)
    end
	
	  local color = mob.color	  
    love.graphics.setColor(color.r, color.g, color.b, color.a)	
	
    local scale = mob.scale
    
    -- special cases
    if mob.type == "player" then
      local tile = playerSet[mob.displayTile]
      drawPlayer(mob, tile, scale)
    elseif mob.type == "creature" then
      local tile = creatureSet[mob.displayTile]
      drawCreature(mob, tile, scale)
    elseif mob.type == "projectile" then
      local tile = projectileSet[mob.displayTile]
      drawProjectile(mob, tile, scale)
    else
      local tile = set[mob.displayTile]
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

  -- there are drawable actions
	for k, v in pairs(map.actions) do
		if v.drawUnder then v:drawUnder() end
	end
  
end


local function drawObjects()
  drawTileTable(map.mobs, mobSet)
  
  -- there are drawable actions
	for k, v in pairs(map.actions) do
		if v.drawOver then v:drawOver() end
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
map.removeObject = removeObject
map.selectObject = selectObject
map.addMove = addMove
map.fireProjectile = fireProjectile
map.playAnimation = playAnimation

return map;
