-- 
-- Textinput field related code
--
-- Author: Hj. Malthaner
-- Date: 2020/03/14
--


local utf8 = require("utf8")


local textinput = {}


function keyReleased(input, key, scancode, isrepeat)

  if input.focused then
  
    -- copy text from global buffer and clear the buffer
    input.text = input.text .. tip.inputtext
    tip.inputtext = ""
      
    if key == "return" then
      -- callback ?
      
    elseif key == "backspace" then
      -- get the byte offset to the last UTF-8 character in the string.
      local byteoffset = utf8.offset(input.text, -1)
 
      if byteoffset then
        -- remove the last UTF-8 character.
        -- string.sub operates on bytes rather than UTF-8 characters, so we couldn't do string.sub(text, 1, -2).
        input.text = string.sub(input.text, 1, byteoffset - 1)
      end    
    end
  end
end


local function init()
  textinput.focused = false
end


local function draw(input, xoff, yoff)

  local x = xoff+input.x
  local y = yoff+input.y
  
  if input.focused then
    love.graphics.setColor(0.2, 0.15, 0.1)
  else  
    love.graphics.setColor(0.1, 0.1, 0.1)
  end
      
  love.graphics.rectangle("fill", x, y, input.width, input.height)
  
  love.graphics.setColor(1, 1, 1)
  love.graphics.rectangle("line", x, y, input.width, input.height)

  local text = input.text
  
  if input.password then
    text = string.rep("*", text:len())
  end

  if input.focused then
    input.pixfont:drawStringScaled(text .. "|", x+4, y+2, 0.25, 0.25)    
  else
    input.pixfont:drawStringScaled(text, x+4, y+2, 0.25, 0.25)  
  end
end


textinput.init = init
textinput.draw = draw
textinput.keyReleased = keyReleased

return textinput
