--
-- Move an object while time passes
--
-- Author: Hj. Malthaner
-- Date: 2020/03/19
--

local moveFactory = {}


local function update(move, dt)

	-- advance, move by dt*speed

  local mob = move.mob
  local speed = 100
  
  local dx = move.x - mob.x
  local dy = move.y - mob.y

  local len = math.sqrt(dx * dx + dy * dy)
  
  -- print("dx=" .. dx .. " dy=" .. dy .. " len="..len)
  
  if len > 1 then
    
    local nx = dx/len * dt * speed
    local ny = dy/len * dt * speed
  
  
    mob.x = mob.x + nx
    mob.y = mob.y + ny
    
    -- print("nx=" .. nx .. " ny=" .. ny .. " mob.x="..mob.x .. " mob.y="..mob.y)
  else
    -- eliminate rounding errors
    mob.x = move.x
    mob.y = move.y
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
