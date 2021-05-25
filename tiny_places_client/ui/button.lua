local button = {}

button.image = love.graphics.newImage("resources/ui/silver/button.png")

local function draw(bt, xoff, yoff)

  local x = bt.x + xoff
  local y = bt.y + yoff
  
  -- text, pixfont, x, y, toff, scale, pressed
  
  love.graphics.setColor(1, 1, 1)
  love.graphics.draw(button.image, x, y, 0, bt.scale, bt.scale)

  if pressed then 
    -- love.graphics.setColor(1, 0.8, 0.4)
    love.graphics.setColor(1, 0.6, 0.2)    
  else 
    -- love.graphics.setColor(1.0*0.8, 0.8*0.8, 0.4*0.8)
    -- love.graphics.setColor(1.0*0.9, 0.8*0.9, 0.4*0.9)
    love.graphics.setColor(1.0, 0.8, 0.4)
  end

  -- love.graphics.print(text, x+10+toff, y+6, 0, 1.25, 1)

  local w = bt.pixfont:calcStringWidth(bt.text) * 0.75
  bt.pixfont:drawStringScaled(bt.text, 
                              x + (button.image:getWidth() - w) * bt.scale / 2,
                              y+5, 
                              bt.scale * 0.75, 
                              bt.scale * 0.5)
end

button.draw = draw

return button

