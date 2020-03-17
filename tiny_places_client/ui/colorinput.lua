-- 
-- Color input field related code
--
-- Author: Hj. Malthaner
-- Date: 2020/03/16
--

local colorinput = {}


local function yuvToRgb(y, u, v)

  -- R = (y + 1.4075 * (v - 128));
  -- G = (y - 0.3455 * (u - 128) - (0.7169 * (v - 128)));
  -- B = (y + 1.7790 * (u - 128));

  local R = (y + 1 * (v - 128));
  local G = (y - 0.5 * (u - 128) - (0.5 * (v - 128)));
  local B = (y + 1 * (u - 128));

  if R < 0 then R=0 end
  if R > 255 then R=255 end
  if G < 0 then G=0 end
  if G > 255 then G=255 end
  if B < 0 then B=0 end
  if B > 255 then B=255 end

  return R, G, B;
  
end


local function setWhiteColor()
  colorinput.color = {y=255, u=127, v=127, r=1, g=1, b=1, a=1}
end


local function init()

  local data = love.image.newImageData(140, 70)
  local s = 1/255
  
  -- default white  
  for j=0, 5 do
    for i=0, 9 do
      data:setPixel(130+i, j, 1, 1, 1, 1)
    end
  end

  -- brightness bar
  for j=0, 5 do
    for i=0, 127 do
      local g = i * 2 * s
      data:setPixel(i, j, g, g, g, 1.0)
    end
  end
 
  -- alpha bar
  for j=0, 63 do
    for i=0, 9 do
      local a = j * 4 * s
      data:setPixel(i + 130, j + 6, 1, 1, 1, a)
    end
  end

  -- color field
  for j=0, 63 do
    for i=0, 127 do
      local r, g, b = yuvToRgb(64, i*2, j*4)
      data:setPixel(i, j+6, r*s, g*s, b*s, 1.0)
    end
  end
  
  local image = love.graphics.newImage(data)

  colorinput.image = image
  
  setWhiteColor()
end


-- round value to 4 digits
local function round4(v)
  local r = math.floor(v * 1000 + 0.5)
  return r / 1000.0
end
    

local function handleClick(input, mx, my)
  mx = mx - input.x
  my = my - input.y
  
  -- lightness bar ?
  if my < 10 then
    local brightness = mx * 2
    colorinput.color.y = brightness
    local r, g, b = yuvToRgb(colorinput.color.y, colorinput.color.u, colorinput.color.v)
    colorinput.color.r = round4(r / 255)
    colorinput.color.g = round4(g / 255)
    colorinput.color.b = round4(b / 255)
    
    -- default white
    if mx > 127 then
      setWhiteColor()
    end
  end
  
  -- alpha bar ?
  if mx >= 130 and my >= 6 then
    local alpha = (my - 6) * 4
    colorinput.color.a = round4(alpha / 255)
  end
  
  -- color chooser ?
  if mx >= 0 and my >= 6 and mx < 128 and my < 70 then
    colorinput.color.u = mx * 2 
    colorinput.color.v = (my - 12) * 4
  
    local r, g, b = yuvToRgb(colorinput.color.y, colorinput.color.u, colorinput.color.v)

    colorinput.color.r = round4(r / 255)
    colorinput.color.g = round4(g / 255)
    colorinput.color.b = round4(b / 255)
  end

  print("r=" .. colorinput.color.r .. 
        " g=" .. colorinput.color.g .. 
        " b=" .. colorinput.color.b .. 
        " a=" .. colorinput.color.a)
        
  return colorinput.color
  
end


local function draw(input)
  love.graphics.setColor(1, 1, 1)

  love.graphics.draw(colorinput.image, 
                     input.x, input.y, 0, 1, 1)
end

colorinput.init = init
colorinput.handleClick = handleClick
colorinput.draw = draw

return colorinput
