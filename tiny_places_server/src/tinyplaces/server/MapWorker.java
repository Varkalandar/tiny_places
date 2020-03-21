package tinyplaces.server;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import tinyplaces.server.isomap.Client;
import java.io.IOException;
import java.nio.channels.SocketChannel;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.logging.Level;
import java.util.logging.Logger;
import tinyplaces.server.isomap.Mob;
import tinyplaces.server.isomap.Room;

/**
 * Worker class for map altering commands. This will be run in a thread of
 * its own.
 * 
 * @author Hj. Malthaner
 */
public class MapWorker implements ServerWorker
{
    // network data queue
    private final List <ServerDataEvent> queue = new ArrayList();

    // client map
    private final Map <SocketChannel, Client> clients = new HashMap();
    
    
    @Override
    public void processData(Server server, SocketChannel socket, byte[] data, int bytes)
    {
        // Make a copy because the buffer can be overwritten any time later
        byte[] dataCopy = new byte[bytes];
        System.arraycopy(data, 0, dataCopy, 0, bytes);
        synchronized(queue)
        {
            queue.add(new ServerDataEvent(server, socket, dataCopy));
            queue.notify();
        }
    }

    @Override
    public void run()
    {
        while(true)
        {
            try
            {
                ServerDataEvent dataEvent;

                // Wait for data to become available
                synchronized (queue)
                {
                    while(queue.isEmpty())
                    {
                        try
                        {
                            queue.wait();
                        }
                        catch (InterruptedException e)
                        {
                            System.err.println("Interrupt during queue wait:" + e);
                        }
                    }
                    dataEvent = (ServerDataEvent) queue.remove(0);
                }

                processCommands(dataEvent);
            }
            catch(Exception ex)
            {
                // report but keep flying
                Logger.getLogger(MapWorker.class.getName()).log(Level.SEVERE, null, ex);                
            }
        }
    }
    
    
    private void processCommands(ServerDataEvent dataEvent)
    {
        String message = new String(dataEvent.data);
        String [] commands = message.split("\n");
        
        for(String command : commands)
        {
            processCommand(dataEvent, command + "\n");
        }
    }
    
    
    private void processCommand(ServerDataEvent dataEvent, String command)
    {
        if(command.startsWith("HELO"))
        {
            loginClient(dataEvent, command);
        }
        else if(command.startsWith("ADDM"))
        {
            addMob(dataEvent, command);
        }
        else if(command.startsWith("ADDP"))
        {
            addPlayer(dataEvent, command);
        }
        else if(command.startsWith("UPDM"))
        {
            updateMob(dataEvent, command);
        }
        else if(command.startsWith("DELM"))
        {
            deleteMob(dataEvent, command);
        }
        else if(command.startsWith("SAVE"))
        {
            saveMap(dataEvent, command);
        }
        else if(command.startsWith("LOAD"))
        {
            loadMap(dataEvent, command);
        }
        else if(command.startsWith("CHAT"))
        {
            sendChat(dataEvent, command);
        }
        else if(command.startsWith("MOVE"))
        {
            doMove(dataEvent, command);
        }
        else
        {
            Logger.getLogger(MapWorker.class.getName()).log(Level.WARNING, "Received unknown command: '{0}'", command);
        }
    }
    
	
    private void loginClient(ServerDataEvent dataEvent, String command)
    {
        System.err.println("HELO from " + dataEvent.socket);
        
        // log in new client .. no authentification currently
        // clients all arrive in the lobby right now
        clients.put(dataEvent.socket, new Client(Room.LOBBY));
    }

    private Mob makeMob(Room room, String [] parts)
    {
        int id = room.getNextObjectId();
        
        Mob mob = new Mob();
        mob.id = id;
        mob.tile = Integer.parseInt(parts[2]);
        mob.x = Integer.parseInt(parts[3]);
        mob.y = Integer.parseInt(parts[4]);
        mob.scale = Float.parseFloat(parts[5]);
        mob.color = parts[6].trim();
        
        int layer = Integer.parseInt(parts[1]);
        room.addMob(layer, mob);
        
        return mob;
    }
    
