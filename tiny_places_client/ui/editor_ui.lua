-- 
-- Map editor UI
--
-- Author: Hj. Malthaner
-- Date: 2020/03/08
--

local cf = require("ui/component_factory")
local tileChooserPopup = require("ui/tile_chooser_popup")

local editorUi = {}
local mode = "place"

local btSave = nil
local btLoad = nil

local btGame = nil

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
    
    -- add preview object (id = -1)
    if editorUi.previewMob == nil then
      editorUi.previewMob = editorUi.map.addObject(-1, 
                                                   editorUi.activeLayer, editorUi.tile, 
                                                   0, 0, 0.5, 
                                                   "1.0 1.0 1.0 0.3",
                                                   nil, nil,
                                                   "prop",
                                                   0,
                                                   0,
                                                   1)
    end
  end
end


local function setModeMove(x, y, pressed)
  if not pressed then 
    mode = "move" 
    btMove.pressed = true
    btPlace.pressed = false  
    btDelete.pressed = false
    
    if editorUi.previewMob then
      editorUi.map.removeObject(editorUi.previewMob.id, editorUi.activeLayer)
      editorUi.previewMob = nil
    end
  end
end


local function setModeDelete(x, y, pressed)
  if not pressed then 
    mode = "delete" 
    btMove.pressed = false
    btPlace.pressed = false  
    btDelete.pressed = true

    if editorUi.previewMob then
      editorUi.map.removeObject(editorUi.previewMob.id, editorUi.activeLayer)
      editorUi.previewMob = nil
    end
  end
end

local function openPopup(x, y, pressed)
  if not pressed then
    editorUi.mainUi.popup = tileChooserPopup
  end
end

local function loadMap(x, y, pressed)
  if not pressed then
    editorUi.map.clear()    
    editorUi.map.clientSocket.send("LOAD")
    btLoad.pressed = false   
  end
end

local function saveMap(x, y, pressed)
  if not pressed then
    local map = editorUi.map
    map.clientSocket.send("SAVE," .. map.filename)
    btSave.pressed = false   
  end
end


local function selectPatchLayer(x, y, pressed)
  if not pressed then
    btPatchlayer.pressed = true
    btMoblayer.pressed = false
    btCloudlayer.pressed = false

    tileChooserPopup.init(editorUi.mainUi, editorUi.map.patchSet)
    
    if editorUi.previewMob then
      editorUi.map.removeObject(editorUi.previewMob.id, editorUi.activeLayer)
      editorUi.previewMob = nil
    end
    
    editorUi.activeLayer = 1
    
  end
end

local function selectMobLayer(x, y, pressed)
  if not pressed then
    btPatchlayer.pressed = false
    btMoblayer.pressed = true
    btCloudlayer.pressed = false
    
    tileChooserPopup.init(editorUi.mainUi, editorUi.map.mobSet)
    
    if editorUi.previewMob then
      editorUi.map.removeObject(editorUi.previewMob.id, editorUi.activeLayer)
      editorUi.previewMob = nil
    end
    
    editorUi.activeLayer = 3
    
  end
end

local function selectCloudLayer(x, y, pressed)
  if not pressed then
    btPatchlayer.pressed = false
    btMoblayer.pressed = false
    btCloudlayer.pressed = true
    
    tileChooserPopup.init(editorUi.mainUi, editorUi.map.cloudSet)
    
    if editorUi.previewMob then
      editorUi.map.removeObject(editorUi.previewMob.id, editorUi.activeLayer)
      editorUi.previewMob = nil
    end
    
    editorUi.activeLayer = 5
    
  end
end


local function switchToGameUi()
  if not pressed then
  
    if editorUi.previewMob then
      editorUi.map.removeObject(editorUi.previewMob.id, editorUi.activeLayer)
      editorUi.previewMob = nil
    end

    editorUi.mainUi.ui = editorUi.mainUi.gameUi
    
    editorUi.map.clientSocket.send("GAME,"
	                                              .."CHARACTER_ID_GOES_HERE")
  end
end


