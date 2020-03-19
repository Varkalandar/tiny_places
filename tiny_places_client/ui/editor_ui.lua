-- 
-- Map editor UI
--
-- Author: Hj. Malthaner
-- Date: 2020/03/08
--

local cf = require("ui/component_factory")
local map = require("map")
local tileChooserPopup = require("ui/tile_chooser_popup")

local editorUi = {}
local mode = "place"

local btSave = nil
local btLoad = nil

local btMove = nil
local btPlace = nil
local btDelete = nil
local btSelect = nil

local btPatchlayer = nil
local btMoblayer = nil
local btCloudlayer = nil

local btColor = nil

-- UI element container for this UI
local container = cf.makeContainer()

--
-- Checks if the given screen coordinate is inside the map area
--
local function isMapArea(mx, my)

	-- normalize coordinates
	mx = mx - 600
	my = my - 28

	local mi = mx + my * 2
	local mj = -mx + my * 2 
	-- print("mi=" .. mi .. " mj=" .. mj)
	
	return mi >= 0 and mi <= 1176 and mj >= 0 and mj <= 1176
end

--
-- button callbacks
--

local function setModePlace(x, y, pressed)
	if not pressed then 
		mode = "place" 
		btMove.pressed = false
		btPlace.pressed = true
		btDelete.pressed = false

		if editorUi.selectedMob then
			editorUi.selectedMob.selected = false
			editorUi.selectedMob = nil
		end
	end
end


local function setModeMove(x, y, pressed)
	if not pressed then 
		mode = "move" 
		btMove.pressed = true
		btPlace.pressed = false  
		btDelete.pressed = false
	end
end


local function setModeDelete(x, y, pressed)
	if not pressed then 
		mode = "delete" 
		btMove.pressed = false
		btPlace.pressed = false  
		btDelete.pressed = true
	end
end

local function openPopup(x, y, pressed)
	if not pressed then
		editorUi.mainUi.popup = tileChooserPopup
	end
end

local function loadMap(x, y, pressed)
	if not pressed then
		map.clear()    
		map.clientSocket.send("LOAD")
		btLoad.pressed = false   
	end
end

local function saveMap(x, y, pressed)
	if not pressed then
		map.clientSocket.send("SAVE")
		btSave.pressed = false   
	end
end


local function selectPatchLayer(x, y, pressed)
	if not pressed then
		editorUi.activeLayer = 1
		
		btPatchlayer.pressed = true
		btMoblayer.pressed = false
		btCloudlayer.pressed = false

		tileChooserPopup.init(editorUi.mainUi, map.patchSet)
	end
end

local function selectMobLayer(x, y, pressed)
	if not pressed then
		editorUi.activeLayer = 3
		
		btPatchlayer.pressed = false
		btMoblayer.pressed = true
		btCloudlayer.pressed = false
		
		tileChooserPopup.init(editorUi.mainUi, map.mobSet)
	end
end

local function selectCloudLayer(x, y, pressed)
	if not pressed then
		editorUi.activeLayer = 5
		
		btPatchlayer.pressed = false
		btMoblayer.pressed = false
		btCloudlayer.pressed = true
		
		tileChooserPopup.init(editorUi.mainUi, map.cloudSet)
	end
end


local function sendUpdateMob(mob, layer)

	map.clientSocket.send("UPDM,"
												..mob.id..","
												..layer..","
												..mob.tile..","
												..mob.x..","
												..mob.y..","
												..mob.scale..","
												..mob.color.r.." "..mob.color.g.." "..mob.color.b.." "..mob.color.a
												)
end


local function colorChanged(x, y, pressed)
	if not pressed then
		local color = btColor:handleClick(x, y)
		local mob = editorUi.selectedMob
		
		-- set color if there is a selected object
		if mob then
			mob.color.r = color.r
			mob.color.g = color.g
			mob.color.b = color.b
			mob.color.a = color.a
		
			sendUpdateMob(editorUi.selectedMob, editorUi.activeLayer)
		end
	end
end


