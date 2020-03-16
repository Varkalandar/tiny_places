-- 
-- Textinput field related code
--
-- Author: Hj. Malthaner
-- Date: 2020/03/14
--

local utf8 = require("utf8")

local textinput = {}


function love.textinput(t)
  if textinput.focusedInput then
    local input = textinput.focusedInput
    input.text = input.text .. t
  end
end


function love.keypressed(key, scancode, isrepeat)

  if textinput.focusedInput then
    local input = textinput.focusedInput
  
    if key == "return" then
      -- finish input, make field lose focus
      textinput.focusedInput.focused = false
      textinput.focusedInput = nil
      
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
  textinput.focusedInput = nil
end


local function drawInput(input)
  love.graphics.setColor(1, 1, 1)
  love.graphics.rectangle("line", input.x, input.y, input.width, input.height)
  
  if input.focused then
    love.graphics.print(input.text .. "|", input.x+10, input.y+6, 0, 1.25, 1)    
  else
    love.graphics.print(input.text, input.x+10, input.y+6, 0, 1.25, 1)  
  end

  if input.focused then
    textinput.focusedInput = input
  end  
end

textinput.init = init
textinput.draw = drawInput

return textinput
