-- 
-- Game UI
--
-- Author: Hj. Malthaner
-- Date: 2020/03/22
--

local cf = require("ui/component_factory")
local inventoryPopup = require("ui/dialogs/inventory_popup")

local gameUi = {}

-- UI element container for this UI
local container = cf.makeContainer()


local function switchToEditorUi()
  gameUi.mainUi.ui = gameUi.mainUi.editorUi
end

--
-- Checks if the given screen coordinate is inside the map area
--
local function isMapArea(mx, my)

  -- normalize coordinates
  mx = mx - 600
  my = my - 28

  local mi = mx + my * 2
  local mj = -mx + my * 2 
  -- print("mi=" .. mi .. " mj=" .. mj)
  
  return mi >= 0 and mi <= 1176 and mj >= 0 and mj <= 1176
end


local function sendMove(mob, layer, x, y)

  local map = gameUi.map
  
  map.clientSocket.send("MOVE,"
                       ..mob.id..","
                       ..layer..","
                       ..x..","
                       ..y
                       )
end


local function fireProjectile(mob, layer, x, y)

  local map = gameUi.map
  local ptype = "fireball"
  map.clientSocket.send("FIRE,"
                       ..layer..","
                       ..ptype..","
                       ..x..","
                       ..y..","
                       )

  tip.sounds.fireballLaunch:stop()
  tip.sounds.fireballLaunch:setPitch(0.8 + math.random() * 0.4)
  tip.sounds.fireballLaunch:play()
end


local function init(mainUi, map)
  print("Loading game ui")
  
  inventoryPopup.init(mainUi, map.itemSet, map.playerInventory)
  
  gameUi.mainUi = mainUi
  gameUi.areaImage = love.graphics.newImage("resources/ui/area_cut.png")
  
  gameUi.gaugeFg = love.graphics.newImage("resources/ui/gauge_fg.png")
  gameUi.gaugeBg = love.graphics.newImage("resources/ui/gauge_bg.png")
  gameUi.gaugeRed = love.graphics.newImage("resources/ui/gauge_red.png")
  gameUi.gaugeBlue = love.graphics.newImage("resources/ui/gauge_blue.png")
  
  cf.init()
  gameUi.map = map
  
end


local function update(dt)

end


local function drawGauge(x, y, filler, shrink, title, min, max, value)
  local pixfont = gameUi.mainUi.pixfont

  love.graphics.setColor(1.0, 1.0, 1.0, 1.0)

  local scale = 0.33
  local w = gameUi.gaugeBg:getWidth() * scale
  local h = gameUi.gaugeBg:getHeight() * scale
  local numbers = value .. "/" .. max
  
    -- love.graphics.print(numbers, x + 5, y+20, 0, 1, 0.5)
  local nw = pixfont:calcStringWidth(numbers) * 0.25
  pixfont:drawStringScaled(numbers, x + (w - nw)/2, y+4, 0.25, 0.25)

  love.graphics.draw(gameUi.gaugeBg, x, y+30, 0, scale, scale)
  love.graphics.draw(filler, x + (w - w*shrink)*0.5, 
                                      y + 30 + h - h*shrink - (1-shrink) * 10,
                                      0, 
                                      scale * shrink, 
                                      scale * shrink)
  love.graphics.draw(gameUi.gaugeFg, x, y+30, 0, scale, scale)

  
  love.graphics.setColor(1.0*0.9, 0.8*0.9, 0.4*0.9)
  -- love.graphics.print(title, x + 5, y, 0, 2, 1)
  local tw = pixfont:calcStringWidth(title) * 0.25
  pixfont:drawStringScaled(title, x + (w - tw)/2 , y+94, 0.25, 0.25)
end


local function draw()
  local pixfont = gameUi.mainUi.pixfont

  love.graphics.setColor(0, 0, 0)
  pixfont:drawStringScaled("Game Mode", 16-10, 30+24, 0.5, 0.25, 0.2, 0)
  love.graphics.setColor(1.0, 1.0, 1.0)
  pixfont:drawStringScaled("Game Mode", 16, 30, 0.5, 0.5)

  local pos = 1200 - 16 - pixfont:calcStringWidth(gameUi.map.name) * 0.5
  love.graphics.setColor(0, 0, 0)
  pixfont:drawStringScaled(gameUi.map.name, pos-10, 30+24, 0.5, 0.25, 0.2, 0)
  love.graphics.setColor(1.0, 1.0, 1.0)
  pixfont:drawStringScaled(gameUi.map.name, pos, 30, 0.5, 0.5)

  local beat = math.sin(love.timer.getTime()  * 1.5)
  local beat = math.abs(beat)
  
  local stats = tip.player.stats

  local shrink = 0.98 * (stats[0].value / stats[0].max) + beat * 0.02
  drawGauge(5, 360, gameUi.gaugeRed, shrink, 
            "Structure", 
            stats[0].min, stats[0].max, stats[0].value) 

  local shrink = 0.98 * (stats[1].value / stats[1].max) + beat * 0.02
  drawGauge(140, 430, gameUi.gaugeBlue, shrink, 
        "Energy",
        stats[1].min, stats[1].max, stats[1].value) 

  container:draw(0, 0)
end


local function mousePressed(button, mx, my)

  if isMapArea(mx, my) then
    
    if(button == 1) then  
      sendMove(gameUi.playerMob, 3, mx, my)
    else
      -- make the projectile appear above the ground
      gameUi.playerMob.zOff = 12;
      fireProjectile(gameUi.playerMob, 3, mx, my)
    end
    
  else
    container:mousePressed(mx, my)
  end
end


local function mouseReleased(button, mx, my)
  if isMapArea(mx, my) then
    gameUi.playerMob.zOff = 0
  else
    container:mouseReleased(mx, my)
  end
end


local function mouseDragged(button, mx, my)

  if isMapArea(mx, my) then  
  end
end


local function keyReleased(key, scancode, isrepeat)

  if key == "i" then
    gameUi.mainUi.popup = inventoryPopup
  end
  
  if key == "escape" then
    switchToEditorUi()
  end
end


gameUi.init = init
gameUi.update = update
gameUi.draw = draw
gameUi.mousePressed = mousePressed
gameUi.mouseReleased = mouseReleased
gameUi.mouseDragged = mouseDragged
gameUi.keyReleased = keyReleased


return gameUi
