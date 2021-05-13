-- 
-- Main UI event dispatcher.
--
-- Author: Hj. Malthaner
-- Date: 2020/03/08
--

local gameUi = require("ui/game_ui")
local editorUi = require("ui/editor_ui")

local map = require("map")

local pixfont = require("ui/pixfont")

local mainUi = {};

-- event handling code

local function mousePressed(button)
  if(mainUi.popup) then
    -- route all events to the popup
    mainUi.popup.mousePressed(button, mainUi.mx, mainUi.my)
  else 
    if mainUi.ui then
      mainUi.ui.mousePressed(button, mainUi.mx, mainUi.my)
    end
  end
end


local function mouseReleased(button)
  if(mainUi.popup) then
    -- route all events to the popup
    mainUi.popup.mouseReleased(button, mainUi.mx, mainUi.my)
  else  
    if mainUi.ui then
      mainUi.ui.mouseReleased(button, mainUi.mx, mainUi.my)
    end
  end
end


local function mouseDragged(button)
  if(mainUi.popup) then
    -- route all events to the popup
    mainUi.popup.mouseDragged(button, mainUi.mx, mainUi.my)
  else  
    if mainUi.ui then
      mainUi.ui.mouseDragged(button, mainUi.mx, mainUi.my)
    end
  end
end

-- end of event handling code

local function init()
  map.init()      

  print("Initializing main ui")
	
  -- pixfont.init("resources/font/humanistic_128b")
  pixfont.init("resources/font/humanistic_128bbl")
  
  mainUi.image = love.graphics.newImage("resources/ui/silver/main_ui.png")
	mainUi.lmbState = love.mouse.isDown(1)
	mainUi.rmbState = love.mouse.isDown(2)
	mainUi.popup = nil
	mainUi.wheelDelta = 0
	mainUi.pixfont = pixfont

	gameUi.init(mainUi, map)
	editorUi.init(mainUi, map)

	mainUi.gameUi = gameUi
	mainUi.editorUi = editorUi

	-- select active ui at start	
	mainUi.ui = editorUi
end


local function updatePlayerStats(args)
  
  local stat = 0
  while stat ~= nil do
    stat = args()
	-- print("stat=" .. stat)
	if stat ~= nil then
	  stat = tonumber(stat)
	  local min = tonumber(args())
	  local max = tonumber(args())
	  local value = tonumber(args())
	
	  player.stats[stat] = {min=min, max=max, value=value}
    end
  end
end


