local button = {}

button.image = love.graphics.newImage("resources/ui/button.png")

local function draw(bt)

  -- text, pixfont, x, y, toff, scale, pressed
  
  love.graphics.setColor(1, 1, 1)
  love.graphics.draw(button.image, bt.x, bt.y, 0, bt.scale, bt.scale)

  if pressed then 
    -- love.graphics.setColor(1, 0.8, 0.4)
    love.graphics.setColor(1, 0.6, 0.2)    
  else 
    -- love.graphics.setColor(1.0*0.8, 0.8*0.8, 0.4*0.8)
    -- love.graphics.setColor(1.0*0.9, 0.8*0.9, 0.4*0.9)
    love.graphics.setColor(1.0, 0.8, 0.4)
  end

  -- love.graphics.print(text, x+10+toff, y+6, 0, 1.25, 1)
  local f = 0.66
  local w = bt.pixfont.calcStringWidth(bt.text) * f
  bt.pixfont.drawStringScaled(bt.text, 
                                             bt.x + (button.image:getWidth() - w) * bt.scale / 2,
                                             bt.y+1, 
                                             bt.scale * f)
end

button.draw = draw

return button

