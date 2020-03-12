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

local function setModePlace(pressed)
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


local function setModeMove(pressed)
  if not pressed then 
    mode = "move" 
    btMove.pressed = true
    btPlace.pressed = false  
    btDelete.pressed = false
  end
end


local function setModeDelete(pressed)
  if not pressed then 
    mode = "delete" 
    btMove.pressed = false
    btPlace.pressed = false  
    btDelete.pressed = true
  end
end

local function openPopup(pressed)
  if not pressed then
    editorUi.mainUi.popup = tileChooserPopup
  end
end

local function loadMap(pressed)
  if not pressed then
    map.clear()    
    map.clientSocket.send("LOAD")
    btLoad.pressed = false   
  end
end

local function saveMap(pressed)
  if not pressed then
    map.clientSocket.send("SAVE")
    btSave.pressed = false   
  end
end


local function selectPatchLayer()
  if not pressed then
    editorUi.activeLayer = 1
    
    btPatchlayer.pressed = true
    btMoblayer.pressed = false
    btCloudlayer.pressed = false
  end
end

local function selectMobLayer()
  if not pressed then
    editorUi.activeLayer = 3
    
    btPatchlayer.pressed = false
    btMoblayer.pressed = true
    btCloudlayer.pressed = false
  end
end

local function selectCloudLayer()
  if not pressed then
    editorUi.activeLayer = 5
    
    btPatchlayer.pressed = false
    btMoblayer.pressed = false
    btCloudlayer.pressed = true
  end
end


local function init(mainUi)
  print("Loading editor ui")
  
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


  btPatchlayer = cf.makeButton("Patch Layer", 180, 680, 8, 0.33, selectPatchLayer)
  container:add(btPatchlayer)
  
  btMoblayer = cf.makeButton("Item Layer", 180, 650, 8, 0.33, selectMobLayer)
  btMoblayer.pressed = true
  container:add(btMoblayer)
  
  btCloudlayer = cf.makeButton("Cloud Layer", 180, 620, 8, 0.33, selectCloudLayer)
  container:add(btCloudlayer)


  btSave = cf.makeButton("Save Map", 1050, 40, 12, 0.33, saveMap)
  container:add(btSave)

  btLoad = cf.makeButton("Load Map", 1050, 70, 12, 0.33, loadMap)
  container:add(btLoad)

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
  
  love.graphics.draw(editorUi.areaImage, 16, 600, 0, 0.5, 0.5)
  
  local tile = map.getLayerTileset(editorUi.activeLayer)[editorUi.tile]
  
  if tile then
    local scale = 0.5
    love.graphics.setColor(1.0, 1.0, 1.0)
    love.graphics.draw(tile.image, 90 - tile.footX*scale, 650 - tile.footY*scale, 0, scale, scale)
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
	    map.clientSocket.send("ADDM,"..editorUi.activeLayer..","..editorUi.tile..","..mx..","..my..",0.5")	
    end
  else
    container:mousePressed(mx, my)
  end
end


local function mouseReleased(button, mx, my)
  if isMapArea(mx, my) then  
    if mode == "move" then
      if editorUi.selectedMob then
	      editorUi.selectedMob.x = mx
	      editorUi.selectedMob.y = my
	      map.clientSocket.send("UPDM,"
                              ..editorUi.selectedMob.id..","
                              ..editorUi.activeLayer..","
                              ..editorUi.selectedMob.tile..","
                              ..editorUi.selectedMob.x..","
                              ..editorUi.selectedMob.y..","
                              ..editorUi.selectedMob.scale
                              )
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
