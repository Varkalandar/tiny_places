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

  local debrisHitData1 = love.sound.newSoundData("resources/sfx/debris.wav")
  local debrisHitData2 = love.sound.newSoundData("resources/sfx/debris_low.wav")

  local vortexBangData1 = love.sound.newSoundData("resources/sfx/vortex_bang.mp3")

  sounds.fireballLaunch = love.audio.newSource(fireballLaunchData)
  sounds.fireballLaunch:setVolume(0.15)

  sounds.fireballHit1 = love.audio.newSource(fireballHitData1)
  sounds.fireballHit1:setVolume(0.3)

  sounds.fireballHit2 = love.audio.newSource(fireballHitData2)
  sounds.fireballHit2:setVolume(0.3)

  sounds.debrisHit1 = love.audio.newSource(debrisHitData1)
  sounds.debrisHit1:setVolume(0.02)

  sounds.debrisHit2 = love.audio.newSource(debrisHitData2)
  sounds.debrisHit2:setVolume(0.02)

  sounds.vortexBang1 = love.audio.newSource(vortexBangData1)
  sounds.vortexBang1:setVolume(0.1)
  
end


local function randplay(source, pitch, rand)
  source:stop()
  source:setPitch(pitch - rand + math.random() * 2 * rand)
  source:play()
end


sounds.init = init
sounds.randplay = randplay


return sounds