package tinyplaces.server.isomap;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
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
    
    private HashMap <Integer, Mob> mobs = new HashMap<Integer, Mob>();
    
    public int getNextObjectId()
    {
        return nextObjectId ++;
    }

    public void addMob(Mob mob)
    {
        mobs.put(mob.id, mob);
    }
    
    public Mob getMob(int id)
    {
        return mobs.get(id);
    }

    public void removeMob(int id)
    {
        mobs.remove(id);
    }

    public void save() 
    {
        try 
        {
            File file = new File("maps", "dummy_map.txt");
            FileWriter writer = new FileWriter(file);
            
            Set <Integer> keys = mobs.keySet();
            
            for(Integer i : keys)
            {
                Mob mob = mobs.get(i);
 
                // id will not be saved but set freshly on loading the map
                String line = "" +
                    mob.tile + "," +
                    mob.x + "," +
                    mob.y + "," +
                    mob.scale + "\n";
                
                writer.write(line);
            }
            
            writer.close();
        }
        catch (IOException ex) 
        {
            Logger.getLogger(Room.class.getName()).log(Level.SEVERE, null, ex);
        }
    }

    public void clear() 
    {
        mobs.clear();
    }
    
}
