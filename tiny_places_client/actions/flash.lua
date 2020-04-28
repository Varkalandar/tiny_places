--
-- Draw a flash of light
--
-- Author: Hj. Malthaner
-- Date: 2020/04/04
--

local flashes = {}


local function update(flash, dt)
  flash.age = flash.age + dt * flash.timelapse
  flash.done = flash.age > 4
end


local function draw(flash)
  local scale = (0.9 + flash.age * 0.4) * flash.scale
  
  local mode, alphamode = love.graphics.getBlendMode()
  love.graphics.setBlendMode(flash.mode, "alphamultiply")
  
  local fade = 1 / (1+flash.age*4)
  
  love.graphics.setColor(flash.r*fade, flash.g*fade, flash.b*fade, fade)
  love.graphics.draw(flash.image, 
                     flash.x - flash.image:getWidth() * scale * 0.5, 
                     flash.y - flash.image:getHeight() * scale * 0.5, 
                     0, 
                     scale, scale)

  love.graphics.setBlendMode(mode, alphamode)
end


local function new(x, y, image, scale, r, g, b, mode, under, timelapse)
  local newflash = 
  {
    x = x, 
    y = y, 
    r = r,
    g = g,
    b = b,
    mode = mode,
    image = image,
    scale = scale,
    timelapse = timelapse,
    age = 0,
    done = false,
    drawOver = drawOver,
    update = update
  }
  
  if under then
    newflash.drawUnder = draw  
  else
    newflash.drawOver = draw
  end
  
  return newflash
end

flashes.new = new

return flashes