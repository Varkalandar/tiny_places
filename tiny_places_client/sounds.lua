-- 
-- Tiny Places game sounds
--
-- Author: Hj. Malthaner
-- Date: 2020/04/04
--


local sounds = {}


local function init()

  local fireballLaunchData = love.sound.newSoundData("resources/sfx/fireball_launch.wav")
  local fireballHitData1 = love.sound.newSoundData("resources/sfx/fireball_hit_2a.wav")
  local fireballHitData2 = love.sound.newSoundData("resources/sfx/fireball_hit_3a.wav")

  sounds.fireballLaunch = love.audio.newSource(fireballLaunchData)
  sounds.fireballLaunch:setVolume(0.15)

  sounds.fireballHit1 = love.audio.newSource(fireballHitData1)
  sounds.fireballHit1:setVolume(0.3)

  sounds.fireballHit2 = love.audio.newSource(fireballHitData2)
  sounds.fireballHit2:setVolume(0.3)

end


sounds.init = init
sounds.play = play


return sounds