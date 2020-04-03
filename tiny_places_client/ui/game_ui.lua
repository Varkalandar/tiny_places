-- 
-- Game UI
--
-- Author: Hj. Malthaner
-- Date: 2020/03/22
--

local cf = require("ui/component_factory")

local gameUi = {}

-- UI element container for this UI
local container = cf.makeContainer()

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
  local ptype = 1
	map.clientSocket.send("FIRE,"
											 ..layer..","
											 ..ptype..","
											 ..x..","
											 ..y..","
											 )

  map.sounds.fireballLaunch:stop()
  map.sounds.fireballLaunch:setPitch(0.8 + math.random() * 0.4)
  map.sounds.fireballLaunch:play()
end


local function init(mainUi, map)
	print("Loading game ui")
	
  gameUi.gaugeFg = love.graphics.newImage("resources/ui/gauge_fg.png")
  gameUi.gaugeBg = love.graphics.newImage("resources/ui/gauge_bg.png")
  gameUi.gaugeRed = love.graphics.newImage("resources/ui/gauge_red.png")
  gameUi.gaugeBlue = love.graphics.newImage("resources/ui/gauge_blue.png")
  
	cf.init()
	gameUi.map = map
	
	-- add player to the map
	gameUi.map.clientSocket.send("ADDP," -- add a player
														.."3,"
														.."1,"  -- tile id
														.."600,"   -- x pos
														.."400,"   -- y pos
														.."1.0,"   -- scale factor
														.."1.0 1.0 1.0 1.0"
														)  
														
	-- this will set gameUi.playerMob when receiving the data from the server
end


local function update(dt)

end


local function draw()
	love.graphics.setColor(1.0, 1.0, 1.0)
	love.graphics.print("Game Mode", 16, 30, 0, 2, 2)
	
	love.graphics.print("Wasteland", 1000, 30, 0, 2, 1)

  
	love.graphics.print("40/40", 600 - 184, 596, 0, 1, 0.5)
	love.graphics.print("14/20", 600 + 146, 596, 0, 1, 0.5)
	love.graphics.setColor(1.0*0.9, 0.8*0.9, 0.4*0.9)
	love.graphics.print("Life", 600 - 188, 576, 0, 2, 1)
	love.graphics.print("Mana", 600 + 132, 576, 0, 2, 1)


  local scale = 0.33
  local w = gameUi.gaugeBg:getWidth() * scale
  local h = gameUi.gaugeBg:getHeight() * scale

  local beat = math.sin(love.timer.getTime()  * 1.5)
  local beat = math.abs(beat)
  
  local top = 610
  local shrink = 0.98 + beat * 0.02
  
	love.graphics.setColor(1.0, 1.0, 1.0)
  love.graphics.draw(gameUi.gaugeBg, 600 - 100 - w, top, 0, scale, scale)
  love.graphics.draw(gameUi.gaugeRed, 600 - 100  - w + (w - w*shrink)*0.5, 
                                      top + h - h*shrink - (1-shrink) * 10,
                                      0, 
                                      scale * shrink, 
                                      scale * shrink)
  love.graphics.draw(gameUi.gaugeFg, 600 - 100 - w, top, 0, scale, scale)

  local shrink = 0.7 + beat * 0.02
  
  love.graphics.draw(gameUi.gaugeBg, 600 + 100 , top, 0, scale, scale)
  love.graphics.draw(gameUi.gaugeBlue, 600 + 100 + (w-w*shrink)*0.5, 
                                       top + h - h*shrink - (1-shrink) * 10,
                                       0,
                                       scale * shrink, 
                                       scale * shrink)
  love.graphics.draw(gameUi.gaugeFg, 600 + 100 , top, 0, scale, scale)
  
	container:draw()
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


gameUi.init = init;
gameUi.update = update;
gameUi.draw = draw;
gameUi.mousePressed = mousePressed;
gameUi.mouseReleased = mouseReleased;
gameUi.mouseDragged = mouseDragged;


return gameUi
