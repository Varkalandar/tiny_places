-- 
-- "Tiny Places" startup file
--
-- Author: Hj. Malthaner
-- Date: 2020/03/08
--

local mainUi = require("main_ui")

-- all init code goes here
function love.load()
  mainUi.init()      

  local flags = {vsync = true}
  success = love.window.setMode(1200, 720, flags)
  if(not success) then
    print("Failed to resize main window")
  end
  
  love.window.setTitle("Tiny Places v0.02")
end

-- the work that has to be done before each frame can be drawn
-- dt is a float, measuring in seconds
function love.update(dt)
  mainUi.update(dt)
end

-- draw the frame
function love.draw()
  mainUi.draw()
end
