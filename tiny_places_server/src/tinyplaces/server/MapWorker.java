package tinyplaces.server;

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
 * Worker class for map altering commands. This will run in a thread of
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
        else if(command.startsWith("UPDM"))
        {
            updateMob(dataEvent, command);
        }
    }
    
	
    private void loginClient(ServerDataEvent dataEvent, String command)
    {
        System.err.println("HELO from " + dataEvent.socket);
        
        // log in new client .. no authentification currently
        // clients all arrive in the lobby right now
        clients.put(dataEvent.socket, new Client(Room.LOBBY));
    }
    
    
    private void addMob(ServerDataEvent dataEvent, String command)
    {
        System.err.println("ADDM from " + dataEvent.socket);
        
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        int id = room.getNextObjectId();
        
        String [] parts = command.split(",");
		
        Mob mob = new Mob();
        mob.id = id;
        mob.tile = Integer.parseInt(parts[1]);
        mob.x = Integer.parseInt(parts[2]);
        mob.y = Integer.parseInt(parts[3]);
        mob.scale = Float.parseFloat(parts[4]);
		
        room.addMob(mob);
        
        roomcast(dataEvent.server,
                 "ADDM," + id + "," + parts[1] + "," + parts[2] + "," + parts[3] + "," + parts[4],
                 room);
    }
    
	
    private void updateMob(ServerDataEvent dataEvent, String command)
    {
        System.err.println("UPDM from " + dataEvent.socket);
        
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        
        String [] parts = command.split(",");
        int id = Integer.parseInt(parts[1]);
		
        Mob mob = room.getMob(id);
        
        mob.tile = Integer.parseInt(parts[2]);
        mob.x = Integer.parseInt(parts[3]);
        mob.y = Integer.parseInt(parts[4]);
        mob.scale = Float.parseFloat(parts[5]);
        
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
