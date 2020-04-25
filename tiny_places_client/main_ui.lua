-- 
-- Main UI event dispatcher.
--
-- Author: Hj. Malthaner
-- Date: 2020/03/08
--

local gameUi = require("ui/game_ui")
local editorUi = require("ui/editor_ui")

local map = require("map")

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
	
	mainUi.image = love.graphics.newImage("resources/ui/main_ui.png")
	mainUi.lmbState = love.mouse.isDown(1)
	mainUi.rmbState = love.mouse.isDown(2)
	mainUi.popup = nil
	mainUi.wheelDelta = 0
	
	gameUi.init(mainUi, map)
	editorUi.init(mainUi, map)
	
	mainUi.gameUi = gameUi
	mainUi.editorUi = editorUi

  -- select active ui at start	
	mainUi.ui = editorUi
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
	  
      if cmd == "ADDM" then
        local id = tonumber(args())
        local layer = tonumber(args())
        local tile = tonumber(args())
        local x = tonumber(args())
        local y = tonumber(args())
        local scale = tonumber(args())
        local color = args()
        local ntype = args()
        local ctype = "prop"
        local faces = 1
        
        if ntype == "1" then ctype = "creature" faces = 8 end
        if ntype == "2" then ctype = "player" faces = 16 end
        
        map.addObject(id, layer, tile, x, y, scale, color, ctype, 120, faces)
      
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
        local x = tonumber(args())
        local y = tonumber(args())
        local scale = tonumber(args())
        local color = args()
        
        local mob = map.addObject(id, layer, tile, x, y, scale, color, "player", 120, 16)
        mainUi.gameUi.playerMob = mob

      elseif cmd == "FIRE" then
        local source = tonumber(args())
        local id = tonumber(args())
        local layer = tonumber(args())
        local ptype = args()
        local sx = tonumber(args())
        local sy = tonumber(args())
        local dx = tonumber(args())
        local dy = tonumber(args())
        local speed = tonumber(args())
        map.fireProjectile(source, id, layer, ptype, sx, sy, dx, dy, speed)
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
	
	local commands = map.clientSocket.receive()
	processCommands(commands)
	-- print("Received: " .. commands)
	
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