local function sendUpdateMob(mob, layer)

  editorUi.map.clientSocket.send("UPDM,"
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


local function init(mainUi, map)
  print("Loading editor ui")
  
  cf.init()
  
  editorUi.areaImage = love.graphics.newImage("resources/ui/area_mid.png")
  
  editorUi.selectedMob = nil
  editorUi.tile = 1
  editorUi.activeLayer = 3
  editorUi.mainUi = mainUi
  editorUi.map = map
  
  tileChooserPopup.init(mainUi, editorUi.map.mobSet)	

  btPlace = cf.makeButton("Place Item", mainUi.uifont, 16, 430, 0, 0.35, setModePlace)
  btPlace.pressed = true
  container:add(btPlace)
  
  btMove = cf.makeButton("Edit Item", mainUi.uifont, 16+68, 464, 0, 0.35, setModeMove)
  container:add(btMove)

  btDelete = cf.makeButton("Remove Item", mainUi.uifont, 16+136, 498, 0, 0.35, setModeDelete)
  container:add(btDelete)

  btSelect = cf.makeButton("Select Item", mainUi.uifont, 4, 680, 0, 0.35, openPopup)
  container:add(btSelect)


  btPatchlayer = cf.makeButton("Ground Layer", mainUi.uifont, 210, 680, 0, 0.35, selectPatchLayer)
  container:add(btPatchlayer)
  
  btMoblayer = cf.makeButton("Item Layer", mainUi.uifont, 210, 646, 0, 0.35, selectMobLayer)
  btMoblayer.pressed = true
  container:add(btMoblayer)
  
  btCloudlayer = cf.makeButton("Cloud Layer", mainUi.uifont, 210, 612, 0, 0.35, selectCloudLayer)
  container:add(btCloudlayer)


  btSave = cf.makeButton("Save Map", mainUi.pixfont,  980, 40, 0, 0.35, saveMap)
  container:add(btSave)

  btLoad = cf.makeButton("Load Map", mainUi.pixfont,  980, 70, 0, 0.35, loadMap)
  container:add(btLoad)

  -- local colorInput = cf.makeInput("1.0, 1.0, 1.0", 16, 510, 160, 24, nil)
  btColor = cf.makeColor(440, 634, colorChanged)
  container:add(btColor)
  
  btGame = cf.makeButton("Game Mode", mainUi.pixfont, 16, 90, 8, 0.35, switchToGameUi)
  container:add(btGame)

  setModePlace()
end


local function update(dt)
  local delta = editorUi.mainUi.wheelDelta * 0.01

  if editorUi.selectedMob and delta ~= 0 then
    editorUi.selectedMob.scale = editorUi.selectedMob.scale + delta
    sendUpdateMob(editorUi.selectedMob, editorUi.activeLayer)
  end
  
  if editorUi.previewMob then
    editorUi.previewMob.tile = editorUi.tile
  
    if isMapArea(editorUi.mainUi.mx, editorUi.mainUi.my) then
      editorUi.previewMob.x = editorUi.mainUi.mx
      editorUi.previewMob.y = editorUi.mainUi.my
    end
  end
  
  if mode == "move" then  
    if editorUi.selectedMob then
      if love.keyboard.isDown("up") then
        editorUi.selectedMob.y = editorUi.selectedMob.y - 1
        sendUpdateMob(editorUi.selectedMob, editorUi.activeLayer)
      end
      if love.keyboard.isDown("down") then
        editorUi.selectedMob.y = editorUi.selectedMob.y + 1
        sendUpdateMob(editorUi.selectedMob, editorUi.activeLayer)
      end
    
      if love.keyboard.isDown("left") then
        editorUi.selectedMob.x = editorUi.selectedMob.x - 1
        sendUpdateMob(editorUi.selectedMob, editorUi.activeLayer)
      end
      if love.keyboard.isDown("right") then
        editorUi.selectedMob.x = editorUi.selectedMob.x + 1
        sendUpdateMob(editorUi.selectedMob, editorUi.activeLayer)
      end
    end
  end
end


local function draw()

  local pixfont = editorUi.mainUi.pixfont
  
	love.graphics.setColor(0, 0, 0)
	pixfont:drawStringScaled("Edit Mode", 16-10, 30+24, 0.5, 0.25, 0.2, 0)
  love.graphics.setColor(1.0, 1.0, 1.0)
	pixfont:drawStringScaled("Edit Mode", 16, 30, 0.5, 0.5)

  -- tile preview area
  love.graphics.draw(editorUi.areaImage, 16, 600, 0, 0.5, 0.5)
  
  -- color selector area
  love.graphics.draw(editorUi.areaImage, 440-2, 632, 0, 0.515, 0.52)

  local tile = editorUi.map.getLayerTileset(editorUi.activeLayer)[editorUi.tile]
  
  if tile and tile.image then
    local scale = 0.5
    love.graphics.setColor(1.0, 1.0, 1.0)
    love.graphics.draw(tile.image, 90 - tile.footX*scale, 650 - tile.footY*scale, 0, scale, scale)
    -- love.graphics.print(tile.name, 32, 650 - math.floor(tile.footY*scale*0.9) - 16, 0, 1.0, 1.0)
    pixfont:drawStringScaled(tile.name, 32, 650 - math.floor(tile.footY*scale*0.9) - 26, 0.18, 0.18)
  end
  
  container:draw(0, 0)
end


local function mousePressed(button, mx, my)

  if isMapArea(mx, my) then  
    if mode == "move" then
      editorUi.selectedMob = editorUi.map.selectObject(editorUi.activeLayer, mx, my, 50)
    
    elseif mode == "delete" then
      local mob = editorUi.map.selectObject(editorUi.activeLayer, mx, my, 50)
      if mob then
        editorUi.selectedMob = mob
        editorUi.map.clientSocket.send("DELM,"..editorUi.selectedMob.id..","..editorUi.activeLayer)
      end
    else
      editorUi.map.clientSocket.send("ADDM,"
                            ..editorUi.activeLayer..","
                            ..editorUi.tile..","
							.."1,"   -- frames : todo handle animated mobs
							.."1,"   -- phases
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


local function keyReleased(key, scancode, isrepeat)
end


editorUi.init = init
editorUi.update = update
editorUi.draw = draw
editorUi.mousePressed = mousePressed
editorUi.mouseReleased = mouseReleased
editorUi.mouseDragged = mouseDragged
editorUi.keyReleased = keyReleased


return editorUi
