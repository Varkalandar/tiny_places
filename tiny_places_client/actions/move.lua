--
-- Move an object while time passes
--
-- Author: Hj. Malthaner
-- Date: 2020/03/19
--


local jumps = require("actions/movement_patterns/jumps")
local drops = require("actions/movement_patterns/drops")

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
  
  -- still moving?
  if len > steplen then
    
    local nx = dx/len * steplen
    local ny = dy/len * steplen
  
    mob.x = mob.x + nx
    mob.y = mob.y + ny

    mob:orient(dx, dy)
  
    -- make it jump
    if move.pattern == "glide" then
      -- nothing to do here
    elseif move.pattern == "glide2" then
      local time = (mob.id + math.floor(love.timer.getTime() * 8))
      local phase = time % 2
      -- print(phase*8)
      mob.displayTile = mob.displayTile + phase * 8
    elseif move.pattern == "drop" then
      drops.calculate(mob, dt)
      move.done = mob.zOff < 0

    elseif move.pattern == "spin" then
      local time = mob.id * 0.123 + love.timer.getTime() * 60 * 0.02
      local tix = mob.tile + math.floor(time % 8)
      mob.displayTile = tix

    else
      -- bounce is the default
      jumps.calculate(mob, dt)
    end
    
    -- print("nx=" .. nx .. " ny=" .. ny .. " mob.x="..mob.x .. " mob.y="..mob.y)
  else
    -- end of move now
    -- eliminate rounding errors
    mob.x = move.x
    mob.y = move.y
    mob.zOff = 0
    mob.zSpeed = 0
    move.done = true
  end    

  if move.done then
    -- print("Move done! id=" .. mob.id)
    
    if mob.type == "projectile" then
      -- print("Removing expired projectile with id=" .. mob.id)
      
      -- are all projectiles on layer 3?
      move.map.removeObject(mob.id, 3)
    
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
  
  return move
end


moves.new = new

return moves
