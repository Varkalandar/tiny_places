-- 
-- Bitmap font
--
-- Author: Hj. Malthaner
-- Date: 2021/04/24
--

local pixfont = {}

--    private final int letterWidths [] = new int [256];
--    private final int letterHeights [] = new int [256];
--    private final String slips [] = new String [256];


local function readKerningInfo(path, pixfont)


  local file, size = love.filesystem.read(path..".kern")
  local lines = {}
  
  for line in string.gmatch(file, ".-\n") do
    -- table.insert(lines, line:match("[^\n]*"))
    -- print(line)
    
    local char = string.byte(line)
    -- check if this is kerning or slip
    if string.sub(line, 2, 2) == " " then
      -- kerning
      local adjustment = tonumber(string.sub(line, 3, -1))
      -- print("Kerning adjustment for letter " .. char .. " is " .. adjustment )
      pixfont.letterWidths[char] = pixfont.letterWidths[char] + adjustment
      
    elseif string.sub(line, 2, 2) == "-" then
      -- slip
      pixfont.slips[char] = string.sub(line, 3, -1)
      -- print("Slips for letter " .. char .. " are " .. pixfont.slips[char])
    end
  end

end

local function scanWidth(rasterX, rasterY, data, sx, sy)

  for x=rasterX-1, 0, -1 do
          
    for y=0, rasterY-1 do
      local xx = sx + x
      local yy = sy + y
              
      -- print("xx=" .. xx .. " yy=" .. yy)       
              
      local r, g, b, a = data:getPixel(xx, yy)
      -- print("a=" .. a)
      if a  > 0.5 then
        -- found a colored pixel
        return x+1
      end         
    end       
  end
  return 0
end


local function scanHeight(rasterX, rasterY, data, sx, sy)

  for y=rasterY-1, 0, -1 do
    for x=0, rasterX-1 do
          
      local xx = sx + x
      local yy = sy + y
              
      -- print("xx=" .. xx .. " yy=" .. yy)       
              
      local r, g, b, a = data:getPixel(xx, yy)
      -- print("a=" .. a)
      if a  > 0.5 then
        -- found a colored pixel
        return y+1
      end         
    end       
  end
  return 0
end

local function scanDimensions(pixfont)

  for letter=0, 255 do
    local sx = (letter % 8) * pixfont.rasterX
    local sy = math.floor(letter / 8) * pixfont.rasterY

    -- print("Sourcing " .. letter .. " from " .. sx .. ", " .. sy)
    pixfont.letterWidths[letter] = scanWidth(pixfont.rasterX, pixfont.rasterY, pixfont.imageData, sx, sy)
    -- print("Width " .. letter .. " = " .. pixfont.letterWidths[letter])
    pixfont.letterHeights[letter] = scanHeight(pixfont.rasterX, pixfont.rasterY, pixfont.imageData, sx, sy)
  
    pixfont.quads[letter] = love.graphics.newQuad(sx, sy, 
                                                  pixfont.letterWidths[letter], 
                                                  pixfont.letterHeights[letter], 
                                                  pixfont.image)
  end
end


local function init(path)

  print("Loading pixfont '" .. path .. "'")

  pixfont.imageData = love.image.newImageData(path .. ".png")
  pixfont.image = love.graphics.newImage(pixfont.imageData)
  pixfont.image:setFilter("linear", "linear")
  -- pixfont.image:setFilter("nearest", "nearest")
  
  pixfont.letterWidths = {}
  pixfont.letterHeights = {}
  pixfont.quads = {}
  pixfont.slips = {}
  
  pixfont.rasterX = pixfont.image:getWidth() / 8
  pixfont.rasterY = pixfont.image:getHeight() / 32
        
  scanDimensions(pixfont)
  readKerningInfo(path, pixfont)
    
end

    
local function calcStringWidth(text)

  local letterWidths = pixfont.letterWidths
  local letters = text:len()
  local w = 0;
        
  for p=1, letters do
    local c = text:byte(p, p)
    w = w + letterWidths[c]
  end     
  
  return w
end


local function drawCharacterScaled(x, y, character, scale)

  local quad = pixfont.quads[character]
  love.graphics.draw(pixfont.image, quad, x, y, 0, scale, scale)
end


local function drawStringScaled(text, x, y, scale)

  local letters = text:len()
        
  local runx = 0;
        
  for p=1, letters do
  -- for p=1, 1 do

    local c = string.byte(text, p)
    -- print("c=" .. string.char(c))
    
    drawCharacterScaled(x+runx*scale, y, c, scale)

    runx = runx + pixfont.letterWidths[c]
            
    if p < letters-1 and pixfont.slips[c] ~= nil then

      local next = string.byte(text, p+1)
      if pixfont.slips[c]:find(string.char(next)) then
        runx = runx - 1
      end
    end
  end
  
  return runx * scale
end


pixfont.init = init
pixfont.drawStringScaled = drawStringScaled
pixfont.calcStringWidth = calcStringWidth

return pixfont