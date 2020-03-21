-- 
-- UI component factory
--
-- Author: Hj. Malthaner
-- Date: 2020/03/14
--


local button = require("ui/button")
local textinput = require("ui/textinput")
local colorinput = require("ui/colorinput")

local component_factory = {}

local clickSoundData = nil
local clickSoundSource = nil


local function init()

  -- only init if not done yet
  if clickSoundData == nil then
    textinput.init()
    colorinput.init()
  
    clickSoundData = love.sound.newSoundData("resources/sfx/hard_click.wav")
    clickSoundSource = love.audio.newSource(clickSoundData)
    clickSoundSource:setVolume(0.2)
  end
end


local function drawButton(bt)
  button.draw(bt.text, bt.x, bt.y, bt.toff, bt.scale, bt.pressed)
end

local function drawInput(input)
  textinput.draw(input)
end

local function drawColor(input)
  colorinput.draw(input)
end


local function makeButton(text, x, y, toff, scale, callback)
  local button = {}

  button.text = text
  button.x = x
  button.y = y
  button.width = 115
  button.height = 28
  button.toff = toff
  button.scale = scale
  button.pressed = false
  button.focused = false
  button.callback = callback
  button.draw = drawButton
  return button
end


local function makeInput(text, x, y, width, height, callback)
  local input = {}

  input.text = text
  input.x = x
  input.y = y
  input.width = width
  input.height = height
  input.pressed = false
  input.focused = false
  input.callback = callback
  input.draw = drawInput
  return input
end


local function makeColor(x, y, callback)
  local input = {}

  input.x = x
  input.y = y
  input.width = 256
  input.height = 256
  input.pressed = false
  input.focused = false
  input.callback = callback
  input.draw = drawColor
  input.handleClick = colorinput.handleClick
  return input
end

local function drawContainer(container)
  for i, element in pairs(container.store) do
    element:draw()
  end
end

local function containerAdd(container, element)
  table.insert(container.store, element)
end

local function containerMousePressed(container, mx, my)
  for i, element in pairs(container.store) do
    element.focused = false
    
    if mx > element.x and my > element.y and mx < element.x + element.width and my < element.y + element.height then
      
      local success = clickSoundSource:play()
      if not success then print("Playing click sound failed!") end 
    
      element.pressed = true
      element.focused = true
      
      if element.callback then
        element.callback(mx, my, element.pressed)
      end
    end
  end
end

local function containerMouseReleased(container, mx, my)
  for i, element in pairs(container.store) do
    if mx > element.x and my > element.y and mx < element.x + element.width and my < element.y + element.height then
      element.pressed = false
      if element.callback then
        element.callback(mx, my, element.pressed)
      end
    end
  end
end

local function makeContainer()
  local container = {}
  container.draw = drawContainer
  container.add = containerAdd
  container.mousePressed = containerMousePressed
  container.mouseReleased = containerMouseReleased
  container.store = {}
  return container
end

component_factory.init = init
component_factory.makeButton = makeButton
component_factory.makeInput = makeInput
component_factory.makeColor = makeColor
component_factory.makeContainer = makeContainer

return component_factory


