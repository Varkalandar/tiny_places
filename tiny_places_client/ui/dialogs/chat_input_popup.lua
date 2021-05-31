-- 
-- Chat text input and display popup
--
-- Author: Hj. Malthaner
-- Date: 2021/05/24
--


local utf8 = require("utf8")


local chatInputPopup = {}


local function init(mainUi)

  print("Loading chat input popup")
  chatInputPopup.mainUi = mainUi
  chatInputPopup.text = ""
  
  chatInputPopup.wingTL = love.graphics.newImage("resources/ui/silver/wing_topleft.png")
  chatInputPopup.wingTR = love.graphics.newImage("resources/ui/silver/wing_topright.png")
  chatInputPopup.wingBL = love.graphics.newImage("resources/ui/silver/wing_bottomleft.png")
  chatInputPopup.wingBR = love.graphics.newImage("resources/ui/silver/wing_bottomright.png")
end


local function update(dt)

end


local function draw()
  
  local w = 640
  local h = 480
  local xoff = (1200-w)/2
  local yoff = (720-h)/2
  local yspace = 28
  
  love.graphics.setColor(0.3, 0.3, 0.3, 0.5)
  love.graphics.rectangle("fill", xoff, yoff, w, h)
  love.graphics.setColor(0.9, 0.7, 0.4)
  love.graphics.rectangle("line", xoff, yoff, w, h)

  love.graphics.setColor(0.05, 0.07, 0.2, 0.5)
  love.graphics.rectangle("fill", xoff+16, yoff+16, w-32, h-32)


  love.graphics.setColor(1, 1, 1)
  local f = 0.75
  love.graphics.draw(chatInputPopup.wingTL, xoff+4, yoff+4, 0, f, f)
  love.graphics.draw(chatInputPopup.wingTR, xoff+w-4-chatInputPopup.wingTR:getWidth()*f, yoff+4, 0, f, f)
  love.graphics.draw(chatInputPopup.wingBL, xoff+4, yoff+h-2-chatInputPopup.wingTR:getHeight()*f, 0, f, f)
  love.graphics.draw(chatInputPopup.wingBR, xoff+w-4-chatInputPopup.wingTR:getWidth()*f, yoff+h-4-chatInputPopup.wingTR:getHeight()*f, 0, f, f)


  love.graphics.setColor(1.0, 0.97, 0.94)
  -- local font = chatInputPopup.mainUi.uifont
  local font = chatInputPopup.mainUi.pixfont
  font:drawBoxStringScaled(chatInputPopup.text, 
                           xoff + 24, yoff + 24, 
                           w-48, h-48, yspace, 0.25, 0.25)

end


local function mousePressed(button, mx, my)
end


local function mouseReleased(button, mx, my)
end


local function mouseDragged(button, mx, my)
end


local function keyReleased(key, scancode, isrepeat)

  chatInputPopup.text = chatInputPopup.text .. tip.inputtext
  tip.inputtext = ""
  
  if key == "backspace" then
    -- get the byte offset to the last UTF-8 character in the string.
    local byteoffset = utf8.offset(chatInputPopup.text, -1)

    if byteoffset then
      -- remove the last UTF-8 character.
      -- string.sub operates on bytes rather than UTF-8 characters, so we couldn't do string.sub(text, 1, -2).
      chatInputPopup.text = string.sub(chatInputPopup.text, 1, byteoffset - 1)
    end    
  end
  
  if key == "insert" and chatInputPopup.oldText then
    chatInputPopup.text = chatInputPopup.oldText
  end
  
end


chatInputPopup.init = init
chatInputPopup.update = update
chatInputPopup.draw = draw
chatInputPopup.mousePressed = mousePressed
chatInputPopup.mouseReleased = mouseReleased
chatInputPopup.mouseDragged = mouseDragged
chatInputPopup.keyReleased = keyReleased


return chatInputPopup
