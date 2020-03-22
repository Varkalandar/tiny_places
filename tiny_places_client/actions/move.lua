--
-- Move an object while time passes
--
-- Author: Hj. Malthaner
-- Date: 2020/03/19
--


local jumps = require("actions/movement_patterns/jumps")

local moveFactory = {}


local function update(move, dt)

	-- advance, move by dt*speed

  local mob = move.mob
  local speed = 120
  
  local dx = move.x - mob.x
  local dy = move.y - mob.y

  local len = math.sqrt(dx * dx + dy * dy)
  
  -- print("dx=" .. dx .. " dy=" .. dy .. " len="..len)
  
  if len > 1 then
    
    local nx = dx/len * dt * speed
    local ny = dy/len * dt * speed
  
    mob.x = mob.x + nx
    mob.y = mob.y + ny
    
    -- calculate facing
    local r = math.atan2(ny*2, nx)
    
    -- round to a segment
    r = r + math.pi + math.pi/8

    -- calculate tile offsets from 0 to 7    
    r = 4 + math.floor((r * 8)  / (math.pi * 2))
    if r >= 8 then r = r - 8 end
    
    -- print("dx=" .. dx .. " dy=" .. dy .. " r="..r)

    -- set the tile to show
    mob.displayTile = mob.tile + r
    
    -- make it jump
    jumps.calculate(mob, dt)
    
    -- print("nx=" .. nx .. " ny=" .. ny .. " mob.x="..mob.x .. " mob.y="..mob.y)
  else
    
    -- eliminate rounding errors
    mob.x = move.x
    mob.y = move.y
    mob.zOff = 0
    mob.zSpeed = 0
    move.done = true
    
    print("Move done! id=" .. mob.id)
  end    
end


local function newMove(mob, x, y)
  local move = {}
  
  move.update = update
  move.mob = mob
  move.x = x
  move.y = y  

  move.done = false
  
  return move
end


moveFactory.newMove = newMove

return moveFactory
