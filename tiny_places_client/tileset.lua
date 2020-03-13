-- 
-- Tileset loading code
--
-- Author: Hj. Malthaner
-- Date: 2020/03/08
--

local tileset = {}


local function readTile(path, tile)
  local set = {}
  for line in tile:gmatch(".-\n") do
    table.insert(set, line:match("[^\n]*"))
  end
  
  local descriptor = {}
  
  descriptor.id = tonumber(set[3])
  
  local iter = set[4]:gmatch("[^ ]+")
  descriptor.width = tonumber(iter())
  descriptor.height = tonumber(iter())

  local iter = set[6]:gmatch("[^ ]+")
  descriptor.footX = tonumber(iter())
  descriptor.footY = tonumber(iter())

  descriptor.name = set[12]
  
--  for key, value in pairs (descriptor) do
--    print(key .. "=" .. value)    
--  end
  
  local filename = descriptor.id .. "-" .. descriptor.name .. ".png"

--   print(filename)    
  
  if(descriptor.width > 1 and descriptor.height > 1) then
    descriptor.image = love.graphics.newImage(path .. filename)
  end

  return descriptor  
end


local function readSet(path, filename)
  print("Initializing tileset " .. path .. filename)

  local set = {}
  
  local file, size = love.filesystem.read(path..filename);
  local count = 0
  
  for tile in string.gmatch(file, "Description.-Tile") do
    -- print("--- Reading tile ---")
    local descriptor = readTile(path, tile)
    set[descriptor.id] = descriptor
    count = count + 1
  end
  
  return set
end


tileset.readSet = readSet;

return tileset;