local function init(mainUi)
	print("Loading editor ui")
	
	cf.init()
	
	editorUi.areaImage = love.graphics.newImage("resources/ui/area_mid.png")
	
	editorUi.selectedMob = nil
	editorUi.tile = 1
	editorUi.activeLayer = 3
	editorUi.mainUi = mainUi
	
	tileChooserPopup.init(mainUi, map.mobSet)	

	btPlace = cf.makeButton("Place Item", 16, 450, 12, 0.33, setModePlace)
	btPlace.pressed = true
	container:add(btPlace)
	
	btMove = cf.makeButton("Move Item", 16, 480, 12, 0.33, setModeMove)
	container:add(btMove)

	btDelete = cf.makeButton("Remove Item", 140, 480, 2, 0.33, setModeDelete)
	container:add(btDelete)

	btSelect = cf.makeButton("Select Item", 28, 680, 8, 0.33, openPopup)
	container:add(btSelect)


	btPatchlayer = cf.makeButton("Ground Layer", 180, 680, 0, 0.33, selectPatchLayer)
	container:add(btPatchlayer)
	
	btMoblayer = cf.makeButton("Item Layer", 180, 650, 10, 0.33, selectMobLayer)
	btMoblayer.pressed = true
	container:add(btMoblayer)
	
	btCloudlayer = cf.makeButton("Cloud Layer", 180, 620, 8, 0.33, selectCloudLayer)
	container:add(btCloudlayer)


	btSave = cf.makeButton("Save Map", 1050, 40, 12, 0.33, saveMap)
	container:add(btSave)

	btLoad = cf.makeButton("Load Map", 1050, 70, 12, 0.33, loadMap)
	container:add(btLoad)

	-- local colorInput = cf.makeInput("1.0, 1.0, 1.0", 16, 510, 160, 24, nil)
	btColor = cf.makeColor(326, 634, colorChanged)
	container:add(btColor)
end


local function update(dt)
	local delta = editorUi.mainUi.wheelDelta * 0.01

	if editorUi.selectedMob then
		editorUi.selectedMob.scale = editorUi.selectedMob.scale + delta
	end
end


local function draw()
	love.graphics.setColor(1.0, 1.0, 1.0)
	love.graphics.print("Edit Mode", 16, 30, 0, 2, 2)
	
	-- tile preview area
	love.graphics.draw(editorUi.areaImage, 16, 600, 0, 0.5, 0.5)
	
	-- color selector area
	love.graphics.draw(editorUi.areaImage, 324, 632, 0, 0.515, 0.52)

	local tile = map.getLayerTileset(editorUi.activeLayer)[editorUi.tile]
	
	if tile and tile.image then
		local scale = 0.5
		love.graphics.setColor(1.0, 1.0, 1.0)
		love.graphics.draw(tile.image, 90 - tile.footX*scale, 650 - tile.footY*scale, 0, scale, scale)
		love.graphics.print(tile.name, 32, 650 - math.floor(tile.footY*scale*0.9) - 16, 0, 1.0, 1.0)
	end
	
	container:draw()
end


local function mousePressed(button, mx, my)

	if isMapArea(mx, my) then  
		if mode == "move" then
			editorUi.selectedMob = map.selectObject(editorUi.activeLayer, mx, my, 50)
		elseif mode == "delete" then
			editorUi.selectedMob = map.selectObject(editorUi.activeLayer, mx, my, 50)
			map.clientSocket.send("DELM,"..editorUi.selectedMob.id..","..editorUi.activeLayer)
		else
			map.clientSocket.send("ADDM,"
														..editorUi.activeLayer..","
														..editorUi.tile..","
														..mx..","
														..my..","
														.."0.5,"
														.."1.0 1.0 1.0 1.0"
														)	
		end
	else
		container:mousePressed(mx, my)
	end
end


local function mouseReleased(button, mx, my)
	if isMapArea(mx, my) then  
		if mode == "move" then
			if editorUi.selectedMob then
				sendUpdateMob(editorUi.selectedMob, editorUi.activeLayer)
			end
		end
	else
		container:mouseReleased(mx, my)
	end
end


local function mouseDragged(button, mx, my)
	if isMapArea(mx, my) then  
		if mode == "move" then
			if editorUi.selectedMob then
				editorUi.selectedMob.x = mx
				editorUi.selectedMob.y = my
			end
		end
	end
end


editorUi.init = init;
editorUi.update = update;
editorUi.draw = draw;
editorUi.mousePressed = mousePressed;
editorUi.mouseReleased = mouseReleased;
editorUi.mouseDragged = mouseDragged;


return editorUi
