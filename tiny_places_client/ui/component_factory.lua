local button = require("ui/button")

local component_factory = {}

local function drawButton(bt)
  button.draw(bt.text, bt.x, bt.y, bt.toff, bt.scale, bt.pressed)
end

local function makeButton(text, x, y, toff, scale, callback)
  local button = {}

  button.text = text
  button.x = x
  button.y = y
  button.toff = toff
  button.scale = scale
  button.pressed = false
  button.callback = callback
  button.draw = drawButton
  return button
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
    if mx > element.x and my > element.y and mx < element.x + 115 and my < element.y+28 then
      element.pressed = true
      element.callback(element.pressed)
    end
  end
end

local function containerMouseReleased(container, mx, my)
  for i, element in pairs(container.store) do
    if mx > element.x and my > element.y and mx < element.x + 115 and my < element.y+28 then
      element.pressed = false
      element.callback(element.pressed)
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

component_factory.makeButton = makeButton
component_factory.makeContainer = makeContainer

return component_factory


