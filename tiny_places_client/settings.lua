-- 
-- "Tiny Places" global settings
--
-- Author: Hj. Malthaner
-- Date: 2021/05/13
--

local settings = {}


local function init()

  local file, size = love.filesystem.read("config.ini")
  
  for line in string.gmatch(file, ".-\n") do
	-- trim line
	line = line:match("[^\n\r]*")
	-- line = line:match("[^\r]*")
    -- print("line=" .. line)
	
	local args = line:gmatch("[^=]+")
	local name = args()
	local value = args()
	
    print("'" .. name .. "'='" .. value .. "'")
	
	settings[name] = value
  end
end


settings.init = init

return settings