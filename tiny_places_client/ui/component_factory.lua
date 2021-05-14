-- 
-- UI component factory
--
-- Author: Hj. Malthaner
-- Date: 2020/03/14
--


local button = require("ui/button")
local textinput = require("ui/textinput")
local colorinput = require("ui/colorinput")

local componentFactory = {initDone=false}


local function init()

  -- only init if not done yet
  if componentFactory.initDone == false then
    textinput.init()
    colorinput.init()
    componentFactory.initDone = true
  end
end


local function drawButton(bt)
  button.draw(bt)
end


local function drawInput(input)
  textinput.draw(input)
end


local function drawColor(input)
  colorinput.draw(input)
end


local function makeButton(text, pixfont, x, y, toff, scale, callback)
  local button = {}

  button.text = text
  button.pixfont = pixfont
  button.x = x
  button.y = y
  button.width = 562*scale
  button.height = 86*scale
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
      
      tip.sounds.randplay(tip.sounds.uiClick, 1, 0.1)
	  
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


componentFactory.init = init
componentFactory.makeButton = makeButton
componentFactory.makeInput = makeInput
componentFactory.makeColor = makeColor
componentFactory.makeContainer = makeContainer

return componentFactory


