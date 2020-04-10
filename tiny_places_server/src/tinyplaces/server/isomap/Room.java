package tinyplaces.server.isomap;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.Writer;
import java.util.ArrayList;
import java.util.Collection;
import java.util.HashMap;
import java.util.List;
import java.util.Set;
import java.util.logging.Level;
import java.util.logging.Logger;
import tinyplaces.server.CommandWorker;
import tinyplaces.server.Server;
import tinyplaces.server.ServerDataEvent;
import tinyplaces.server.isomap.actions.MapAction;


/**
 * A room (map segment)
 * @author Hj. Malthaner
 */
public class Room 
{
    private static final HashMap<String, Room> rooms = new HashMap<String, Room>(64);

    private int nextObjectId = 1;
    
    private final HashMap <Integer, Mob> patches = new HashMap<Integer, Mob>();
    private final HashMap <Integer, Mob> mobs = new HashMap<Integer, Mob>();
    private final HashMap <Integer, Mob> clouds = new HashMap<Integer, Mob>();
    
    private final ArrayList<MapAction> actions = new ArrayList<MapAction>(256);
    private final ArrayList<CreatureGroup> groups = new ArrayList<CreatureGroup>(32);

    private CommandWorker commandWorker;
    private Server server;
    
    public final String name;
    public final String backdrop;
    
    
    public static HashMap<String, Room> getRooms()
    {
        return rooms;
    }


    public Room(String name, String backdrop)
    {
        this.name = name;
        this.backdrop = backdrop;
        rooms.put(name, this);
    }

    
    public ArrayList<MapAction> getActions()
    {
        return actions;
    }
    
    
    public void addAction(MapAction move) 
    {
        synchronized(actions)
        {
            actions.add(move);
        }
    }

    
    public HashMap <Integer, Mob> getLayerMap(int layer)
    {
        switch(layer)
        {
            case 1:
                return patches;
            case 3:
                return mobs;
            case 5:
                return clouds;
            default:
                Logger.getLogger(Room.class.getName()).log(Level.SEVERE, "No such layer: {0}", layer);
                return null;
        }
    }
    
    
    public int getNextObjectId()
    {
        return nextObjectId ++;
    }

    public void addMob(int layer, Mob mob)
    {
        HashMap <Integer, Mob> lmap = getLayerMap(layer);
        
        synchronized(lmap)
        {
            lmap.put(mob.id, mob);
        }
    }
    
    public Mob getMob(int layer, int id)
    {
        HashMap <Integer, Mob> lmap = getLayerMap(layer);
        return lmap.get(id);
    }

    
    public Mob removeMob(int layer, int id)
    {
        HashMap <Integer, Mob> lmap = getLayerMap(layer);
        Mob mob;
        
        synchronized(lmap)
        {
            mob = lmap.remove(id);
        }
        return mob;
    }


    public void save(String filename) 
    {
        try 
        {
            File file = new File("maps", filename);
            FileWriter writer = new FileWriter(file);
            
            writer.write("v10\n");
            writer.write(name + "\n");
            writer.write(backdrop + "\n");
            saveLayer(writer, 1);
            saveLayer(writer, 3);
            saveLayer(writer, 5);
            
            writer.close();
        }
        catch (IOException ex) 
        {
            Logger.getLogger(Room.class.getName()).log(Level.SEVERE, null, ex);
        }
    }
    
    
    private void saveLayer(Writer writer, int layer) throws IOException 
    {
        HashMap <Integer, Mob> lmap = getLayerMap(layer);

        Set <Integer> keys = lmap.keySet();

        for(Integer i : keys)
        {
            Mob mob = lmap.get(i);
            if(mob.type == Mob.TYPE_PROP)
            {
                // id will not be saved but set freshly on loading the map
                String line = "" + layer + "," +
                    mob.tile + "," +
                    mob.x + "," +
                    mob.y + "," +
                    mob.scale + "," +
                    mob.color + "\n";

                writer.write(line);
            }
        }
    }

    /*
    public void init(String backdrop) 
    {
        patches.clear();
        mobs.clear();
        clouds.clear();
        
        actions.clear();
        groups.clear();
        
        this.backdrop = backdrop;
    }
*/

