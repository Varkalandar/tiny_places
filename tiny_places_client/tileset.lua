-- 
-- Tileset loading code
--
-- Author: Hj. Malthaner
-- Date: 2020/03/08
--

local tileset = {}


local function readTile(tile)
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
    descriptor.image = love.graphics.newImage("resources/objects/" .. filename)
  end

  return descriptor  
end


local function init()
  print("Initializing tileset")

  local file, size = love.filesystem.read("resources/objects/map_objects.tica");
  local count = 0
  
  for tile in string.gmatch(file, "Description.-Tile") do
    -- print("--- Reading tile ---")
    local descriptor = readTile(tile)
    tileset[descriptor.id] = descriptor
    count = count + 1
  end
  
end


local function get(index)
  return tileset[index]
end


tileset.init = init;
tileset.get = get;

return tileset;
