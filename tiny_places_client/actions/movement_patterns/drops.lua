--
-- Calculate drops by z offset and z speed
--
-- Author: Hj. Malthaner
-- Date: 2020/03/22
--

local drops = {}


local function calculate(mob, dt)
        
  local gravity = 16;
  
  mob.zSpeed = mob.zSpeed - gravity * dt
  mob.zOff = mob.zOff + mob.zSpeed
end


drops.calculate = calculate


return drops
