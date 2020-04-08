--
-- Draw a flash of light
--
-- Author: Hj. Malthaner
-- Date: 2020/04/04
--

local flashes = {}


local function new(x, y, image, r, g, b)
  local newflash = 
  {
    x=x, 
    y=y, 
    r=r,
    g=g,
    b=b,
    image = image,
    age = 0,
    done = false,
    draw = flashes.draw,
    update = flashes.update
  }
  return newflash
end


local function update(flash, dt)
  flash.age = flash.age + dt
  flash.done = flash.age > 4
end


local function draw(flash)
  local scale = 0.9 + flash.age * 0.4
  
  local mode, alphamode = love.graphics.getBlendMode()
  love.graphics.setColor(flash.r, flash.g, flash.b, 1 / (1+flash.age*8))
  love.graphics.setBlendMode("add", "alphamultiply")
  love.graphics.draw(flash.image, 
                     flash.x - flash.image:getWidth() * scale * 0.5, 
                     flash.y - flash.image:getHeight() * scale * 0.5, 
                     0, 
                     scale, scale)

  love.graphics.setBlendMode(mode, alphamode)
end


flashes.new = new
flashes.update = update
flashes.draw = draw

return flashes