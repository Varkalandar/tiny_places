-- 
-- Map editor UI
--
-- Author: Hj. Malthaner
-- Date: 2020/03/08
--

local cf = require("ui/component_factory")
local map = require("map")
local tileset = require("tileset")
local tileChooserPopup = require("ui/tile_chooser_popup")

local editorUi = {}
local mode = "place"

local btSave = nil
local btLoad = nil

local btMove = nil
local btPlace = nil
local btSelect = nil

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


local function setModePlace(pressed)
  if not pressed then 
    mode = "place" 
    btMove.pressed = false
    btPlace.pressed = true
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

local function init(mainUi)
  print("Loading editor ui")
  
	editorUi.areaImage = love.graphics.newImage("resources/ui/area_mid.png")
  
  editorUi.selectedMob = nil
  editorUi.tile = 1
  editorUi.mainUi = mainUi
	
  tileChooserPopup.init(mainUi, map.tileset)	

  btPlace = cf.makeButton("Place Item", 16, 450, 12, 0.33, setModePlace)
  btPlace.pressed = true
  container:add(btPlace)
  
  btMove = cf.makeButton("Move Item", 16, 480, 12, 0.33, setModeMove)
  container:add(btMove)

  btSelect = cf.makeButton("Select Item", 28, 680, 8, 0.33, openPopup)
  container:add(btSelect)

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
  
  local tile = tileset.get(editorUi.tile)
  local scale = 0.5
  love.graphics.setColor(1.0, 1.0, 1.0)
  love.graphics.draw(tile.image, 90 - tile.footX*scale, 650 - tile.footY*scale, 0, scale, scale)

  container:draw()
end


local function mousePressed(button, mx, my)

  if isMapArea(mx, my) then  
    if mode == "move" then
      editorUi.selectedMob = map.selectObject(mx, my, 50)
    else
	    map.clientSocket.send("ADDM,"..editorUi.tile..","..mx..","..my..",0.5")	
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
  if mode == "move" then
    if editorUi.selectedMob then
	    editorUi.selectedMob.x = mx
	    editorUi.selectedMob.y = my
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
