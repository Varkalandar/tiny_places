--
-- Draw a sequence of images as animation
--
-- Author: Hj. Malthaner
-- Date: 2020/04/18
--

local animations = {}


local function update(animation, dt)
  animation.age = animation.age + dt
  animation.done = animation.age > animation.playtime
end


local function drawOver(animation)

  local mode, alphamode = love.graphics.getBlendMode()
  love.graphics.setColor(animation.r, animation.g, animation.b, animation.a)
  love.graphics.setBlendMode(animation.mode, "alphamultiply")

  local frame = math.floor(animation.sf + animation.age / animation.time)
  
  local tile = animation.tileset[frame]
  local image = tile.image
  local scalex, scaley = animation.scalef(animation.age / animation.playtime)
  
  love.graphics.draw(image, 
                     animation.x - image:getWidth() * scalex * 0.5, 
                     animation.y - image:getHeight() * scaley * 0.5, 
                     0, 
                     scalex, scaley)

  love.graphics.setBlendMode(mode, alphamode)
end


local function new(x, y, tileset, scalef, sf, ef, time, r, g, b, a, mode)
  local animation = 
  {
    x=x, 
    y=y, 
    scalef=scalef,
    r=r,
    g=g,
    b=b,
    a=a,
    mode=mode,
    tileset = tileset,
    sf=sf,
    ef=ef,
    age = 0,
    time = time,
    done = false,
    playtime = (ef - sf + 1) * time,
    drawOver = drawOver,
    update = update
  }
  return animation
end


animations.new = new

return animations