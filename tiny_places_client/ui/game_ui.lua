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

  gameUi.map.clientSocket.send("MOVE,"
                               ..mob.id..","
                               ..layer..","
                               ..x..","
                               ..y
                               )
end


local function init(mainUi, map)
  print("Loading game ui")
  
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
  
  container:draw()
end


local function mousePressed(button, mx, my)

  if isMapArea(mx, my) then  
    sendMove(gameUi.playerMob, 3, mx, my)
  else
    container:mousePressed(mx, my)
  end
end


local function mouseReleased(button, mx, my)
  if isMapArea(mx, my) then  
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
