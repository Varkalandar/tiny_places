-- 
-- Tiny Places game sounds
--
-- Author: Hj. Malthaner
-- Date: 2020/04/04
--


local sounds = {}


local function init()

  local fireballLaunchData = love.sound.newSoundData("resources/sfx/fireball_launch.wav")
  local fireballHitData = love.sound.newSoundData("resources/sfx/fireball_hit_3.wav")

  sounds.fireballLaunch = love.audio.newSource(fireballLaunchData)
  sounds.fireballLaunch:setVolume(0.15)

  sounds.fireballHit = love.audio.newSource(fireballHitData)
  sounds.fireballHit:setVolume(0.3)

end


sounds.init = init
sounds.play = play


return sounds