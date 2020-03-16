local button = require("ui/button")
local textinput = require("ui/textinput")
local colorinput = require("ui/colorinput")

local component_factory = {}


local function init()
  textinput.init()
  colorinput.init()
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
    
    if mx > element.x and my > element.y and mx < element.x + 115 and my < element.y+28 then
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