--
-- process commands received from the server
--
local function processCommands(commands)

  if commands:len() > 0 then
    local iter = commands:gmatch("[^\n]+")

    for command in iter do
      print("Command: " .. command);
      local args = command:gmatch("[^,]+")
      local cmd = args()
      print("Cmd: " .. cmd);
	  
      if cmd == "ADDI" then
        local mobString = args()
        local mobId = nil
        
        if "-" ~= mobString then
          mobId = tonumber(mobString)
        end
        
        local item = 
        {
          baseId = args(),
          itemId = tonumber(args()),
          id = tonumber(args()),
          displayName = args(),
          iclass = args(),
          itype = args(),
          value = tonumber(args()),
          tile = tonumber(args()),
          color = args(),
          scale = tonumber(args()),
          shadow = tonumber(args()),
          shadowScale = tonumber(args()),
          where = tonumber(args()),
          x = tonumber(args()),
          y = tonumber(args()),
          energyDamage = tonumber(args()),
          physicalDamage = tonumber(args()),
          description = args()
        }

        -- debug data
        for k, v in pairs(item) do print("  " .. k, v) end
        
        if mobId == nil then
          -- place item on map ground
          map.addObject(item.id, 3, item.tile, item.x, item.y, item.scale, item.color,
                                   item.shadow, item.shadowScale,          
                                  "item", 0, 1, 1)
          
        else
          -- this item goes to the player inventory
          -- check if mobId is the actual player?
          
          table.insert(map.playerInventory, item)
        end
      
      elseif cmd == "ADDM" then
        local id = tonumber(args())
        local layer = tonumber(args())
        local tile = tonumber(args())
        local frames = tonumber(args())
        local phases = tonumber(args())
        local x = tonumber(args())
        local y = tonumber(args())
        local scale = tonumber(args())
        local color = args()
        local ntype = args()

        local ctype
        if ntype == "0" then
          ctype = "prop"
        elseif ntype == "1" then 
          ctype = "creature" 
        elseif ntype == "2" then 
          ctype = "player"
        else
          print("Invalid ntype=" .. ntype .. " must be 0..2")
          print(debug.traceback)
        end
        
        map.addObject(id, layer, tile, x, y, scale, color, nil, nil, ctype, 120, frames, phases)
      
      elseif cmd == "ANIM" then
        local id = tonumber(args())
        local layer = tonumber(args())
        local x = tonumber(args())
        local y = tonumber(args())

        map.playAnimation(id, layer, x, y)
        
      elseif cmd == "UPDM" then
        local id = tonumber(args())
        local layer = tonumber(args())
        local tile = tonumber(args())
        local x = tonumber(args())
        local y = tonumber(args())
        local scale = tonumber(args())
        local color = args()
        
        map.updateObject(id, layer, tile, x, y, scale, color)
        
      elseif cmd == "DELM" then
        local id = tonumber(args())
        local layer = tonumber(args())
        map.removeObject(id, layer)
        
      elseif cmd == "LOAD" then
        local name = args()
        local backdrop = args()
        local filename = args()
        map.load(name, backdrop, filename)

      elseif cmd == "MOVE" then
        local id = tonumber(args())
        local layer = tonumber(args())
        local x = tonumber(args())
        local y = tonumber(args())
        local speed = tonumber(args())
        local pattern = args()
        map.addMove(id, layer, x, y, speed, pattern)
		
      elseif cmd == "ADDP" then
        local id = tonumber(args())
        local layer = tonumber(args())
        local tile = tonumber(args())
        local frames = tonumber(args())
        local phases = tonumber(args())
        local x = tonumber(args())
        local y = tonumber(args())
        local scale = tonumber(args())
        local color = args()
        
        local mob = map.addObject(id, layer, tile, x, y, scale, color, nil, nil, "player", 120, frames, phases)
        mainUi.gameUi.playerMob = mob

      elseif cmd == "FIRE" then
        local source = tonumber(args())
        local id = tonumber(args())
        local layer = tonumber(args())
        local ptype = args()
        local castTime = tonumber(args()) / 1000.0
        local dx = tonumber(args())
        local dy = tonumber(args())
        local speed = tonumber(args())
        map.fireProjectile(source, id, layer, ptype, castTime, dx, dy, speed)

	  elseif cmd == "STAT" then
	    updatePlayerStats(args)
      end
    end
  end
end


function love.wheelmoved(dx, dy)
	-- record the changes till the next update call
	mainUi.wheelDelta = mainUi.wheelDelta + dy
end


local function update(dt)

	-- check position changes
	local mx, my = love.mouse.getPosition()
	if mx ~= mainUi.mx or my ~= mainUi.my then
		mainUi.mx = mx
		mainUi.my = my
	
		-- moving the mouse while lmb is down means dragging
		if mainUi.lmbState then
		  mouseDragged(1)
		end
	end			
	
	-- check state changes
	local lmbState = love.mouse.isDown(1)
	if(lmbState ~= mainUi.lmbState) then
		mainUi.lmbState = lmbState;
		print("Left mouse button went " .. (lmbState and "down" or "up") .. " at " .. mx .. ", " .. my);
		
		if lmbState then
			mousePressed(1)
		else
			mouseReleased(1)
		end
	end

	local rmbState = love.mouse.isDown(2)
	if(rmbState ~= mainUi.rmbState) then
		mainUi.rmbState = rmbState;
		print("Right mouse button went " .. (rmbState and "down" or "up") .. " at " .. mx .. ", " .. my);
		
		if rmbState then
			mousePressed(2)
		else
			mouseReleased(2)
		end
	end

	if mainUi.ui then
    mainUi.ui.update(dt)
  end
	
	
	if mainUi.popup then
	  mainUi.popup.update(dt)
	end
	
  
	local commands
  repeat
    commands = map.clientSocket.receive()
    processCommands(commands)
    -- print("Received: " .. commands)
  until commands:len() <= 0
	
	-- clear delta to collect updates till next frame
	mainUi.wheelDelta = 0
  
  map.update(dt)
end


local function draw()
  love.graphics.setColor(1.0, 1.0, 1.0)
  map.drawFloor()

  love.graphics.setColor(1.0, 1.0, 1.0)
	love.graphics.draw(mainUi.image)

	if mainUi.ui then
	  mainUi.ui.draw()
	end

  map.drawObjects()
  map.drawClouds()
	
	if mainUi.popup then
	  mainUi.popup.draw()
	end
end


mainUi.init = init;
mainUi.update = update;
mainUi.draw = draw;


return mainUi;
