-- 
-- Display for text chat messages
--
-- Author: Hj. Malthaner
-- Date: 2021/05/28
--


local chat_display = {}


local function init(font)
	
  chat_display.font = font
  chat_display.canvas = love.graphics.newCanvas(600, 1200)
  chat_display.canvas_bottom = 0
  
  chat_display.history = {}
  chat_display.history.sender = {}
  chat_display.history.color = {}
  chat_display.history.message = {}
  chat_display.history.next = 0
  chat_display.history.length = 16


  for i = 0, chat_display.history.length-1 do
    chat_display.history.sender[i] = ""
    chat_display.history.color[i] = {r=1, g=1, b=1, a=1}
    chat_display.history.message[i] = ""
  end
  
end


local function unmarshallColor(color)
  if color then
    -- components are space separated
    local iter = color:gmatch("[^ ]+")

    local r = tonumber(iter())
    local g = tonumber(iter())
    local b = tonumber(iter())
    local a = tonumber(iter())

    return {r=r, b=b, g=g, a=a}
  else
    -- backwards compatibility - old objects had no color data
    return {r=1, g=1, b=1, a=1}
  end
end


local function refreshCanvas()

  local scale = 0.2
  local font = chat_display.font
  local next = chat_display.history.next
  local yspace = 8
  local left = 16
  
  love.graphics.setCanvas(chat_display.canvas)
  love.graphics.clear(0.1, 0.1, 0.1, 0.1)
  -- love.graphics.clear()
  love.graphics.setBlendMode("alpha")

  for i = 0, chat_display.history.length-1 do
  
    local ni = (next + i) % chat_display.history.length
    
    local color = chat_display.history.color[ni]
    local sender = chat_display.history.sender[ni]
    
    love.graphics.setColor(color.r, color.g, color.b, 1)
    font:drawStringScaled(sender, left, yspace, scale, scale)
    
    local indent = font:calcStringWidth(sender) * scale
    
    local text = chat_display.history.message[ni]
    
    love.graphics.setColor(1, 1, 1, 1)
    local lines = font:drawBoxStringScaled(text, left+indent, yspace, 450, 600, 24, scale, scale)
    
    yspace = yspace + lines * 24 + 8
    
  end

  chat_display.canvas_bottom = yspace
  -- print("b=" .. chat_display.canvas_bottom)

  love.graphics.setCanvas()
end


local function addChatMessage(sender, color, message)

  print("Adding s=" .. sender .. "c=" .. color .. " m=" .. message)

  local h = chat_display.history
  
  h.sender[h.next] = sender .. ": "
  h.color[h.next] = unmarshallColor(color)
  h.message[h.next] = message:gsub("\\n", "\n")

  -- ringbuffer
  h.next = h.next + 1
  if h.next >= chat_display.history.length then
    h.next = 0
  end
  
  refreshCanvas()
end


local function draw(xoff, yoff)

  local x, y, width, height = love.graphics.getScissor( )

  local h=220
  love.graphics.setScissor(xoff, yoff, 1200-xoff, h)
  love.graphics.setColor(1, 1, 1, 1)
  
  love.graphics.draw(chat_display.canvas, 
                    xoff, yoff + h - chat_display.canvas_bottom)

  love.graphics.setScissor(x, y, width, height)
end


chat_display.draw = draw
chat_display.addChatMessage = addChatMessage
chat_display.init = init


return chat_display

