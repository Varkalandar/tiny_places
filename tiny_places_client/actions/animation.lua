--
-- Draw a sequence of images as animation
--
-- Author: Hj. Malthaner
-- Date: 2020/04/18
--

local animations = {}


local function new(x, y, tileset, scale, sf, ef, time, r, g, b, a)
  local animation = 
  {
    x=x, 
    y=y, 
    scale=scale,
    r=r,
    g=g,
    b=b,
    a=a,
    tileset = tileset,
    sf=sf,
    ef=ef,
    age = 0,
    time = time,
    done = false,
    draw = animations.draw,
    update = animations.update
  }
  return animation
end


local function update(animation, dt)
  animation.age = animation.age + dt
  
  playtime = (animation.ef - animation.sf + 1) * animation.time
  
  animation.done = animation.age > playtime
end


local function draw(animation)

  local mode, alphamode = love.graphics.getBlendMode()
  love.graphics.setColor(animation.r, animation.g, animation.b, animation.a)
  love.graphics.setBlendMode("add", "alphamultiply")

  local frame = math.floor(animation.sf + animation.age / animation.time)
  
  local tile = animation.tileset[frame]
  local image = tile.image
  local scale = animation.scale
  
  love.graphics.draw(image, 
                     animation.x - image:getWidth() * scale * 0.5, 
                     animation.y - image:getHeight() * scale * 0.5, 
                     0, 
                     scale, scale)

  love.graphics.setBlendMode(mode, alphamode)
end


animations.new = new
animations.update = update
animations.draw = draw

return animations