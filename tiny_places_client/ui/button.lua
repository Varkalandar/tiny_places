local button = {}

button.image = love.graphics.newImage("resources/ui/button.png")

local function draw(text, x, y, toff, scale, pressed)
  love.graphics.setColor(1, 1, 1)
  love.graphics.draw(button.image, x, y, 0, scale, scale)

  if pressed then 
    -- love.graphics.setColor(1, 0.8, 0.4)
    love.graphics.setColor(1, 0.6, 0.2)    
  else 
    -- love.graphics.setColor(1.0*0.8, 0.8*0.8, 0.4*0.8)
    love.graphics.setColor(1.0*0.9, 0.8*0.9, 0.4*0.9)
  end

  love.graphics.print(text, x+10+toff, y+6, 0, 1.25, 1)
end

button.draw = draw

return button

