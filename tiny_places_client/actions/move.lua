--
-- Move an object while time passes
--
-- Author: Hj. Malthaner
-- Date: 2020/03/19
--

local moveFactory = {}

local function update(move, dt)

	-- advance move by dt*speed


end

local function newMove(path)
  local move = {}
  move.update = update
  move.path = path
  return move
end


moveFactory.newMove = newMove

return moveFactory