    public Mob makeMob(String [] parts)
    {
        int layer = Integer.parseInt(parts[1]);
        int tile = Integer.parseInt(parts[2]);
        int x = Integer.parseInt(parts[3]);
        int y = Integer.parseInt(parts[4]);
        float scale = Float.parseFloat(parts[5]);
        String color = parts[6].trim();

        return makeMob(layer, tile, x, y, scale, color, Mob.TYPE_PROP);
    }
    
    
    public Mob makeMob(int layer, int tile, int x, int y, float scale, String color, int type)
    {
        int id = getNextObjectId();
        
        Mob mob = new Mob();
        mob.id = id;
        mob.tile = tile;
        mob.x = x;
        mob.y = y;
        mob.scale = scale;
        mob.color = color;
        mob.type = type;
        
        addMob(layer, mob);
        
        return mob;
    }

    
    public List <Mob> makeMobGroup(int spacing)
    {
        ArrayList <Mob> result = new ArrayList<Mob>();
        
        for(int i=0; i<7; i++)
        {
            int x = 300 + spacing * 2 * (int)(Math.random() * 5 - 2.5);
            int y = 350 + spacing * (int)(Math.random() * 5 - 2.5);

            // Imps
            // Mob mob = makeMob(3, 1, x, y, 1.0f, "0.8 0.9 1 1", Mob.TYPE_CREATURE);
            
            // Vortices
            Mob mob = makeMob(3, 9, x, y, 1.0f, "1 0.9 0.6 1", Mob.TYPE_CREATURE);
            mob.nextAiTime = System.currentTimeMillis() + (int)(Math.random() * 10000);
            mob.speed = 20;
            
            result.add(mob);
        }
        
        CreatureGroup creatureGroup = new CreatureGroup(result, 300, 350);
        groups.add(creatureGroup);
        
        
        return result;
    }
    
    /*
     * Todo: Move this to a better place someday?
     */
    public void aiCall()
    {
        long time = System.currentTimeMillis();
        
        for(CreatureGroup group : groups)
        {
            for(Mob creature : group.creatures)
            {
                if(creature.nextAiTime < time)
                {
                    // fire at a player?
                    if(Math.random() < 0.75)
                    {
                        ArrayList<Mob> moblist = new ArrayList<Mob>  (mobs.values());
                        // find a player
                        for(Mob mob : moblist)
                        {
                            if(mob.type == Mob.TYPE_PLAYER)
                            {
                                // commandWorker.fireProjectile(this, creature, 3, 1, mob.x, mob.y);
                                commandWorker.fireProjectile(this, creature, 3, 3, mob.x, mob.y);
                            }
                        }
                    }
                    
                    // move
                    int x, y, len;
                    int count = 0;
                            
                    do
                    {
                        x = creature.x + 100 - (int)(Math.random() * 200);
                        y = creature.y + 100 - (int)(Math.random() * 200);
                        
                        int dx = (x - group.cx);
                        int dy = (y - group.cy);
                        
                        len = dx * dx + (dy * dy) * 4;
                        count ++;
                        
                        // System.err.println("len=" + len);
                    } while(len > 100 * 100 && count < 5);

                    if(count >= 5)
                    {
                        x = group.cx + 50 - (int)(Math.random() * 100);
                        y = group.cy + 50 - (int)(Math.random() * 100);
                    }
                    
                    // System.err.println("id=" + creature.id + "moves to " + x + ", " + y);
                    commandWorker.doMove(null, this, creature.id, 3, x, y, creature.speed, "glide");
                    
                    creature.nextAiTime = time + 3000 + (int)(Math.random() * 2000);
                }
            }
        }
    }

    public void setCommandWorker(CommandWorker commandWorker) 
    {
        this.commandWorker = commandWorker;
    }

    public void setServer(Server server) 
    {
        this.server = server;
    }

    public Server getServer() 
    {
        return server;
    }

    /**
     * Transit player to a new room
     */ 
    void transit(ServerDataEvent dataEvent, Mob mob, String roomname) 
    {
        commandWorker.transit(dataEvent, mob, this, roomname);
    }

}
