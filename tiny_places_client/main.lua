-- 
-- "Tiny Places" startup file
--
-- Author: Hj. Malthaner
-- Date: 2020/03/08
--


-- global values all go into the "tip" table
tip = 
{
  settings = require("settings"),
  sounds = require("sounds"),
  player = {stats = {}},
  
  -- text input will be buffered here, see code in mainUi
  inputtext = ""  
}
-- globals end


local mainUi = require("main_ui")


-- all init code goes here
function love.load()

  -- love.graphics.setDefaultFilter("linear", "linear", 8)
  -- love.graphics.setDefaultFilter("nearest", "nearest", 8)
  love.keyboard.setKeyRepeat(true)
  
  tip.settings.init()
  tip.sounds.init()
  
  mainUi.init()      

  local flags = {vsync = true}
  success = love.window.setMode(1200, 720, flags)
  if(not success) then
    print("Failed to resize main window")
  end
  
  love.window.setTitle("Tiny Places v0.05")
end


-- dt is a float, measuring in seconds
function love.update(dt)
  mainUi.update(dt)
end


-- draw the frame
function love.draw()
  mainUi.draw()
end
