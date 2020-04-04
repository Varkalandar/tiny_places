--
-- Move an object while time passes
--
-- Author: Hj. Malthaner
-- Date: 2020/03/19
--


local jumps = require("actions/movement_patterns/jumps")

local moves = {}


local function update(move, dt)

	-- advance, move by dt*speed

  local mob = move.mob
  local speed = mob.speed
  
  local dx = move.x - mob.x
  local dy = move.y - mob.y

  local len = math.sqrt(dx * dx + dy * dy)
  
  -- print("dx=" .. dx .. " dy=" .. dy .. " len="..len)
  
  local steplen = dt * speed
  
  if len > steplen then
    
    local nx = dx/len * steplen
    local ny = dy/len * steplen
  
    mob.x = mob.x + nx
    mob.y = mob.y + ny

    mob:orient(dx, dy)
  
    -- make it jump
    if move.pattern == "glide" then
      -- nothing to do here
    else
      -- bounce is the default
      jumps.calculate(mob, dt)
    end
    
    -- print("nx=" .. nx .. " ny=" .. ny .. " mob.x="..mob.x .. " mob.y="..mob.y)
  else
    
    -- eliminate rounding errors
    mob.x = move.x
    mob.y = move.y
    mob.zOff = 0
    mob.zSpeed = 0
    move.done = true
    
    print("Move done! id=" .. mob.id)
    
    if mob.type == "projectile" then
      print("Removing expired projectile with id=" .. mob.id)
      
      -- are all projectiles on layer 3?
      move.map.deleteObject(mob.id, 3)
    
    end
    
  end    
end


local function new(map, mob, x, y, pattern)
  local move = {}
  
  move.update = update
  move.map = map
  move.mob = mob
  move.x = x
  move.y = y
  move.pattern = pattern  

  move.done = false
  
  if mob.type == "projectile" and pattern == "glide" then
    mob.zOff = 20
  end

  return move
end


moves.new = new

return moves
