--
-- Cast a spell. Draw all required effects and fire a projectile.
--
-- Author: Hj. Malthaner
-- Date: 2020/04/26
--

local spells = {}


local function fire(spell)
  
  local map = spell.map
  local shooter = spell.shooter
  local id = spell.id
  local layer = spell.layer
  local ptype = spell.ptype
  local sx = spell.shooter.x
  local sy = spell.shooter.y
  local dx = spell.dx
  local dy = spell.dy
  local speed = spell.speed
  
  -- print("Adding projectile with type " .. ptype .. " fired at " .. dx .. ", " .. dy)

  
  -- there shouldbe a ptype -> tile calculation here, once
  -- there is more than one projectile type
  local tile = 1
  local color = "1 1 1 1"
  
  if ptype == "debris" then
    tile = 9
  elseif ptype == "fireball" then
    tile = 25
    color = "1 1 1 1"
  elseif ptype == "dust_vortex" then
    tile = 17
    color = "1 0.8 0.1 0.8"
  elseif ptype == "dirt_shot" then
    tile = 9
  end
  
  local nx = dx-sx
  local ny = dy-sy
  local len = math.sqrt(nx*nx + ny*ny)
  
  nx = nx / len
  ny = ny / len

  -- make projectile appear somewhere in front of the shooter
  local distance = 12
  if ptype == "debris" then
    distance = 4
  elseif ptype == "dust_vortex" then
    distance = 8
  elseif ptype == "dirt_shot" then
    distance = 4
  end
  
  sx = sx + nx * distance * 2
  sy = sy + ny * distance
  
  shooter:orient(nx, ny)

  local projectile = map.addObject(id, layer, tile, sx, sy, 1, color, nil, nil, "projectile", speed, 8, 1)
  projectile.ptype = ptype
  
  local pattern = "glide"
  
  if ptype == "debris" then
    pattern = "drop"
                          
    projectile.color.r = 0.2 + math.random() * 0.2
    projectile.color.g = 0.2 + math.random() * 0.1
    projectile.color.b = 0.2 + math.random() * 0.1
    projectile.color.a = 0.8 + math.random() * 0.2
    
    projectile.zSpeed = 0.5 + math.random() * 1
    projectile.scale = 0.2 + math.random() * 0.6

  elseif ptype == "fireball" then
    pattern = "glide"
    projectile.scale = 0.5
    projectile.zOff = 20

  elseif ptype == "dust_vortex" then
    pattern = "spin"
    projectile.scale = 0.18
    
  elseif ptype == "dirt_shot" then
    projectile.scale = 0.5
    projectile.zOff = 2
    
  else
    -- fire at half height of the shooter
    projectile.zOff = 20
  end
  
  local move = map.addMove(id, layer, dx, dy, speed, pattern)
  
  -- projectile launching sound: todo - move player fireball sound here
  if ptype == "dirt_shot" then
    tip.sounds.randplay2(tip.sounds.debrisHit1, tip.sounds.debrisHit2, 0.5, 2.0, 1.0)
  end
  
  return projectile, move
end


local function update(spell, dt)
  spell.age = spell.age + dt
end


local function drawCastAnimation(spell)

  local mode, alphamode = love.graphics.getBlendMode()
  love.graphics.setColor(spell.r, spell.g, spell.b, spell.a)
  love.graphics.setBlendMode("add", "alphamultiply")

  local frame = math.floor(spell.sf + spell.age / spell.time)
  
  -- print("frame=" .. frame)
  
  local tile = spell.tileset[frame]
  local image = tile.image
  local scale = spell.scale
  local sx = spell.shooter.x
  local sy = spell.shooter.y - 30
  
  love.graphics.draw(image, 
                     sx - image:getWidth() * scale * 0.5, 
                     sy - image:getHeight() * scale * 0.5, 
                     0, 
                     scale, scale)

  love.graphics.setBlendMode(mode, alphamode)
end


local function drawOver(spell)

  if spell.age < spell.castTime then
    drawCastAnimation(spell)
  else
    if not spell.done then
      spell:fire()
      spell.done = true
    end
  end

end


local function new(map, shooter, id, layer, ptype, castTime, dx, dy, speed, animationSet)
  local spell = 
  {
    -- data to fire the projectile
    map = map, 
    shooter = shooter, 
    id = id,
    layer = layer,
    ptype = ptype,
    castTime = castTime,
    dx = dx,
    dy = dy,
    speed = speed,

    -- data for the animation
    scale = 1,
    r = 1,
    g = 0.7,
    b = 0.0,
    a = 0.15,
    tileset = animationSet,
    sf=1,
    ef=21,
    age=0,
    time=0.02,
    done=false,

    -- methods
    fire=fire,
    drawOver=drawOver,
    update=update
  }
  
  return spell
end


spells.new = new

return spells