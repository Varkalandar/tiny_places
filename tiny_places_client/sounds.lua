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

  local vortexBangData = love.sound.newSoundData("resources/sfx/vortex_bang.mp3")
  local vortexBangData1 = love.sound.newSoundData("resources/sfx/vortex_bang_1.wav")
  local vortexBangData2 = love.sound.newSoundData("resources/sfx/vortex_bang_2.wav")

  local noisedChirpData = love.sound.newSoundData("resources/sfx/noised_chirp.wav")

  local clickSoundData = love.sound.newSoundData("resources/sfx/hard_click.wav")
  
  sounds.uiClick = love.audio.newSource(clickSoundData)
  sounds.uiClick:setVolume(0.2)
  
  sounds.fireballLaunch = love.audio.newSource(fireballLaunchData)
  sounds.fireballLaunch:setVolume(0.15)

  sounds.fireballHit1 = love.audio.newSource(fireballHitData1)
  sounds.fireballHit1:setVolume(0.4)

  sounds.fireballHit2 = love.audio.newSource(fireballHitData2)
  sounds.fireballHit2:setVolume(0.4)

  sounds.debrisHit1 = love.audio.newSource(debrisHitData1)
  sounds.debrisHit1:setVolume(0.05)

  sounds.debrisHit2 = love.audio.newSource(debrisHitData2)
  sounds.debrisHit2:setVolume(0.05)

  sounds.vortexBang = love.audio.newSource(vortexBangData)
  sounds.vortexBang:setVolume(0.15)

  sounds.vortexBang1 = love.audio.newSource(vortexBangData1)
  sounds.vortexBang1:setVolume(0.15)

  sounds.vortexBang2 = love.audio.newSource(vortexBangData1)
  sounds.vortexBang2:setVolume(0.15)
  
  sounds.noisedChirp = love.audio.newSource(noisedChirpData)
  sounds.noisedChirp:setVolume(0.15)
end


local function randplay(source, pitch, rand)
  source:stop()
  source:setPitch(pitch - rand + math.random() * 2 * rand)
  source:play()
end


local function randplay2(source1, source2, factor, pitch, rand)
  if math.random() < factor then
    randplay(source1, pitch, rand)
  else
    randplay(source2, pitch, rand)
  end  
end


sounds.init = init
sounds.randplay = randplay
sounds.randplay2 = randplay2


return sounds