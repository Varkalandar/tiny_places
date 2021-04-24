-- 
-- Tiny Places item data
--
-- Author: Hj. Malthaner
-- Date: 2021/04/23
--

local items = {}


local function init()  
  print("Initializing map")
  
  local itemSet = tileset.readSet("resources/items/", "items.tica")
  
  items.itemSet = itemSet

end


items.init = init

return items;