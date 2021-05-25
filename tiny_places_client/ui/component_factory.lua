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


local function drawColor(input)
  colorinput.draw(input)
end


local function makeButton(text, pixfont, x, y, toff, scale, callback)
  local bt = {}

  bt.text = text
  bt.pixfont = pixfont
  bt.x = x
  bt.y = y
  bt.width = 562*scale
  bt.height = 86*scale
  bt.toff = toff
  bt.scale = scale
  bt.pressed = false
  bt.focused = false
  bt.callback = callback
  bt.draw = button.draw
  
  return bt
end


local function makeInput(text, pixfont, x, y, width, height, callback)
  local input = {}

  input.text = text
  input.pixfont = pixfont
  input.x = x
  input.y = y
  input.width = width
  input.height = height
  input.hasFocus = false
  input.callback = callback
  input.draw = textinput.draw
  input.keyReleased = textinput.keyReleased
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


local function drawContainer(container, x, y)
  
  container.x = x
  container.y = y
  
  for i, element in pairs(container.store) do
    element:draw(x, y)
  end
end


local function containerAdd(container, element)
  table.insert(container.store, element)
end


local function containerMousePressed(container, mx, my)
  
  local x = mx - container.x
  local y = my - container.y

  for i, element in pairs(container.store) do
    element.focused = false
    
    if x > element.x and y > element.y and x < element.x + element.width and y < element.y + element.height then
      
      tip.sounds.randplay(tip.sounds.uiClick, 1, 0.1)

      if element.focused == false then
        -- clear global text buffer, a new element was selected
        tip.textinput = ""
      end
	  
      element.pressed = true
      element.focused = true
      
      if element.callback then
        element.callback(mx, my, element.pressed)
      end
    end
  end
end


local function containerMouseReleased(container, mx, my)
  
  local x = mx - container.x
  local y = my - container.y

  for i, element in pairs(container.store) do
    if x > element.x and y > element.y and x < element.x + element.width and y < element.y + element.height then
      element.pressed = false
      if element.callback then
        element.callback(mx, my, element.pressed)
      end
    end
  end
end


local function containerKeyReleased(container, key, scancode, isrepeat)
  for i, element in pairs(container.store) do
    if element.focused and element.keyReleased then
      element:keyReleased(key, scancode, isrepeat)
    end
  end
end


local function makeContainer()
  local container = {}
  container.x = 0
  container.y = 0
  container.draw = drawContainer
  container.add = containerAdd
  container.mousePressed = containerMousePressed
  container.mouseReleased = containerMouseReleased
  container.keyReleased = containerKeyReleased
  container.store = {}
  return container
end


componentFactory.init = init
componentFactory.makeButton = makeButton
componentFactory.makeInput = makeInput
componentFactory.makeColor = makeColor
componentFactory.makeContainer = makeContainer

return componentFactory


