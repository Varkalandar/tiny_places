-- 
-- Client socket connection
--
-- Author: Hj. Malthaner
-- Date: 2020/03/10
--

local socket = require("socket")


local clientSocket = {socket = socket }


local function connect(host, port)

  local tcp = assert(socket.tcp())

  local ok, err = tcp:connect(host, port)
  
  if(not ok) then
    print("Connection problem: " .. err)
  end
  
  clientSocket.tcp = tcp
end  
  
  
local function send(message)
  
  --note the newline below
  -- tcp:send("HELO\n");
  clientSocket.tcp:send(message .. "\n");
end


local function receive()
  clientSocket.tcp:settimeout(0)

  -- nonblocking
  -- socket.select(tcp, nil, 0)

  local s, status, partial = clientSocket.tcp:receive()
  
  local message = s or partial
  
  -- print(message)

  if status == "closed" then 
    print("Server connection lost")
  end
  
  return message
end


local function close()
  tcp:close()
end

clientSocket.connect = connect
clientSocket.send = send
clientSocket.receive = receive
clientSocket.close = close

return clientSocket

