-- 
-- Server IP dialog
--
-- Author: Hj. Malthaner
-- Date: 2021/05/25
--

local cf = require("ui/component_factory")

-- UI element container for this UI
local container = cf.makeContainer()

local serverIpPopup = {}


local function serverCallback(x, y, pressed)
  if not pressed then 
    print("Connect pressed!")
    tip.settings.server_ip = serverIpPopup.serverInput.text
  end
end


local function init(mainUi)

  print("Initializing server ip popup")
  serverIpPopup.mainUi = mainUi
  
  local serverInput = cf.makeInput("127.0.0.1", mainUi.uifont, 220, 110, 360, 32, nil)
  container:add(serverInput)
  serverIpPopup.serverInput = serverInput
  
  local connectButton = cf.makeButton("Connect", mainUi.uifont, 190, 200, 0, 0.5, serverCallback)
  container:add(connectButton)
end


local function update(dt)

end


local function draw()
  
  local w = 640
  local h = 320
  local xoff = (1200-w)/2
  local yoff = (720-h)/2
  local yspace = 28
  local font = serverIpPopup.mainUi.uifont
  
  love.graphics.setColor(0.05, 0.1, 0.2, 0.5)
  love.graphics.rectangle("fill", xoff, yoff, w, h)
  love.graphics.setColor(0.9, 0.7, 0.4)
  love.graphics.rectangle("line", xoff, yoff, w, h)

  love.graphics.setColor(1, 1, 1)
  font:drawStringScaled("Connecting to server ...", xoff + 20, yoff + 20, 0.5, 0.5)

  font:drawStringScaled("Server IP:", xoff + 20, yoff + 110, 0.25, 0.25)
  
  if serverIpPopup.errorMessage then
    love.graphics.setColor(1, 0.5, 0)
    font:drawStringScaled(serverIpPopup.errorMessage, xoff + 20, yoff + 160, 0.25, 0.25)    
  end

  container:draw(xoff, yoff)
end


local function mousePressed(button, mx, my)
  container:mousePressed(mx, my)
end


local function mouseReleased(button, mx, my)
  container:mouseReleased(mx, my)
end


local function mouseDragged(button, mx, my)
end


local function keyReleased(key, scancode, isrepeat)
  container:keyReleased(key, scancode, isrepeat)
end


serverIpPopup.init = init
serverIpPopup.update = update
serverIpPopup.draw = draw
serverIpPopup.mousePressed = mousePressed
serverIpPopup.mouseReleased = mouseReleased
serverIpPopup.mouseDragged = mouseDragged
serverIpPopup.keyReleased = keyReleased


return serverIpPopup
