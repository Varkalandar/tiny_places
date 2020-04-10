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
	
  gameUi.areaImage = love.graphics.newImage("resources/ui/area_cut.png")
  
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


local function drawGauge(x, y, filler, shrink, title, numbers)

	love.graphics.setColor(1.0, 1.0, 1.0, 1.0)

  local scale = 0.33
  local w = gameUi.gaugeBg:getWidth() * scale
  local h = gameUi.gaugeBg:getHeight() * scale

  love.graphics.draw(gameUi.gaugeBg, x, y+30, 0, scale, scale)
  love.graphics.draw(filler, x + (w - w*shrink)*0.5, 
                                      y + 30 + h - h*shrink - (1-shrink) * 10,
                                      0, 
                                      scale * shrink, 
                                      scale * shrink)
  love.graphics.draw(gameUi.gaugeFg, x, y+30, 0, scale, scale)

 	love.graphics.print(numbers, x + 5, y+20, 0, 1, 0.5)
	love.graphics.setColor(1.0*0.9, 0.8*0.9, 0.4*0.9)
	love.graphics.print(title, x + 5, y, 0, 2, 1)

end


local function draw()
	love.graphics.setColor(1.0, 1.0, 1.0)
	love.graphics.print("Game Mode", 16, 30, 0, 2, 2)
	
	love.graphics.print(gameUi.map.name, 1000, 30, 0, 2, 1)

  local beat = math.sin(love.timer.getTime()  * 1.5)
  local beat = math.abs(beat)
  
  local shrink = 0.98 + beat * 0.02

  drawGauge(5, 360, gameUi.gaugeRed, shrink, "Life", "40/40") 

  local shrink = 0.7 + beat * 0.02
  
  drawGauge(140, 430, gameUi.gaugeBlue, shrink, "Mana", "16/40") 

	love.graphics.setColor(1.0, 1.0, 1.0)
  love.graphics.draw(gameUi.areaImage, 840, 500, 0, 0.5, 0.5)
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
