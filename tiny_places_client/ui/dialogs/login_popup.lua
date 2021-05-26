-- 
-- Login dialog
--
-- Author: Hj. Malthaner
-- Date: 2021/05/25
--

local cf = require("ui/component_factory")

-- UI element container for this UI
local container = cf.makeContainer()

local loginPopup = {}


local function loginChatCatcher(name, message)

  if message == "successful" then
    loginPopup.mainUi.popup = nil
    loginPopup.clientSocket.send("LOAD,lobby")
  else  
    loginPopup.errorMessage = message
  end
end


local function loginCallback(x, y, pressed)

  loginPopup.mainUi.chatCatcher = loginChatCatcher

  if not pressed then 
    print("Login pressed!")
    
    local name = loginPopup.accountNameInput.text
    local hash = love.data.hash("sha256", 
                                loginPopup.accountPassInput.text)
    
    local pass = love.data.encode("string", "hex", hash)
    
    print("name=" .. name .. " pass=" .. pass)
    
    loginPopup.clientSocket.send("HELO," .. name .. "," .. pass)
  end
end


local function createCallback(x, y, pressed)
  if not pressed then 
    print("Create pressed!")
    local mainUi = loginPopup.mainUi
    mainUi.popup = mainUi.newAccountPopup
  end
end


local function init(mainUi, clientSocket)

  print("Initializing login popup")
  loginPopup.mainUi = mainUi
  loginPopup.clientSocket = clientSocket
  
  local accountNameInput = cf.makeInput("Test", mainUi.uifont, 220, 110, 360, 32, nil)
  container:add(accountNameInput)
  loginPopup.accountNameInput = accountNameInput

  local accountPassInput = cf.makeInput("Test", mainUi.uifont, 220, 150, 360, 32, nil)
  container:add(accountPassInput)
  loginPopup.accountPassInput = accountPassInput
  
  local createButton = cf.makeButton("Login", mainUi.uifont, 190, 240, 0, 0.5, loginCallback)
  container:add(createButton)
  
  local createButton = cf.makeButton("Create Account", mainUi.uifont, 300, 345, 0, 0.4, createCallback)
  container:add(createButton)  
end


local function update(dt)

  -- newAccountPopup.text = newAccountPopup.text .. newAccountPopup.mainUi.inputtext
  -- newAccountPopup.mainUi.inputtext = ""
end


local function draw()
  
  local w = 640
  local h = 410
  local xoff = (1200-w)/2
  local yoff = (720-h)/2
  local yspace = 28
  local font = loginPopup.mainUi.uifont
  
  love.graphics.setColor(0.05, 0.1, 0.2, 0.5)
  love.graphics.rectangle("fill", xoff, yoff, w, h)
  love.graphics.setColor(0.9, 0.7, 0.4)
  love.graphics.rectangle("line", xoff, yoff, w, h)

  love.graphics.setColor(1, 1, 1)
  font:drawStringScaled("Log In To Tiny Places!", xoff + 20, yoff + 20, 0.5, 0.5)

  font:drawStringScaled("Account Name:", xoff + 20, yoff + 110, 0.25, 0.25)
  font:drawStringScaled("Password:", xoff + 20, yoff + 150, 0.25, 0.25)

  font:drawStringScaled("No account yet?", xoff + 100, yoff + 350, 0.25, 0.25)
  
  if loginPopup.errorMessage then
    love.graphics.setColor(1, 0.5, 0)
    font:drawStringScaled(loginPopup.errorMessage, xoff + 20, yoff + 200, 0.25, 0.25)    
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


loginPopup.init = init
loginPopup.update = update
loginPopup.draw = draw
loginPopup.mousePressed = mousePressed
loginPopup.mouseReleased = mouseReleased
loginPopup.mouseDragged = mouseDragged
loginPopup.keyReleased = keyReleased


return loginPopup
