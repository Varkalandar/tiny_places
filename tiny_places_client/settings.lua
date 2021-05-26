-- 
-- "Tiny Places" global settings
--
-- Author: Hj. Malthaner
-- Date: 2021/05/13
--


local settings = {}


--
-- on Linux this ends up in ~/.local/share/love/lovegame/tinyplaces.ini
-- Love2D doesn't allow access to write to user home directory, it seems?
--
local config = "tinyplaces.ini"


local function init()

  local file = love.filesystem.newFile(config)

  local success, message = file:open("r")
  
  if not success then 
    print ("Could not open config file: " .. message)
  else
  
    local contents, size = file:read()
    
    for line in string.gmatch(contents, ".-\n") do
      -- trim line
      line = line:match("[^\n\r]*")
      -- print("line=" .. line)
      
      local args = line:gmatch("[^=]+")
      local name = args()
      local value = args()
      
      print("'" .. name .. "'='" .. value .. "'")
      
      settings[name] = value
    end
    
    file:close()
  end
end


local function save()

  local file = love.filesystem.newFile(config)

  local success, message = file:open("w")
  
  if not success then 
    print ("Could not open config file: " .. message)
  else

    local data =
      "server_ip=" .. settings.server_ip .. "\n"

    local success, message = file:write(data)
  
    if not success then 
      print ("Could not write config data: " .. message)
    end
    
    file:close()
  end
end


settings.init = init
settings.save = save

return settings
