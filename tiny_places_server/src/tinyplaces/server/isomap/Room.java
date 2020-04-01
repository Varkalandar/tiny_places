package tinyplaces.server.isomap;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.Writer;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Set;
import java.util.logging.Level;
import java.util.logging.Logger;
import tinyplaces.server.isomap.actions.MapAction;


/**
 * A room (map segment)
 * @author Hj. Malthaner
 */
public class Room 
{
    private static final ArrayList<Room> rooms = new ArrayList<Room> (256);
    public static final Room LOBBY = new Room();

    private int nextObjectId = 1;
    
    private String backdrop;
    private final HashMap <Integer, Mob> patches = new HashMap<Integer, Mob>();
    private final HashMap <Integer, Mob> mobs = new HashMap<Integer, Mob>();
    private final HashMap <Integer, Mob> clouds = new HashMap<Integer, Mob>();
    
    private final ArrayList<MapAction> actions = new ArrayList<MapAction>(256);


    public static ArrayList<Room> rooms()
    {
        return rooms;
    }


    public Room()
    {
        rooms.add(this);
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

    
    private HashMap <Integer, Mob> getLayerMap(int layer)
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
                Logger.getLogger(Room.class.getName()).log(Level.SEVERE, "No such layer: " + layer);
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

    public Mob deleteMob(int layer, int id)
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
            
            writer.write(backdrop + "\n");
            save(writer, 1);
            save(writer, 3);
            save(writer, 5);
            
            writer.close();
        }
        catch (IOException ex) 
        {
            Logger.getLogger(Room.class.getName()).log(Level.SEVERE, null, ex);
        }
    }
    
    
    private void save(Writer writer, int layer) throws IOException 
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

    
    public void init(String backdrop) 
    {
        patches.clear();
        mobs.clear();
        clouds.clear();
        this.backdrop = backdrop;
    }


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

    
    public List <Mob> makeMobGroup()
    {
        ArrayList <Mob> result = new ArrayList<Mob>();
        
        for(int i=0; i<7; i++)
        {
            int x = 200 + (int)(Math.random() * 100);
            int y = 300 + (int)(Math.random() * 100);

            Mob mob = makeMob(3, 1, x, y, 1.0f, "0.8 0.9 1 1", Mob.TYPE_CREATURE);
            result.add(mob);
        }
        
        return result;
    }
}
