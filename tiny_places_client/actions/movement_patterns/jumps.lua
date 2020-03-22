--
-- Calculate jumps by z offset and z speed
--
-- Author: Hj. Malthaner
-- Date: 2020/03/22
--

local jumps = {}


local function calculate(mob, dt)
        
  local gravity = 16;
  
  if mob.zOff <= 0 then

      -- floor touched, new jump
      mob.zOff = 0
      mob.zSpeed = 2
  end
  
  mob.zSpeed = mob.zSpeed - gravity * dt
  mob.zOff = mob.zOff + mob.zSpeed
end


jumps.calculate = calculate


return jumps
