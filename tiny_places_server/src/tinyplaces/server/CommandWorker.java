package tinyplaces.server;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.nio.channels.SocketChannel;
import java.util.ArrayList;
import java.util.Collection;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.logging.Level;
import java.util.logging.Logger;
import tinyplaces.server.data.AnimationType;
import tinyplaces.server.data.Item;
import tinyplaces.server.data.ItemBuilder;
import tinyplaces.server.data.Spell;
import tinyplaces.server.data.SpellCatalog;
import tinyplaces.server.isomap.Client;
import tinyplaces.server.isomap.Mob;
import tinyplaces.server.isomap.Room;
import tinyplaces.server.isomap.actions.Action;
import tinyplaces.server.isomap.actions.Move;
import tinyplaces.server.isomap.actions.SpellCast;

/**
 * Worker class for map altering commands. This will be run in a thread of
 * its own.
 * 
 * @author Hj. Malthaner
 */
public class CommandWorker implements ServerWorker
{
    // network data queue
    private final List <ServerDataEvent> queue = new ArrayList();

    // client map
    private final Map <SocketChannel, Client> clients = new HashMap();
    
    private final ChatCommandWorker chatCommandWorker = new ChatCommandWorker();
    
    /**
     * Process data sent by a client
     * @param server The server that received the data
     * @param socket The socket that is connected to the client
     * @param data The actual data
     * @param bytes The amount of bytes (the data array might have more entries, but only this much were sent by the client)
     */
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
                        catch (InterruptedException iex)
                        {
                            System.err.println("CommandWorker: Interrupt during queue wait:" + iex);
                        }
                    }
                    dataEvent = (ServerDataEvent) queue.remove(0);
                }

                processCommands(dataEvent);
            }
            catch(Exception ex)
            {
                // report but keep flying
                Logger.getLogger(CommandWorker.class.getName()).log(Level.SEVERE, null, ex);                
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
        else if(command.startsWith("GAME"))
        {
            startGame(dataEvent, command);
        }
        else if(command.startsWith("UPDI"))
        {
            updateItem(dataEvent, command);
        }
        else if(command.startsWith("UPDM"))
        {
            updateMob(dataEvent, command);
        }
        else if(command.startsWith("DELM"))
        {
            deleteMob(dataEvent, command);
        }
        else if(command.startsWith("FIRE"))
        {
            fireProjectile(dataEvent, command);
        }
        /*
        else if(command.startsWith("GAME"))
        {
            startGame(dataEvent, command);
        }
        */
        else if(command.startsWith("GBYE"))
        {
            logoutClient(dataEvent, command);
        }
        else if(command.startsWith("SAVE"))
        {
            saveMap(dataEvent, command);
        }
        else if(command.startsWith("LOAD"))
        {
            loadMap(dataEvent.server, clients.get(dataEvent.socket), command);
        }
        else if(command.startsWith("CHAT"))
        {
            handleChat(dataEvent, command);
        }
        else if(command.startsWith("MOVE"))
        {
            doMove(dataEvent, command);
        }
        else if(command.startsWith("REGI"))
        {
            registerAccount(dataEvent, command);
        }
        else
        {
            Logger.getLogger(CommandWorker.class.getName()).log(Level.WARNING, "Received unknown command: '{0}'", command);
        }
    }
    
	
    private void loginClient(ServerDataEvent dataEvent, String command)
    {
        System.err.println("HELO from " + dataEvent.socket);
        String [] parts = command.trim().split(",");
        
        String name = parts[1];
        
        File folder = new File("players", name.toLowerCase());
        boolean success = false;
        String message = "";
        if(folder.exists()) 
        {
            try
            {
                File character = new File(folder, name.toLowerCase() + ".ini");
                FileReader fr = new FileReader(character);
                BufferedReader reader = new BufferedReader(fr);
                
                String line;
                
                line = reader.readLine(); // version
                line = reader.readLine(); // name
                line = reader.readLine(); // password
                
                if(line.equals(parts[2]))
                {
                    success = true;                    
                }
                else
                {
                    message = "Login failed. Please try again.";
                }
                reader.close();
            } 
            catch(IOException ex)
            {
                message = "Account creation failed:\n" + ex.getMessage();
            }
        }
        if(success)
        {
            message = "CHAT,System,successful\n";
            singlecast(dataEvent.server, dataEvent.socket, message);
            Client client = new Client(parts[1], dataEvent.socket);
            clients.put(dataEvent.socket, client);
            sendFullPlayerStats(dataEvent.server, client);
        }
        else
        {
            message = "CHAT,System," + message+"\n";
            singlecast(dataEvent.server, dataEvent.socket, message);
        }
    }

    
    private void logoutClient(ServerDataEvent dataEvent, String command)
    {
        System.err.println("GBYE from " + dataEvent.socket);
        
        Client client = clients.get(dataEvent.socket);
        if(client != null)
        {
            Room room = client.getCurrentRoom();
            Object test;

            // if the client didn't start a game, there is no mob assigned
            if(client.mob != null)
            {
                test = room.removeMob(3, client.mob.id);
                if(test == null)
                {
                    Logger.getLogger(CommandWorker.class.getName()).log(Level.WARNING, 
                            "Logout problem: client avatar was not in room.");
                }
            }
        }
        
        Client test = clients.remove(dataEvent.socket);
        if(test == null)
        {
            Logger.getLogger(CommandWorker.class.getName()).log(Level.WARNING, 
                    "Logout problem: client was not in list.");
        }
        
        Logger.getLogger(CommandWorker.class.getName()).log(Level.INFO, 
                "Remaining clients: {0}", clients.size());
    }

    
    public void sendPlayerStat(Server server, Client client, int statIndex)
    {
        Client.Stat stat = client.stats[statIndex];

        if(stat != null)
        {
            StringBuilder message = new StringBuilder();

            message.append("STAT,");
            message.append(statIndex);
            message.append(",");
            message.append(stat.min);
            message.append(",");
            message.append(stat.max);
            message.append(",");
            message.append(stat.value);
            message.append('\n');
        
            // System.err.println(message.toString());
            singlecast(server, client.socket, message.toString());
        }
    }
    
    
    private void sendFullPlayerStats(Server server, Client client)
    {
        StringBuilder message = new StringBuilder();

        message.append("STAT,");
        
        for(int i=0; i<client.stats.length; i++)
        {
            Client.Stat stat = client.stats[i];
            if(stat != null)
            {
                message.append(i);
                message.append(",");
                message.append(stat.min);
                message.append(",");
                message.append(stat.max);
                message.append(",");
                message.append(stat.value);
                message.append(",");
            }
        }
        message.deleteCharAt(message.length()-1);
        message.append('\n');
        
        System.err.println(message.toString());
        singlecast(server, client.socket, message.toString());
    }
    
    
    private void addMob(ServerDataEvent dataEvent, String command)
    {
        System.err.println("ADDM from " + dataEvent.socket);
        
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        String [] parts = command.trim().split(",");

        int layer = Integer.parseInt(parts[1]);
        Mob mob = room.makeMob(parts);
        String cmd = makeAddMobCommand(mob, layer, "n");
        
        roomcast(dataEvent.server, cmd, room);
    }

    
    private void startGame(ServerDataEvent dataEvent, String command)
    {
        System.err.println("GAME from " + dataEvent.socket);
        
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        
        Mob mob = room.makeMob(3, 39, 16, 1, 600, 400, 0.5f, "1.0 1.0 1.0 1.0", Mob.TYPE_PLAYER);
        
        // set new player avatar
        client.mob = mob;
        
        // reply with ADDP to sender only

        String message = "ADDP," + mob.id + "," + client.displayName + "," +
                         "3" + "," + mob.tile + "," + mob.frames + "," + mob.phases + "," +
                         mob.x + "," + mob.y + "," + mob.scale + "," + mob.color + "\n";
        byte [] data = message.getBytes();
        
        SocketChannel senderSocket = dataEvent.socket;
        Server server = dataEvent.server;
        server.send(senderSocket, data);

        // for everyone else in the room it is an ADDM

        message = makeAddMobCommand(mob, 3, client.displayName);
        data = message.getBytes();

        Set <SocketChannel> keys = clients.keySet();
        
        for(SocketChannel socket : keys)
        {
            if(socket != senderSocket)
            {
                Client c = clients.get(socket);
                if(c.getCurrentRoom() == room)
                {
                    server.send(socket, data);
                }
            }
        }
        
        // give the player their items.
        equipPlayer(client);
    }
    
    
    private void equipPlayer(Client client)
    {
        // todo - player database
        
        // for the moment, just hand some default items to the client
        // so there is something for testing item related code
        
        Item item1 = ItemBuilder.create("small_blaster");
        item1.where = Item.IN_FIRST_SLOT + 1;
        
        addItem(client, item1);
        
        Item item2 = ItemBuilder.create("blaster");
        item2.where = Item.IN_INVENTORY;
        if(client.findSuitableLocation(item2) != null)
        {
            addItem(client, item2);
        }    
            
        Item item3 = ItemBuilder.create("firebolt_core");
        item3.where = Item.IN_INVENTORY;
        if(client.findSuitableLocation(item3) != null)
        {
            addItem(client, item3);
        }    
        
        Item item4 = ItemBuilder.create("frostbolt_core");
        item4.where = Item.IN_INVENTORY;
        if(client.findSuitableLocation(item4) != null)
        {
            addItem(client, item4);
        }    
    }
    
    
    private void updateItem(ServerDataEvent dataEvent, String command)
    {
        System.err.println(command);
        
        Client client = clients.get(dataEvent.socket);
        String [] parts = command.trim().split(",");

        int id = Integer.parseInt(parts[1]);
        int where = Integer.parseInt(parts[2]);
        int x = Integer.parseInt(parts[3]);
        int y = Integer.parseInt(parts[4]);
        
        client.updateItem(id, where, x, y);
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

        if(mob != null)
        {
            mob.tile = Integer.parseInt(parts[3]);
            mob.x = Integer.parseInt(parts[4]);
            mob.y = Integer.parseInt(parts[5]);
            mob.scale = Float.parseFloat(parts[6]);
            mob.color = parts[7].trim();

            roomcast(dataEvent.server, command, room);
        }
        else
        {
            Logger.getLogger(CommandWorker.class.getName()).log(Level.SEVERE, "Could not find mob for id={0}", id);
        }
    }

    
    public void updateMob(Client client, int id, int tile, int x, int y, float scale, String color)
    {
        Room room = client.getCurrentRoom();
        
        int layer = 3;
		
        Mob mob = room.getMob(layer, id);

        if(mob != null)
        {
            mob.tile = tile;
            mob.x = x;
            mob.y = y;
            mob.scale = scale;
            mob.color = color;
            
            String command = 
                    "UPDM," + id + "," +
                    layer + "," +
                    tile + "," +
                    x + "," +
                    y + "," +
                    scale + "," +
                    color + "\n";
            
            roomcast(room.getServer(), command, room);
        }
        else
        {
            Logger.getLogger(CommandWorker.class.getName()).log(Level.SEVERE, "Could not find mob for id={0}", id);
        }
    }

    
    public void deleteMob(ServerDataEvent dataEvent, String command)
    {
        System.err.println("DELM from " + dataEvent.socket);
        
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        
        String [] parts = command.split(",");
        int id = Integer.parseInt(parts[1].trim());
        int layer = Integer.parseInt(parts[2].trim());
		
        room.removeMob(layer, id);
        
        roomcast(dataEvent.server, command, room);
    }
	

    private void saveMap(ServerDataEvent dataEvent, String command) 
    {
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        
        String [] parts = command.split(",");
        String filename = parts[1].trim() + ".txt";
        room.save(filename);        
    }

    
    private void loadMap(Server server, Client client, String command) 
    {
        System.err.println("LOAD from " + client.socket);
        
        String [] parts = command.split(",");
        String filename = parts[1].trim();

        // check if the room is already loaded
        HashMap<String, Room> rooms = Room.getRooms();
        Room room = rooms.get(filename);

        if(room == null)
        {
            // room not loaded yet -> load it
            room = loadRoom(filename);

            room.setCommandWorker(this);
            room.setServer(server);
            
            client.setCurrentRoom(room);
            roomcast(server, "LOAD," + room.name + "," + room.backdrop + "," + filename + "\n", room);
            
            serveRoom(room, client.socket);
            rooms.put(filename, room);
        }
        else
        {
            // room is already loaded -> join it
            singlecast(room.getServer(), client.socket, "LOAD," + room.name + "," + room.backdrop + "," + filename + "\n");
            client.setCurrentRoom(room);
            serveRoom(room, client.socket);
        }
    }

    
    private void serveRoom(Room room, SocketChannel socket)
    {
        for(int layer = 1; layer < 6; layer += 2)
        {
            HashMap <Integer, Mob> map = room.getLayerMap(layer);
            Collection <Mob> mobs = map.values();
            
            for(Mob mob : mobs)
            {
                String command = makeAddMobCommand(mob, layer, "n");

                singlecast(room.getServer(), socket, command);
            }
        }
    }

    
    private Room loadRoom(String filename)
    {
        String mapname = filename + ".txt";        
        File file = new File("maps", mapname);

        Room result = null;
        
        try 
        {
            BufferedReader reader = new BufferedReader(new FileReader(file));

            String version = reader.readLine();
            String roomname = reader.readLine();
            String backdrop = reader.readLine();

            result = new Room(roomname, backdrop);
            
            String line;
            while((line = reader.readLine()) != null)
            {
                System.err.println(line);
                String [] parts = line.split(",");
                
                int layer, tile, frames, phases, x, y;
                float scale;
                String color;
                
                // if("v10".equals(version))
                // {
                    layer = Integer.parseInt(parts[0]);
                    tile = Integer.parseInt(parts[1]);
                    frames = 1;
                    phases = 2;
                    x = Integer.parseInt(parts[2]);
                    y = Integer.parseInt(parts[3]);
                    scale = Float.parseFloat(parts[4]);
                    color = parts[5].trim();
                // }
                
                result.makeMob(layer, tile, frames, phases, x, y, scale, color, Mob.TYPE_PROP);
            }
            
            reader.close();
        }
        catch (IOException ex) 
        {
            Logger.getLogger(CommandWorker.class.getName()).log(Level.SEVERE, null, ex);
        }

        return result;
    }

    
    private void handleChat(ServerDataEvent dataEvent, String command)
    {
        Client client = clients.get(dataEvent.socket);
        System.err.println("CHAT from " + dataEvent.socket);
        String chat = command.substring(5);
        
        if(chat.startsWith("/"))
        {
            chatCommandWorker.processChatCommand(this, client, chat);
        }
        else
        {
            StringBuilder buf = new StringBuilder("CHAT,");
            buf.append(client.displayName);
            buf.append(',');
            buf.append(chat);

            Room room = client.getCurrentRoom();
            roomcast(dataEvent.server, buf.toString(), room);
        }
    }
    
    private void registerAccount(ServerDataEvent dataEvent, String command) 
    {
        String [] parts = command.trim().split(",");
        String name = parts[1];
        
        File folder = new File("players", name.toLowerCase());
        boolean success = false;
        String message = "";
        if(folder.exists()) 
        {
            message = "Account name is taken already.";
        }
        else
        {
            try
            {
                folder.mkdirs();
                File character = new File(folder, name.toLowerCase() + ".ini");
                FileWriter fw = new FileWriter(character);
                fw.write("v10\n");
                fw.write(name + "\n");
                fw.write(parts[2] + "\n");
                fw.write("0,0,40,40\n");
                fw.write("1,0,40,40\n");
                fw.close();
                success = true;
            } 
            catch(IOException ex)
            {
                message = "Account creation failed: " + ex.getMessage();
            }
        }
        if(success)
        {
            message = "CHAT,System,successful\n";
            singlecast(dataEvent.server, dataEvent.socket, message);
        }
        else
        {
            message = "CHAT,System," + message + "\n";
            singlecast(dataEvent.server, dataEvent.socket, message);
        }
    }

    
    private void doMove(ServerDataEvent dataEvent, String command) 
    {
        System.err.println("MOVE from " + dataEvent.socket + "\n  " + command);

        String [] parts = command.trim().split(",");

        int id = Integer.parseInt(parts[1]);
        int layer = Integer.parseInt(parts[2]);
        int dx = Integer.parseInt(parts[3]);
        int dy = Integer.parseInt(parts[4]);
        int speed = 120;
        
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        Mob mob = room.getMob(layer, id);
        
        String pattern = "bounce";
        
        if(mob != null)
        {
            // todo - make some catalog of players with their properties
            if(mob.type == Mob.TYPE_PLAYER)
            {
                if(mob.tile == 9 || mob.tile == 20 || mob.tile == 39)
                {
                    // spectres glide
                    pattern = "glide";
                }
            }

            doMove(dataEvent, room, id, layer, dx, dy, speed, pattern);
        }
    }
    
    
    public void doMove(ServerDataEvent dataEvent,
                       Room room, int id, int layer, int dx, int dy, int speed, String pattern)
    {
        // NPC mobs have no client data
        Client client = null;
        if(dataEvent != null)
        {
            client = clients.get(dataEvent.socket);
        }
        
        Mob mob = room.getMob(layer, id);
        Move move = new Move(client, mob, layer, dx, dy, speed);
        
        // check and cancel former move ...
        List <Action> actions = room.getActions();
        ArrayList<Action> actionsCopy = new ArrayList<Action>(actions);
        
        for(Action action : actionsCopy)
        {
            if(action instanceof Move)
            {
                Move m = (Move)action;
                if(m.getMob().id == mob.id)
                {
                    System.err.println("Removing old move for mob id=" + mob.id);
                    
                    synchronized(actions)
                    {
                        actions.remove(m);
                    }
                }
            }
        }
        
        String command =
                "MOVE," +
                id + "," +
                layer + "," +
                dx + "," +
                dy + "," +
                speed + "," + 
                pattern + "\n";

        room.addAction(move);
        roomcast(room.getServer(), command, room);
    }

    
    public void transit(Client client, Mob mob, Room from, String roomname, int newx, int newy) 
    {
        from.removeMob(3, mob.id);
        
        String command = "LOAD," + roomname + "\n";
        loadMap(from.getServer(), client, command);

        Room room = client.getCurrentRoom();
        room.populateRoom(from.getServer(), roomname);

        // in a new room there are new mob ids. Give the player a matching new id
        mob.id = room.getNextObjectId();
        mob.x = newx;
        mob.y = newy;

        command =
            "ADDP," + 
            mob.id + "," +
            client.displayName + "," +
            "3," + // layer
	    mob.tile + "," + // tile id
            mob.frames + "," +
            mob.phases + "," +
	    newx + "," + // x pos
	    newy + "," + // y pos
	    mob.scale + "," + // scale factor
            mob.color + "\n"; // color string
        
        room.addMob(3, mob);
        
        singlecast(room.getServer(), client.socket, command);
    }    

    
    public void removeMob(int id, Room room, int layer)
    {        
        String command = 
                "DELM," +
                id + "," +
                layer + "\n";
        
        room.removeMob(layer, id);

        roomcast(room.getServer(), command, room);
    }
    

    /**
     * Animation types are abstract, it's upon the client to interpret them
     * @param room The room where the animation will be played
     * @param atype Abstract animation type
     * @param layer Map layer to play the animation
     * @param x X position on map
     * @param y Y position on map
     */
    public void playAnimation(Room room, int atype, int layer, int x, int y)
    {
        String command = 
                "ANIM," +
                atype + "," +
                layer + "," +
                x + "," +
                y + "," +
                "\n";

        roomcast(room.getServer(), command, room);
    }
    
    
    public void kill(Mob target, Room room) 
    {
        int layer = 3;   // are targets always layer 3?
        
        removeMob(target.id, room, layer);
        
        int atype = AnimationType.CREATURE_DEATH;  // standard explosion
        int zoff = 20;
        
        if(target.tile == 17)
        {
            atype = AnimationType.CREATURE_BLACK_DEATH; // black death swirl
            zoff = 0;
        }

        playAnimation(room, atype, layer, target.x, (target.y - zoff));
    }
    
    /*
    private void startGame(ServerDataEvent dataEvent, String command) 
    {
        System.err.println("GAME from " + dataEvent.socket);

        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
    }
    */
    
    private void fireProjectile(ServerDataEvent dataEvent, String command) 
    {
        System.err.println("FIRE from " + dataEvent.socket);

        String [] parts = command.split(",");
        
        int layer = Integer.parseInt(parts[1]);
        String ptype = parts[2];
        int dx = Integer.parseInt(parts[3]);
        int dy = Integer.parseInt(parts[4]);
        
        Client client = clients.get(dataEvent.socket);
        Room room = client.getCurrentRoom();
        
        Spell spell = SpellCatalog.get(ptype);
        
        fireProjectile(room, client.mob, layer, dx, dy, spell);
    }

    
    public void fireProjectile(Room room, Mob shooter, int layer, int dx, int dy, Spell spell)
    {
        int sx = shooter.x;
        int sy = shooter.y;

        Mob projectile = room.makeMob(layer, 1, 16, 1, sx, sy, 1.0f, "1 1 1 1", Mob.TYPE_PROJECTILE);
        
        SpellCast spellCast = new SpellCast(shooter, spell, projectile, layer, dx, dy);
        room.addAction(spellCast);
        
        String command = 
                "FIRE," +
                shooter.id + "," +
                projectile.id + "," +
                layer + "," +
                spell.ptype + "," +
                spell.castTime + "," +
                dx + "," +
                dy + "," +
                spell.speed + "\n";

        roomcast(room.getServer(), command, room);
    }

    
    /**
     * If the operation is obstructed by another item, the obstructing
     * item will be returned.
     * @param client The client to receive the item
     * @param item The item to add to the client
     * @return null or the obstructing item
     */
    public Item addItem(Client client, Item item)
    {      
        Room room = client.getCurrentRoom();
        
        Item obstruction = null;
        
        if(item.where != Item.ON_MAP)
        {
            obstruction = client.addItem(item);
        }
        
        if(obstruction == null)
        {
            String command = 
                    "ADDI," +
                    client.mob.id + "," +   // todo - mob == null case
                    item.baseItem.id + "," +
                    item.id + "," +
                    item.mobId + "," +
                    item.displayName + "," +
                    item.baseItem.iclass + "," +
                    item.baseItem.itype + "," +
                    item.baseItem.baseValue + "," +
                    item.baseItem.tile + "," +
                    item.baseItem.color + "," +
                    item.baseItem.scale + "," +
                    item.baseItem.shadow + "," +
                    item.baseItem.shadowScale + "," +
                    item.where + "," +
                    item.position.x + "," +
                    item.position.y + "," +
                    item.energyDamage + "," +
                    item.physicalDamage + "," +
                    item.baseItem.description + "," +
                    "\n";

            if(item.where == Item.ON_MAP)
            {
                roomcast(room.getServer(), command, room);
            }
            else
            {
                System.err.println(command);
                singlecast(room.getServer(), client.socket, command);
            }
        }
        else
        {
            Logger.getLogger(CommandWorker.class.getName()).log(Level.WARNING, "addItem not possible, location blocked by {0}", obstruction.displayName);
        }
        
        return obstruction;
    }
    
    
    public void dropItem(Room room, Item item)
    {      
        assert(item.where == Item.ON_MAP);
        
        String command = 
                "ADDI," +
                "-" + "," +
                item.baseItem.id + "," +
                item.id + "," +
                item.mobId + "," +
                item.displayName + "," +
                item.baseItem.iclass + "," +
                item.baseItem.itype + "," +
                item.baseItem.baseValue + "," +
                item.baseItem.tile + "," +
                item.baseItem.color + "," +
                item.baseItem.scale + "," +
                item.baseItem.shadow + "," +
                item.baseItem.shadowScale + "," +                
                item.where + "," +
                item.position.x + "," +
                item.position.y + "," +
                item.energyDamage + "," +
                item.physicalDamage + "," +
                item.baseItem.description + "," +
                "\n";

        roomcast(room.getServer(), command, room);
    }

    
    public void singlecast(Server server, SocketChannel socket, String message)
    {
        byte [] data = message.getBytes();
        server.send(socket, data);
    }
    
    
    /**
     * Send a message to all clients in the given room
     * @param server
     * @param message 
     */
    public void roomcast(Server server, String message, Room room)
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

    
    private String makeAddMobCommand(Mob mob, int layer, String name)
    {
        String command =
                "ADDM," +
                mob.id + "," +
                name + "," +
                layer + "," +
                mob.tile + "," +
                mob.frames + "," +
                mob.phases + "," +
                mob.x + "," +
                mob.y + "," +
                mob.scale + "," +
                mob.color + "," +
                mob.type +
                "\n";

        return command;
    }
    
    
    public void addMobGroup(Server server, Room room, Collection <Mob> mobs, int layer) 
    {
        for(Mob mob : mobs)
        {
            String command = makeAddMobCommand(mob, layer, "n");
        
            roomcast(server, command, room);
        }
    }

}