    private void addMob(ServerDataEvent dataEvent, String command)
    {
        System.err.println("ADDM from " + dataEvent.socket);
        
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        String [] parts = command.split(",");

        Mob mob = makeMob(room, parts);
        
        roomcast(dataEvent.server,
                 "ADDM," + mob.id + "," + parts[1] + "," + parts[2] + "," + parts[3] + "," + parts[4] + "," + parts[5] + "," + parts[6],
                 room);
    }
    
    private void addPlayer(ServerDataEvent dataEvent, String command)
    {
        System.err.println("ADDP from " + dataEvent.socket);
        
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        String [] parts = command.split(",");

        Mob mob = makeMob(room, parts);
        
        roomcast(dataEvent.server,
                 "ADDP," + mob.id + "," + parts[1] + "," + parts[2] + "," + parts[3] + "," + parts[4] + "," + parts[5] + "," + parts[6],
                 room);
	
    }
    
    private void updateMob(ServerDataEvent dataEvent, String command)
    {
        System.err.println("UPDM from " + dataEvent.socket);
        
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        
        String [] parts = command.split(",");
        int id = Integer.parseInt(parts[1]);
        int layer = Integer.parseInt(parts[2]);
		
        Mob mob = room.getMob(layer, id);
        
        mob.tile = Integer.parseInt(parts[3]);
        mob.x = Integer.parseInt(parts[4]);
        mob.y = Integer.parseInt(parts[5]);
        mob.scale = Float.parseFloat(parts[6]);
        mob.color = parts[7].trim();
        
        roomcast(dataEvent.server, command, room);
    }
	

    private void deleteMob(ServerDataEvent dataEvent, String command)
    {
        System.err.println("DELM from " + dataEvent.socket);
        
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        
        String [] parts = command.split(",");
        int id = Integer.parseInt(parts[1].trim());
        int layer = Integer.parseInt(parts[2].trim());
		
        room.deleteMob(layer, id);
        
        roomcast(dataEvent.server, command, room);
    }
	

    private void saveMap(ServerDataEvent dataEvent, String command) 
    {
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        
        room.save();        
    }

    
    private void loadMap(ServerDataEvent dataEvent, String command) 
    {
        System.err.println("LOAD from " + dataEvent.socket);

        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        room.clear();

        File file = new File("maps", "dummy_map.txt");
        
        try 
        {
            BufferedReader reader = new BufferedReader(new FileReader(file));
            
            String line;
            while((line = reader.readLine()) != null)
            {
                System.err.println(line);
                addMob(dataEvent, "ADDM," + line + "\n");
            }
            
            reader.close();
        }
        catch (IOException ex) 
        {
            Logger.getLogger(MapWorker.class.getName()).log(Level.SEVERE, null, ex);
        }
    }

    
    private void sendChat(ServerDataEvent dataEvent, String command)
    {
        System.err.println("CHAT from " + dataEvent.socket);

        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        roomcast(dataEvent.server, command, room);
    }


    private void doMove(ServerDataEvent dataEvent, String command) 
    {
        System.err.println("MOVE from " + dataEvent.socket);

        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        roomcast(dataEvent.server, command, room);
    }

    /**
     * Send a message to all clients in the given room
     * @param server
     * @param message 
     */
    private void roomcast(Server server, String message, Room room)
    {
        byte [] data = message.getBytes();
        Set <SocketChannel> keys = clients.keySet();
        
        for(SocketChannel socket : keys)
        {
            Client client = clients.get(socket);
            if(client.getCurrentRoom() == room)
            {
                server.send(socket, data);
            }
        }
    }

    
    /**
     * Send a message to all clients
     * @param server
     * @param message 
     */
    private void broadcast(Server server, String message)
    {
        byte [] data = message.getBytes();
        Set <SocketChannel> keys = clients.keySet();
        
        for(SocketChannel socket : keys)
        {
            server.send(socket, data);
        }
    }

}
