package tinyplaces.server.isomap;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.Writer;
import java.util.HashMap;
import java.util.Set;
import java.util.logging.Level;
import java.util.logging.Logger;

/**
 * A room (map segment)
 * @author Hj. Malthaner
 */
public class Room 
{
    public static final Room LOBBY = new Room();
    private int nextObjectId = 1;
    
    private final HashMap <Integer, Mob> patches = new HashMap<Integer, Mob>();
    private final HashMap <Integer, Mob> mobs = new HashMap<Integer, Mob>();
    private final HashMap <Integer, Mob> clouds = new HashMap<Integer, Mob>();
    
    
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
        lmap.put(mob.id, mob);
    }
    
    public Mob getMob(int layer, int id)
    {
        HashMap <Integer, Mob> lmap = getLayerMap(layer);
        return lmap.get(id);
    }

    public Mob deleteMob(int layer, int id)
    {
        HashMap <Integer, Mob> lmap = getLayerMap(layer);
        return lmap.remove(id);
    }


    public void save() 
    {
        try 
        {
            File file = new File("maps", "dummy_map.txt");
            FileWriter writer = new FileWriter(file);
            
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

            // id will not be saved but set freshly on loading the map
            String line = "" + layer + "," +
                mob.tile + "," +
                mob.x + "," +
                mob.y + "," +
                mob.scale + "\n";

            writer.write(line);
        }
    }

    
    public void clear() 
    {
        patches.clear();
        mobs.clear();
        clouds.clear();
    }
}
